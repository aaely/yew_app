use serde_json::json;
use web_sys::{HtmlInputElement, KeyboardEvent};
use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use reqwest::Client;
use gloo::console::log;
use crate::{models::*, state::AppStateContext, AppStateAction};
use chrono::prelude::*;

#[function_component(Shipments)]
pub fn shipments() -> Html {

    let app_state = use_context::<AppStateContext>().expect("no state found");

    {
        let app_state = app_state.clone();
        use_effect_with((), move |_| {
            let app_state = app_state.clone();

            spawn_local(async move {
                let client = Client::new();
                if let Some(user) = &app_state.user {
                    match client.get("http://192.168.4.172:8000/api/get_shipments")
                        .header("Authorization", format!("Bearer {}", user.token))
                        .send()
                        .await {
                            Ok(resp) => {
                                match resp.json::<Vec<Shipment>>().await {
                                    Ok(shipments) => app_state.dispatch(AppStateAction::SetShipments(shipments)),
                                    Err(error) => {
                                        log!(format!("{:?}", error));
                                        app_state.dispatch(AppStateAction::ClearUser);
                                    }
                                }
                            },
                            Err(error) => log!(format!("{:?}", error))
                        }
                }
            });
        });
    }

    html! {
        <div style="margin-top: 7vh; width: 90vw;">
            <h1 style="text-align: center;">{ "Most Recent Shipments" }</h1>
            <table>
                <thead>
                    <tr style="text-align: center;">
                        <th>{"#"}</th>
                        <th>{"LoadId"}</th>
                        <th>{"Status"}</th>
                        <th>{"Scheduled Date"}</th>
                        <th>{"Scheduled Time"}</th>
                        <th>{"Arrival Time"}</th>
                        <th>{"Departure Time"}</th>
                        <th>{"Door"}</th>
                        <th>{"Dock"}</th>
                        <th>{"Trailer Number"}</th>
                        <th>{"Load Number"}</th>
                        <th>{"Picker"}</th>
                        <th>{"Pick Start Time"}</th>
                        <th>{"Verified By"}</th>
                    </tr>
                </thead>
                <tbody>
                { app_state.shipments.iter().enumerate().map(|(index, shipment)| {
                    let user = app_state.user.as_ref().unwrap();
                    html! {
                        <tr style="text-align: center;">
                            <td>{index + 1}</td>
                            <td>{shipment.LoadId.clone()}</td>
                            <td>{shipment.Status.clone()}</td>
                            <td>{shipment.ScheduleDate.clone()}</td>
                            <td>{shipment.ScheduleTime.clone()}</td>
                            <td>{shipment.ArrivalTime.clone()}</td>
                            <td>{shipment.DepartTime.clone()}</td>
                            <td>{shipment.Door.clone()}</td>
                            <td>{shipment.Dock.clone()}</td>
                            <td>{shipment.TrailerNum.clone()}</td>
                            <td>{shipment.LoadNum.clone()}</td>
                            <td>{shipment.Picker.clone()}</td>
                            <td>{shipment.PickStartTime.clone()}</td>
                            <td>{shipment.VerifiedBy.clone()}</td>
                        </tr>
                    }
                }).collect::<Html>() }
                </tbody>
            </table>
        </div>
    }
}