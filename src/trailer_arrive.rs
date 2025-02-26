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

#[function_component(SetTrailer)]
pub fn set_trailer() -> Html {

    let app_state = use_context::<AppStateContext>().expect("no state found");
    let shipment = app_state.current_shipment.as_ref().unwrap().clone();
    let trailer = use_state(|| shipment.TrailerNum.clone());

    let set_trailer = {
        let app_state = app_state.clone();
        let trailer = trailer.clone();
        let shipment = shipment.clone();
        Callback::from(move |_| {
            let app_state = app_state.clone();
            let trailer = trailer.clone();
            let shipment = shipment.clone();
            spawn_local(async move {
                let client = Client::new();
                if let Some(user) = &app_state.user {
                    let request = TrailerArrivalRequest {
                        ArrivalTime: time(),
                        LoadId: shipment.LoadId,
                        TrailerNum: (*trailer).clone()
                    };
                    match client.post("http://172.16.1.172:8000/api/set_shipment_trailer")
                        .header("Authorization", format!("Bearer {}", user.token))
                        .json(&request)
                        .send()
                        .await {
                            Ok(resp) => {
                                match resp.json::<Shipment>().await {
                                    Ok(shipment) => {
                                        let msg = TrailerArrivalMessage {
                                            LoadId: shipment.LoadId,
                                            ArrivalTime: shipment.ArrivalTime,
                                            TrailerNum: shipment.TrailerNum,
                                        };
                                        let json_string = serde_json::to_string(&msg).unwrap();
                                        let message = json!({
                                            "type": "shipment_trailer_arrival",
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
        let trailer = trailer.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            let id = input.id();
            let value = input.value();

            trailer.set(value);
        })
    };

    html! {
        <div style="text-align: center;">
            <label for="trailer">{ "Trailer" }</label>
            <input style="text-align: center; width: 25vw;" id="trailer" type="text" value={(*trailer).clone()} oninput={on_change.clone()} />
            <button style="background-color: green; color: white; padding: 14px 20px; border: none; cursor: pointer; border-radius: 4px;" onclick={set_trailer}>{"Set Details"}</button>
        </div> 
    }
}

