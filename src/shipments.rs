use serde_json::json;
use web_sys::{HtmlInputElement, KeyboardEvent};
use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use reqwest::Client;
use gloo::console::log;
use crate::{models::*, state::AppStateContext, AppStateAction, float_button::FloatingIcon};
use chrono::prelude::*;

fn time() -> String {
    let now = Local::now();
    format!("{:02}:{:02}:{:02}", now.hour(), now.minute(), now.second())
}

fn get_background(status: &str) -> String {
    match status {
        "PICKING" => "yellow".to_string(),
        "READY TO LOAD" => "blue".to_string(),
        "LOADING" => "green".to_string(),
        "COMPLETE" => "gray".to_string(),
        "HOLD" => "red".to_string(),
        "VERIFICATION" => "orange".to_string(),
        "NOT STARTED" => "aqua".to_string(),
        _ => "".to_string(),
    }
}

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
                    match client.get("http://localhost:8000/api/get_shipments")
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

    let set_trailer = {
        let app_state = app_state.clone();
        Callback::from(move |obj: (String, Shipment)| {
            app_state.dispatch(AppStateAction::SetCurrentShipment(obj.1));
            app_state.dispatch(AppStateAction::SetCurrentView(obj.0));
        })
    };

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
                        <th>{"Pick Finish Time"}</th>
                        <th>{"Verified By"}</th>
                    </tr>
                </thead>
                <tbody>
                { app_state.shipments.iter().enumerate().map(|(index, shipment)| {
                    let user = app_state.user.as_ref().unwrap();
                    let ship = shipment.clone();
                    let ship2 = shipment.clone();
                    html! {
                        <tr style="text-align: center;">
                            <td>{index + 1}</td>
                            <td>{shipment.LoadId.clone()}</td>
                            <td style={format!("background-color: {};", get_background(shipment.Status.as_str()))}>{shipment.Status.clone()}</td>
                            <td>{shipment.ScheduleDate.clone()}</td>
                            <td>{shipment.ScheduleTime.clone()}</td>
                            {
                                if user.is_authorized() && shipment.ArrivalTime.len() == 0 {
                                    html! {<td><button style="background-color: blue; color: white; padding: 14px 20px; border: none; cursor: pointer; border-radius: 4px;" onclick={set_trailer.clone().reform(move |_| ("set_trailer".to_string(), ship2.clone()))}>{ "Set Trailer" }</button></td>}
                                } else {
                                    html! {<td>{shipment.ArrivalTime.clone()}</td>}
                                }
                            }
                            <td>{shipment.DepartTime.clone()}</td>
                            {
                                if user.is_authorized() && shipment.Door.len() == 0 {
                                    html! {<td><a onclick={set_trailer.clone().reform(move |_| ("set_door".to_string(), ship.clone()))}>{ "Set Door" }</a></td>}
                                } else if user.is_authorized() && shipment.Status.as_str() != "COMPLETE" {
                                    html! {<td><a onclick={set_trailer.clone().reform(move |_| ("set_door".to_string(), ship.clone()))}>{shipment.Door.clone()}</a></td>}
                                } else {
                                    html! {<td>{shipment.Door.clone()}</td>}
                                }
                            }
                            <td>{shipment.Dock.clone()}</td>
                            <td>{shipment.TrailerNum.clone()}</td>
                            <td>{shipment.LoadNum.clone()}</td>
                            <td>{shipment.Picker.clone()}</td>
                            <td>{shipment.PickStartTime.clone()}</td>
                            <td>{shipment.PickFinishTime.clone()}</td>
                            <td>{shipment.VerifiedBy.clone()}</td>
                            <td>
                                <ActionButton user={user.clone()} shipment={shipment.clone()} />
                            </td>
                        </tr>
                    }
                }).collect::<Html>() }
                </tbody>
            </table>
            <FloatingIcon />
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct PropTypes {
    user: User,
    shipment: Shipment,
}

