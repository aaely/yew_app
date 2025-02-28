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

#[function_component(VerifiedBy)]
pub fn verified_by() -> Html {

    let app_state = use_context::<AppStateContext>().expect("no state found");
    let shipment = app_state.current_shipment.as_ref().unwrap().clone();
    let verifier = use_state(|| shipment.VerifiedBy.clone());
    
    let set_pick_start = {
        let app_state = app_state.clone();
        let verifier = verifier.clone();
        let shipment = shipment.clone();
        Callback::from(move |_| {
            let app_state = app_state.clone();
            let verifier = verifier.clone();
            let shipment = shipment.clone();
            spawn_local(async move {
                let client = Client::new();
                if let Some(user) = &app_state.user {
                    let request = VerifiedByRequest {
                        LoadId: shipment.LoadId,
                        VerifiedBy: (*verifier).clone()
                    };
                    match client.post("http://localhost:8000/api/shipment_verification")
                        .header("Authorization", format!("Bearer {}", user.token))
                        .json(&request)
                        .send()
                        .await {
                            Ok(resp) => {
                                match resp.json::<Shipment>().await {
                                    Ok(shipment) => {
                                        let msg = VerifiedByMessage {
                                            LoadId: shipment.LoadId,
                                            VerifiedBy: shipment.VerifiedBy,
                                        };
                                        let json_string = serde_json::to_string(&msg).unwrap();
                                        let message = json!({
                                            "type": "verified_by",
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
        let verifier = verifier.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            let id = input.id();
            let value = input.value();

            verifier.set(value);
        })
    };

    html! {
        <div style="text-align: center;">
            <h1>{"Load: "} {shipment.LoadId}</h1>
            <label for="verifier">{ "Verified By" }</label>
            <input style="text-align: center; width: 25vw;" id="picker" type="text" value={(*verifier).clone()} oninput={on_change.clone()} />
            <button style="background-color: green; color: white; padding: 14px 20px; border: none; cursor: pointer; border-radius: 4px;" onclick={set_pick_start}>{"Set Details"}</button>
        </div> 
    }
}

