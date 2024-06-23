use std::rc::Rc;

use crate::{requests::*, state::*};
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
    let fetch_trailers = {
        let trailers = trailers.clone();
        let app_state = app_state.clone();
        Callback::from(move |_| {
            let trailers = trailers.clone();
            let app_state = app_state.clone();
            spawn_local(async move {
                let client = Client::new();
                if let Some(user) = &app_state.user {
                    match client.get("http://192.168.4.92:8000/api/schedule_trailer")
                        .header("Authorization", format!("Bearer {}", user.token))
                        .send()
                        .await {
                            Ok(resp) => {
                                match resp.json::<Vec<TrailerResponse>>().await {
                                    Ok(trailer_response) => trailers.set(Rc::new(trailer_response)),
                                    Err(error) => log!(format!("Error: {:?}", error)),
                                }
                            },
                            Err(error) => log!(format!("{:?}", error))
                        }
                }
            });
        })
    };

    html! {
        <div>
            <h1>{ "Scheduled Trailers" }</h1>
            <button onclick={fetch_trailers}>{ "Fetch Trailers" }</button>
            <table>
                <thead>
                    <tr>
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
                { for trailers.iter().map(|trailer| html! { 
                    <tr>
                        <td>{trailer.Schedule.RequestDate.clone()}</td>
                        <td>{trailer.TrailerID.clone()}</td>
                        <td>{trailer.Schedule.CarrierCode.clone()}</td>
                        <td>{render_locations(&trailer.CiscoIDs)}</td>
                        <td>{trailer.Schedule.LastFreeDate.clone()}</td>
                        <td>{trailer.Schedule.ScheduleDate.clone()}</td>
                        <td>{trailer.Schedule.ScheduleTime.clone()}</td>
                        <td>{trailer.Schedule.ArrivalTime.clone()}</td>
                        <td>{trailer.Schedule.DoorNumber.clone()}</td>
                    </tr>
                }) }
                </tbody>
            </table>
        </div>
    }
}
