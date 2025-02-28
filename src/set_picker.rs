use serde_json::json;
use web_sys::{HtmlInputElement, KeyboardEvent};
use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use reqwest::Client;
use gloo::console::log;
use crate::{models::*, state::AppStateContext, AppStateAction};
use chrono::prelude::*;

fn time() -> String {
    let now = Local::now();
    format!("{:02}:{:02}:{:02}", now.hour(), now.minute(), now.second())
}

#[function_component(SetPicker)]
pub fn set_picker() -> Html {

    let app_state = use_context::<AppStateContext>().expect("no state found");
    let shipment = app_state.current_shipment.as_ref().unwrap().clone();
    let picker = use_state(|| shipment.Picker.clone());
    
    let set_pick_start = {
        let app_state = app_state.clone();
        let picker = picker.clone();
        let ship = shipment.clone();
        Callback::from(move |_| {
            let app_state = app_state.clone();
            let picker = picker.clone();
            let ship = ship.clone();
            spawn_local(async move {
                let client = Client::new();
                if let Some(user) = &app_state.user {
                    let mut t = String::new();
                    if ship.PickStartTime.len() > 0 {
                        t = ship.PickStartTime;
                    } else {
                        t = time();
                    }
                    let request = PickStartRequest {
                        StartTime: t,
                        LoadId: ship.LoadId,
                        Picker: (*picker).clone()
                    };
                    match client.post("http://localhost:8000/api/set_shipment_pick_start")
                        .header("Authorization", format!("Bearer {}", user.token))
                        .json(&request)
                        .send()
                        .await {
                            Ok(resp) => {
                                match resp.json::<Shipment>().await {
                                    Ok(shipment) => {
                                        let msg = PickStartMessage {
                                            LoadId: shipment.LoadId,
                                            StartTime: shipment.PickStartTime,
                                            Picker: shipment.Picker,
                                        };
                                        let json_string = serde_json::to_string(&msg).unwrap();
                                        let message = json!({
                                            "type": "start_shipment_pick",
                                            "data": {
                                                "message": json_string
                                            }
                                        }).to_string();
                                        log!(format!("{:?}", message.clone()));
                                        app_state.send_ws_message(&message);
                                        app_state.dispatch(AppStateAction::SetCurrentView("shipments".to_string()));
                                    },
                                    Err(e) => {
                                        app_state.dispatch(AppStateAction::ClearUser);
                                        log!(format!("{:?}", e));
                                    }
                                }
                            },
                            Err(e) => log!(format!("{:?}", e)),
                        }
                }
            })
        })
    };

    let on_change = {
        let picker = picker.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            let id = input.id();
            let value = input.value();

            picker.set(value);
        })
    };

    html! {
        <div style="text-align: center;">
            <h1>{"Load: "} {shipment.LoadId}</h1>
            <label for="picker">{ "Picker" }</label>
            <input style="text-align: center; width: 25vw;" id="picker" type="text" value={(*picker).clone()} oninput={on_change.clone()} />
            <button style="background-color: green; color: white; padding: 14px 20px; border: none; cursor: pointer; border-radius: 4px;" onclick={set_pick_start}>{"Set Details"}</button>
        </div> 
    }
}

