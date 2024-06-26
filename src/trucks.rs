use std::rc::Rc;

use crate::{requests::*, state::*};
use serde_json::json;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use reqwest::Client;
use gloo::console::log;

use crate::{AppState, AppStateContext};

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

#[function_component(Trucks)]
pub fn trucks() -> Html {
    let app_state = use_context::<AppStateContext>().expect("no state found");

    if let Some(user) = &app_state.user {
        html! {
            <>
                <RenderTrucks />
            </>
        }
    } else {
        html! {
            <>
                { "No token availabe" }
            </>
        }
    }
}

#[function_component(RenderTrucks)]
fn render_trucks() -> Html {
    let app_state = use_context::<AppStateContext>().expect("no app state found");
    let trailers = use_state(|| Rc::new(Vec::<TrailerResponse>::new()));
    
    let toggle_hot = {
        let trailers = trailers.clone();
        let app_state = app_state.clone();
        Callback::from(move |trailer_id: String| {
            let trailers = trailers.clone();
            let app_state = app_state.clone();
            spawn_local(async move {
                let client = Client::new();
                if let Some(user) = &app_state.user {
                    let request = HotTrailerRequest {
                        TrailerID: trailer_id.clone(),
                    };

                    match client.post("http://192.168.4.97:8000/api/hot_trailer")
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
                                    Err(error) => log!(format!("Error: {:?}", error)),
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

    {
        let app_state = app_state.clone();
        {
            let app_state = app_state.clone();
    
            use_effect_with((), move |_| {
                let app_state = app_state.clone();
    
                spawn_local(async move {
                    let client = Client::new();
                    if let Some(user) = &app_state.user {
                        match client.get("http://192.168.4.97:8000/api/schedule_trailer")
                            .header("Authorization", format!("Bearer {}", user.token))
                            .send()
                            .await {
                                Ok(resp) => {
                                    match resp.json::<Vec<TrailerResponse>>().await {
                                        Ok(trailer_response) => app_state.dispatch(AppStateAction::SetTrailers(trailer_response)),
                                        Err(error) => log!(format!("Error: {:?}", error)),
                                    }
                                },
                                Err(error) => log!(format!("{:?}", error))
                            }
                    }
                });
    
                || ()
            });
        }
    }

    let app_state = app_state.clone();

    html! {
        <div class="container">
            <h1 style="text-align: center;">{ "Testing All Trailers" }</h1>
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
                { for app_state.trailers.iter().map(|trailer| 
                    if trailer.Schedule.IsHot {
                    let trailer_id = trailer.TrailerID.clone();
                    let tr = trailer.clone();
                    html! { 
                        <tr style="background-color: red; text-align: center;">
                            <td>{trailer.Schedule.RequestDate.clone()}</td>
                            <td><a onclick={load.clone().reform(move |_| tr.clone())}>{trailer.TrailerID.clone()}</a></td>
                            <td>{trailer.Schedule.CarrierCode.clone()}</td>
                            <td>{render_locations(&trailer.CiscoIDs)}</td>
                            <td>{trailer.Schedule.LastFreeDate.clone()}</td>
                            <td>{trailer.Schedule.ScheduleDate.clone()}</td>
                            <td>{trailer.Schedule.ScheduleTime.clone()}</td>
                            <td>{trailer.Schedule.ArrivalTime.clone()}</td>
                            <td>{trailer.Schedule.DoorNumber.clone()}</td>
                            <td><button style="background-color: #4CAF50; color: white; padding: 14px 20px; border: none; cursor: pointer; border-radius: 4px;" onclick={toggle_hot.clone().reform(move |_| trailer_id.clone())}>{"Mark Not Hot"}</button></td>
                        </tr>
                }} else {
                    let trailer_id = trailer.TrailerID.clone();
                    let tr = trailer.clone();
                    html! {
                        <tr style="text-align: center;">
                            <td>{trailer.Schedule.RequestDate.clone()}</td>
                            <td><a onclick={load.clone().reform(move |_| tr.clone())}>{trailer.TrailerID.clone()}</a></td>
                            <td>{trailer.Schedule.CarrierCode.clone()}</td>
                            <td>{render_locations(&trailer.CiscoIDs)}</td>
                            <td>{trailer.Schedule.LastFreeDate.clone()}</td>
                            <td>{trailer.Schedule.ScheduleDate.clone()}</td>
                            <td>{trailer.Schedule.ScheduleTime.clone()}</td>
                            <td>{trailer.Schedule.ArrivalTime.clone()}</td>
                            <td>{trailer.Schedule.DoorNumber.clone()}</td>
                            <td><button style="background-color: #F44336; color: white; padding: 14px 20px; border: none; cursor: pointer; border-radius: 4px;" onclick={toggle_hot.clone().reform(move |_| trailer_id.clone())}>{"Mark Hot"}</button></td>
                        </tr>
                    }
                }) }
                </tbody>
            </table>
        </div>
    }
}