#[function_component(ActionButton)]
fn action_button(props: &PropTypes) -> Html {

    let app_state = use_context::<AppStateContext>().expect("no state found");

    let finish_picking = {
        let app_state = app_state.clone();
        let shipment = props.shipment.clone();
        Callback::from(move |_| {
            let app_state = app_state.clone();
            let shipment = shipment.clone();
            spawn_local(async move {
                let client = Client::new();
                if let Some(user) = &app_state.user {
                    let request = ShipmentPickFinishRequest {
                        LoadId: shipment.LoadId.clone(),
                        FinishTime: time(),
                    };
                    log!(format!("{:?}",request.clone()));
                    match client.post("http://localhost:8000/api/shipment_pick_finish")
                        .header("Authorization", format!("Bearer {}", user.token))
                        .json(&request)
                        .send()
                        .await {
                            Ok(resp) => {
                                match resp.json::<Shipment>().await {
                                    Ok(shipment) => {
                                        let msg = PickFinishMessage {
                                            LoadId: shipment.LoadId,
                                            FinishTime: shipment.PickFinishTime,
                                        };
                                        let json_string = serde_json::to_string(&msg).unwrap();
                                        let message = json!({
                                            "type": "finish_shipment_pick",
                                            "data": {
                                                "message": json_string
                                            }
                                        }).to_string();
                                        app_state.send_ws_message(&message);
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

    let start_loading = {
        let app_state = app_state.clone();
        let shipment = props.shipment.clone();
        Callback::from(move |_| {
            let app_state = app_state.clone();
            let shipment = shipment.clone();
            spawn_local(async move {
                let client = Client::new();
                if let Some(user) = &app_state.user {
                    let request = ShipmentLoadingRequest {
                        LoadId: shipment.LoadId.clone(),
                    };
                    match client.post("http://localhost:8000/api/shipment_begin_loading")
                        .header("Authorization", format!("Bearer {}", user.token))
                        .json(&request)
                        .send()
                        .await {
                            Ok(resp) => {
                                match resp.json::<Shipment>().await {
                                    Ok(shipment) => {
                                        let msg = ShipmentLoadingMessage {
                                            LoadId: shipment.LoadId,
                                        };
                                        let json_string = serde_json::to_string(&msg).unwrap();
                                        let message = json!({
                                            "type": "shipment_start_loading",
                                            "data": {
                                                "message": json_string
                                            }
                                        }).to_string();
                                        app_state.send_ws_message(&message);
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

    let depart = {
        let app_state = app_state.clone();
        let shipment = props.shipment.clone();
        Callback::from(move |_| {
            let app_state = app_state.clone();
            let shipment = shipment.clone();
            spawn_local(async move {
                let client = Client::new();
                if let Some(user) = &app_state.user {
                    let request =  ShipmentDepartRequest {
                        LoadId: shipment.LoadId.clone(),
                        DepartTime: time(),
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
                                        };
                                        let json_string = serde_json::to_string(&msg).unwrap();
                                        let message = json!({
                                            "type": "shipment_depart",
                                            "data": {
                                                "message": json_string
                                            }
                                        }).to_string();
                                        app_state.send_ws_message(&message);
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

    let on_action = {
        let app_state = app_state.clone();
        let shipment = props.shipment.clone();
        Callback::from(move |_| {
            app_state.dispatch(AppStateAction::SetCurrentShipment(shipment.clone()));
            
            match shipment.Status.as_str() {
                "NOT STARTED" => app_state.dispatch(AppStateAction::SetCurrentView("set_picker".to_string())),
                "PICKING" => {
                    finish_picking.emit(());
                },
                "READY TO LOAD" => {
                    start_loading.emit(());
                },
                "LOADING" => {
                    depart.emit(())
                },
                "VERIFICATION" => app_state.dispatch(AppStateAction::SetCurrentView("verified_by".to_string())),
                _ => todo!(),
            }
        })
    };

    match props.shipment.Status.as_str() {
        "NOT STARTED" if props.user.is_authorized() => html! {
            <button style="background-color: red; 
                    color: black; 
                    padding: 14px 20px; 
                    border: none; 
                    cursor: pointer; 
                    border-radius: 4px;" 
                    onclick={on_action}>
                        {"Add Picker"}
            </button>
        },
        "PICKING" if props.user.is_authorized() => html! {
            <button style="background-color: orange; 
                    color: black; 
                    padding: 14px 20px; 
                    border: none; 
                    cursor: pointer; 
                    border-radius: 4px;" 
                    onclick={on_action}>
                        {"Finish Pick"}
            </button>
        },
        "READY TO LOAD" if props.user.is_authorized() && props.shipment.ArrivalTime.len() > 0 => html! {
            <button style="background-color: green; 
                    color: black; 
                    padding: 14px 20px; 
                    border: none; 
                    cursor: pointer; 
                    border-radius: 4px;" 
                    onclick={on_action}>
                        {"Start Loading"}
            </button>
        },
        "LOADING" if props.user.is_authorized() => html! {
            <button style="background-color: blue; 
                    color: white; 
                    padding: 14px 20px; 
                    border: none; 
                    cursor: pointer; 
                    border-radius: 4px;" 
                    onclick={on_action}>
                        {"Depart"}
            </button>
        },
        "VERIFICATION" if props.user.is_authorized() => html! {
            <button style="background-color: teal; 
                    color: black; 
                    padding: 14px 20px; 
                    border: none; 
                    cursor: pointer; 
                    border-radius: 4px;" 
                    onclick={on_action}>
                        {"Validate"}
            </button>
        },
        _ => html! {
            <></>
        }
    }
}