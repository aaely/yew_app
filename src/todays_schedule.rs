use crate::{models::*, state::*};
use chrono::prelude::*;
use serde_json::json;
use wasm_bindgen_futures::spawn_local;
use web_sys::{js_sys, window};
use yew::prelude::*;
use reqwest::Client;
use gloo::console::log;
use crate::daily_csv::DailyCsv;
use std::fmt::Write;

use crate::AppStateContext;

fn render_locations(locations: &Vec<String>) -> String {
    let mut txt = String::new();
    for location in locations {
        match location.as_str() {
            "18008" => txt.push_str(" AR"),
            "18044" => txt.push_str(" FF"),
            "22010" => txt.push_str(" 40"),
            _ => {}
        }
    }
    txt.trim().to_string()
}

fn create_csv(data: &Vec<TrailerResponse>) -> String {
    let mut csv_string = String::new();
    let _ = writeln!(csv_string, "Container ID, Request Date, SCAC Code, Plant Code, Schedule Date, Schedule Time, Arrival Time, Door Number, Contact Email");
    for trailer in data {
        let _ = writeln!(csv_string, "{},{},{},{},{},{},{},{},{}", trailer.TrailerID, trailer.Schedule.RequestDate, trailer.Schedule.CarrierCode ,render_locations(&trailer.CiscoIDs), trailer.Schedule.ScheduleDate, trailer.Schedule.ScheduleTime, trailer.Schedule.ArrivalTime, trailer.Schedule.DoorNumber, trailer.Schedule.ContactEmail);
    }
    csv_string
}

fn parse_time(time_str: &str) -> (u32, u32) {
    let time_str = if time_str.is_empty() { "00:00" } else { time_str };
    
    // Split the string by ':'
    let parts: Vec<&str> = time_str.split(':').collect();
    
    // Initialize hours and minutes to 0
    let mut hours = 0;
    let mut minutes = 0;

    // Parse hours if available
    if let Some(hour_str) = parts.get(0) {
        hours = hour_str.parse::<u32>().unwrap_or(0);
    }

    // Parse minutes if available
    if let Some(minute_str) = parts.get(1) {
        minutes = minute_str.parse::<u32>().unwrap_or(0);
    }

    (hours, minutes)
}

fn time() -> String {
    let now = Local::now();
    format!("{:02}:{:02}:{:02}", now.hour(), now.minute(), now.second())
}

fn format_current_date() -> String {
    let local: DateTime<Local> = Local::now();
    let year = local.year();
    let month = format!("{:02}", local.month());
    let day = format!("{:02}", local.day());
    format!("{}-{}-{}", year, month, day)
}

