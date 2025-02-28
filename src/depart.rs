use serde_json::json;
use web_sys::{HtmlInputElement};
use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use reqwest::Client;
use gloo::console::log;
use crate::{models::*, state::{AppState, AppStateContext}, AppStateAction};
use chrono::prelude::*;

fn time() -> String {
    let now = Local::now();
    format!("{:02}:{:02}:{:02}", now.hour(), now.minute(), now.second())
}

#[function_component(Depart)]
pub fn set_picker() -> Html {

    let app_state = use_context::<AppStateContext>().expect("no state found");
    let shipment = app_state.current_shipment.as_ref().unwrap().clone();
    let seal = use_state(|| shipment.Seal.clone());
    
    let depart = {
        let app_state = app_state.clone();
        let shipment = shipment.clone();
        let seal = (*seal).clone();
        Callback::from(move |_| {
            let app_state = app_state.clone();
            let shipment = shipment.clone();
            let seal = seal.clone();
            spawn_local(async move {
                let client = Client::new();
                if let Some(user) = &app_state.user {
                    let request =  ShipmentDepartRequest {
                        LoadId: shipment.LoadId.clone(),
                        DepartTime: time(),
                        Seal: seal,
                    };
                    match client.post("http://localhost:8000/api/set_shipment_departureTime")
                        .header("Authorization", format!("Bearer {}", user.token))
                        .json(&request)
                        .send()
                        .await {
                            Ok(resp) => {
                                match resp.json::<Shipment>().await {
                                    Ok(shipment) => {
                                        let msg = ShipmentDepartRequest {
                                            LoadId: shipment.LoadId,
                                            DepartTime: shipment.DepartTime,
                                            Seal: shipment.Seal,
                                        };
                                        let json_string = serde_json::to_string(&msg).unwrap();
                                        let message = json!({
                                            "type": "shipment_depart",
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
                                    },
                                }
                            },
                            Err(e) => log!(format!("{:?}", e)),
                        }
                }
            })
        })
    };

    let on_change = {
        let seal = seal.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            let id = input.id();
            let value = input.value();

            seal.set(value);
        })
    };

    html! {
        <div style="text-align: center;">
            <h1>{"Load: "} {shipment.LoadId}</h1>
            <label for="seal">{ "Seal" }</label>
            <input style="text-align: center; width: 25vw;" id="seal" type="text" value={(*seal).clone()} oninput={on_change.clone()} />
            <button style="background-color: green; color: white; padding: 14px 20px; border: none; cursor: pointer; border-radius: 4px;" onclick={depart}>{"Set Details"}</button>
        </div> 
    }
}

