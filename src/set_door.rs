use serde_json::json;
use web_sys::HtmlInputElement;
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

#[function_component(SetDoor)]
pub fn set_door() -> Html {

    let app_state = use_context::<AppStateContext>().expect("no state found");
    let shipment = app_state.current_shipment.as_ref().unwrap().clone();
    let door = use_state(|| shipment.TrailerNum.clone());
    log!(format!("{:?}", shipment.clone()));
    let set_door = {
        let app_state = app_state.clone();
        let door = door.clone();
        let shipment = shipment.clone();
        Callback::from(move |_| {
            let app_state = app_state.clone();
            let door = door.clone();
            let shipment = shipment.clone();
            spawn_local(async move {
                let client = Client::new();
                if let Some(user) = &app_state.user {
                    let request = SetShipmentDoorRequest {
                        LoadId: shipment.LoadId,
                        Door: (*door).clone()
                    };
                    match client.post("http://localhost:8000/api/shipment_door")
                        .header("Authorization", format!("Bearer {}", user.token))
                        .json(&request)
                        .send()
                        .await {
                            Ok(resp) => {
                                match resp.json::<Shipment>().await {
                                    Ok(shipment) => {
                                        let msg = SetShipmentDoorMessage {
                                            LoadId: shipment.LoadId,
                                            Door: shipment.Door,
                                        };
                                        let json_string = serde_json::to_string(&msg).unwrap();
                                        let message = json!({
                                            "type": "set_shipment_door",
                                            "data": {
                                                "message": json_string
                                            }
                                        }).to_string();
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
        let door = door.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            let id = input.id();
            let value = input.value();

            door.set(value);
        })
    };

    html! {
        <div style="text-align: center;">
            <label for="door">{ "Door" }</label>
            <input style="text-align: center; width: 25vw;" id="door" type="text" value={(*door).clone()} oninput={on_change.clone()} />
            <button style="background-color: green; color: white; padding: 14px 20px; border: none; cursor: pointer; border-radius: 4px;" onclick={set_door}>{"Set Details"}</button>
        </div> 
    }
}