#[function_component(TodaysSchedule)]
pub fn todays_schedule() -> Html {
    let app_state = use_context::<AppStateContext>().expect("no state found");

    {
        let app_state = app_state.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                let client = Client::new();
                let date = format_current_date();
                let request = TodaysTrucksRequest {
                    date: date.clone()
                };
                if let Some(user) = &app_state.user {
                    match client.post("http://192.168.4.122:8000/api/todays_trucks")
                        .json(&request)
                        .header("Authorization", format!("Bearer {}", user.token))
                        .send()
                        .await {
                            Ok(resp) => {
                                match resp.json::<Vec<TrailerResponse>>().await {
                                    Ok(todays_trailers) => {
                                        let mut trailers = todays_trailers.clone();
                                        trailers.sort_by(|a, b| {
                                            let (hours_a, minutes_a) = parse_time(&a.Schedule.ScheduleTime);
                                            let (hours_b, minutes_b) = parse_time(&b.Schedule.ScheduleTime);
                                            if hours_a == hours_b {
                                                minutes_a.cmp(&minutes_b)
                                            } else {
                                                hours_a.cmp(&hours_b)
                                            }
                                        });
                                        //log!(format!("{:?}", trailers));
                                        app_state.dispatch(AppStateAction::SetTrailers(trailers));
                                    },
                                    Err(error) => {
                                        log!(format!("{:?}", error));
                                        app_state.dispatch(AppStateAction::ClearUser);
                                    },
                                }
                            },
                            Err(error) => log!(format!("Error: {:?}", error))
                        }
                }
            });
            || ()
        })
    }

    let download_csv = {
        let data = app_state.trailers.clone();
        Callback::from(move |_: MouseEvent| {
            let csv_string = create_csv(&data);
            let filename = "daily.csv";
            let window = window().unwrap();
            let document = window.document().unwrap();
            let element = document.create_element("a").unwrap();
            element.set_attribute("href", &format!("data:text/csv;charset=utf-8,{}", js_sys::encode_uri_component(&csv_string))).unwrap();
            element.set_attribute("download", filename).unwrap();
            let body = document.body().unwrap();
            body.append_child(&element).unwrap();
            let event = document.create_event("MouseEvent").unwrap();
            event.init_event("click");
            element.dispatch_event(&event).unwrap();
            body.remove_child(&element).unwrap();
        })
    };

    let un_arrived = {
        let app_state = app_state.clone();
        Callback::from(move |trailer_id: String| {
            let app_state = app_state.clone();
            spawn_local(async move {
                let client = Client::new();
                if let Some(user) = &app_state.user {
                    let request = SetArrivalTimeRequest {
                        TrailerID: trailer_id.clone(),
                        ArrivalTime: "".to_string(),
                    };
                    match client.post("http://192.168.4.122:8000/api/set_arrivalTime")
                        .header("Authorization", format!("Bearer {}", user.token))
                        .json(&request)
                        .send()
                        .await {
                            Ok(resp) => {
                                match resp.json::<Vec<TrailerSchedule>>().await {
                                    Ok(_trailer_response) => {
                                        let msg = ArrivalMessage {
                                            TrailerID: trailer_id,
                                            ArrivalTime: "".to_string(),
                                        };
                                        let json_string = serde_json::to_string(&msg).unwrap();
                                        let message = json!({
                                            "type": "trailer_arrived",
                                            "data": {
                                                "message": json_string
                                            }
                                        }).to_string();
                                        app_state.send_ws_message(&message);
                                    },
                                    Err(error) => {
                                        log!(format!("{:?}", error));
                                        app_state.dispatch(AppStateAction::ClearUser);
                                    },
                                }
                            },
                            Err(error) => log!(format!("{:?}", error))
                        }
                }
            })
        })
    };

    let arrived = {
        let app_state = app_state.clone();
        Callback::from(move |trailer_id: String| {
            let app_state = app_state.clone();
            spawn_local(async move {
                let client = Client::new();
                let now = time();
                if let Some(user) = &app_state.user {
                    let request = SetArrivalTimeRequest {
                        TrailerID: trailer_id.clone(),
                        ArrivalTime: now.clone(),
                    };
                    match client.post("http://192.168.4.122:8000/api/set_arrivalTime")
                        .header("Authorization", format!("Bearer {}", user.token))
                        .json(&request)
                        .send()
                        .await {
                            Ok(resp) => {
                                match resp.json::<Vec<TrailerSchedule>>().await {
                                    Ok(_trailer_response) => {
                                        let msg = ArrivalMessage {
                                            TrailerID: trailer_id,
                                            ArrivalTime: now.clone(),
                                        };
                                        let json_string = serde_json::to_string(&msg).unwrap();
                                        let message = json!({
                                            "type": "trailer_arrived",
                                            "data": {
                                                "message": json_string
                                            }
                                        }).to_string();
                                        app_state.send_ws_message(&message);
                                    },
                                    Err(error) => {
                                        log!(format!("{:?}", error));
                                        app_state.dispatch(AppStateAction::ClearUser);
                                    },
                                }
                            },
                            Err(error) => log!(format!("{:?}", error))
                        }
                }
            })
        })
    };

    let toggle_hot = {
        let app_state = app_state.clone();
        Callback::from(move |trailer_id: String| {
            let app_state = app_state.clone();
            spawn_local(async move {
                let client = Client::new();
                if let Some(user) = &app_state.user {
                    let request = HotTrailerRequest {
                        TrailerID: trailer_id.clone(),
                    };

                    match client.post("http://192.168.4.122:8000/api/hot_trailer")
                        .header("Authorization", format!("Bearer {}", user.token))
                        .json(&request)
                        .send()
                        .await {
                            Ok(resp) => {
                                match resp.json::<Vec<TrailerSchedule>>().await {
                                    Ok(trailer_response) => {
                                        log!(format!("{:?}", trailer_response));
                                        let message = json!({
                                            "type": "hot_trailer",
                                            "data": {
                                                "message": trailer_id.clone()
                                            }
                                        }).to_string();
                                        app_state.send_ws_message(&message);
                                    },
                                    Err(error) => {
                                        log!(format!("{:?}", error));
                                        app_state.dispatch(AppStateAction::ClearUser);
                                    },
                                }
                            },
                            Err(error) => log!(format!("{:?}", error))
                        }
                }
            });
        })
    };

    let load = {
        let app_state = app_state.clone();
        Callback::from(move |trailer: TrailerResponse| {
            app_state.dispatch(AppStateAction::SetCurrentTrailer(trailer));
            app_state.dispatch(AppStateAction::SetCurrentView("load_details".to_string()));
        })
    };

    let edit = {
        let app_state = app_state.clone();
        Callback::from(move |trailer: TrailerResponse| {
            app_state.dispatch(AppStateAction::SetCurrentTrailer(trailer));
            app_state.dispatch(AppStateAction::SetCurrentView("edit_trailer".to_string()));
        })
    };


    html! {
        <div style="margin-top: 7vh; width: 90vw;">
            <h1 style="text-align: center;">{"Today's Schedule"}</h1>
            <div style="
            text-align: center;
            width: 30vw;
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: space-evenly;
            margin-left: auto;
            margin-right: auto;
            margin-bottom: 3%">
                <a onclick={download_csv}>{"Download Todays Schedule"}</a>
            </div>
            <table>
                <thead>
                    <tr style="text-align: center;">
                        <th>{"Request Date"}</th>
                        <th>{"Trailer ID"}</th>
                        <th>{"SCAC"}</th>
                        <th>{"Plant"}</th>
                        <th>{"Last Free Day"}</th>
                        <th>{"Scheduled Date"}</th>
                        <th>{"Scheduled Time"}</th>
                        <th>{"Arrival Time"}</th>
                        <th>{"Door"}</th>
                        <th>{"Hot?"}</th>
                    </tr>
                </thead>
                <tbody>
                { app_state.trailers.iter().enumerate().map(|(index, trailer)| {
                    if trailer.Schedule.IsHot {
                        let trailer_id = trailer.TrailerID.clone();
                        let trailer_id1 = trailer.TrailerID.clone();
                        let trailer_id2 = trailer.TrailerID.clone();
                        let tr = trailer.clone();
                        let tr1 = trailer.clone();
                        let user = app_state.user.as_ref().unwrap();
                        html! { 
                            <tr style="background-color: red; text-align: center;">
                                <td>{index + 1}</td>
                                <td>{trailer.Schedule.RequestDate.clone()}</td>
                                <td><a onclick={load.clone().reform(move |_| tr.clone())}>{trailer.TrailerID.clone()}</a></td>
                                <td>{trailer.Schedule.CarrierCode.clone()}</td>
                                <td>{render_locations(&trailer.CiscoIDs)}</td>
                                <td>{trailer.Schedule.LastFreeDate.clone()}</td>
                                <td>{trailer.Schedule.ScheduleDate.clone()}</td>
                                <td>{trailer.Schedule.ScheduleTime.clone()}</td>
                                { if trailer.Schedule.ScheduleDate.len() > 0 && trailer.Schedule.ArrivalTime.len() < 1 && user.role.clone() == "write".to_string() {
                                    html! { <td><button onclick={arrived.clone().reform(move |_| trailer_id1.clone())}>{"Arrived"}</button></td> }
                                } else if trailer.Schedule.ArrivalTime.len() > 0 && user.role.clone() == "write".to_string() {
                                    html! { <td><a onclick={un_arrived.clone().reform(move |_| trailer_id2.clone())}>{trailer.Schedule.ArrivalTime.clone()}</a></td>}
                                } else {
                                    html! { <td>{trailer.Schedule.ArrivalTime.clone()}</td> }
                                }}
                                <td>{trailer.Schedule.DoorNumber.clone()}</td>
                                <td><button style="background-color: #4CAF50; color: white; padding: 14px 20px; border: none; cursor: pointer; border-radius: 4px;" onclick={toggle_hot.clone().reform(move |_| trailer_id.clone())}>{"Mark Not Hot"}</button></td>
                                { if user.role.clone() == "write".to_string() {
                                    html! {<td><button style="background-color: blue; color: white; padding: 14px 20px; border: none; cursor: pointer; border-radius: 4px;" onclick={edit.clone().reform(move |_| tr1.clone())}>{"Edit"}</button></td>}
                                } else {
                                    html! {<></>}
                                }}
                            </tr>
                        }
                    } else {
                        let trailer_id = trailer.TrailerID.clone();
                        let trailer_id1 = trailer.TrailerID.clone();
                        let trailer_id2 = trailer.TrailerID.clone();
                        let tr = trailer.clone();
                        let tr1 = trailer.clone();
                        let user = app_state.user.as_ref().unwrap();
                        html! {
                            <tr style="text-align: center;">
                                <td>{index + 1}</td>
                                <td>{trailer.Schedule.RequestDate.clone()}</td>
                                <td><a onclick={load.clone().reform(move |_| tr.clone())}>{trailer.TrailerID.clone()}</a></td>
                                <td>{trailer.Schedule.CarrierCode.clone()}</td>
                                <td>{render_locations(&trailer.CiscoIDs)}</td>
                                <td>{trailer.Schedule.LastFreeDate.clone()}</td>
                                <td>{trailer.Schedule.ScheduleDate.clone()}</td>
                                <td>{trailer.Schedule.ScheduleTime.clone()}</td>
                                { if trailer.Schedule.ScheduleDate.len() > 0 && trailer.Schedule.ArrivalTime.len() < 1 && user.role.clone() == "write".to_string() {
                                    html! { <td><button onclick={arrived.clone().reform(move |_| trailer_id1.clone())}>{"Arrived"}</button></td> }
                                } else if trailer.Schedule.ArrivalTime.len() > 0 && user.role.clone() == "write".to_string() {
                                    html! { <td><a onclick={un_arrived.clone().reform(move |_| trailer_id2.clone())}>{trailer.Schedule.ArrivalTime.clone()}</a></td>}
                                } else {
                                    html! { <td>{trailer.Schedule.ArrivalTime.clone()}</td> }
                                }}
                                <td>{trailer.Schedule.DoorNumber.clone()}</td>
                                <td><button style="background-color: #F44336; color: white; padding: 14px 20px; border: none; cursor: pointer; border-radius: 4px;" onclick={toggle_hot.clone().reform(move |_| trailer_id.clone())}>{"Mark Hot"}</button></td>
                                { if user.role.clone() == "write".to_string() {
                                    html! {<td><button style="background-color: blue; color: white; padding: 14px 20px; border: none; cursor: pointer; border-radius: 4px;" onclick={edit.clone().reform(move |_| tr1.clone())}>{"Edit"}</button></td>}
                                } else {
                                    html! {<></>}
                                }}
                            </tr>
                        }
                    }
                }).collect::<Html>() }
                </tbody>
            </table>
            <DailyCsv />
        </div>
    }
}