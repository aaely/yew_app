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
        "VERIFICATION" => "orange".to_string(),
        "NOT STARTED" => "aqua".to_string(),
        _ => "".to_string(),
    }
}

fn format_current_date() -> String {
    let local: DateTime<Local> = Local::now();
    let year = local.year();
    let month = format!("{:02}", local.month());
    let day = format!("{:02}", local.day());
    format!("{}-{}-{}", year, month, day)
}

fn total_expected(shipments: &Vec<Shipment>) -> (u32, u32, u32, u32, u32, u32, u32) {
    let (mut ns, mut p, mut rtl, mut ld, mut cmp, mut hld, mut vl) = (0,0,0,0,0,0,0);
    for shipment in shipments {
        match (shipment.Status.as_str(), shipment.IsHold) {
            ("NOT STARTED", false) => ns += 1,
            ("READY TO LOAD", false) => rtl += 1,
            ("LOADING", false) => ld += 1,
            ("COMPLETE", false) => cmp += 1,
            ("VERIFICATION", false) => vl += 1,
            ("PICKING",  false) => p += 1,
            _ => hld += 1,
        }
    }
    (ns, p, rtl, ld, cmp, vl, hld)
}

fn total_by_plant(shipments: &Vec<Shipment>) -> (u32, u32) {
    let (mut ar, mut sh) = (0, 0);
    for shipment in shipments {
        match shipment.Dock.to_lowercase().as_str() {
            "73y" => sh += 1,
            "74y" => sh += 1,
            "75y" => sh += 1,
            "uuu" => ar += 1,
            "vaa" => ar += 1,
            _ => todo!(),
        }
    }
    (ar, sh)
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


#[function_component(TodaysShipments)]
pub fn shipments() -> Html {

    let app_state = use_context::<AppStateContext>().expect("no state found");
    {
        let app_state = app_state.clone();
        use_effect_with((), move |_| {
            let app_state = app_state.clone();

            spawn_local(async move {
                let client = Client::new();
                let request = TodaysTrucksRequest {
                    date: format_current_date(),
                };
                if let Some(user) = &app_state.user {
                    match client.post("http://localhost:8000/api/get_todays_shipments")
                        .json(&request)
                        .header("Authorization", format!("Bearer {}", user.token))
                        .send()
                        .await {
                            Ok(resp) => {
                                match resp.json::<Vec<Shipment>>().await {
                                    Ok(shipments) => {
                                        let mut sh = shipments.clone();
                                        sh.sort_by(|a, b| {
                                            let (hours_a, minutes_a) = parse_time(&a.ScheduleTime);
                                            let (hours_b, minutes_b) = parse_time(&b.ScheduleTime);
                                            if hours_a == hours_b && minutes_a == minutes_b {
                                                a.Dock.cmp(&b.Dock)
                                            } else if hours_a == hours_b && minutes_a != minutes_b {
                                                minutes_a.cmp(&minutes_b)
                                            } else {
                                                hours_a.cmp(&hours_b)
                                            }
                                        });
                                        app_state.dispatch(AppStateAction::SetShipments(sh));
                                    },
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

    let set_picker = {
        let app_state = app_state.clone();
        Callback::from(move |shipment: Shipment| {
            app_state.dispatch(AppStateAction::SetCurrentShipment(shipment));
            app_state.dispatch(AppStateAction::SetCurrentView("set_picker".to_string()));
        })
    };

    let hold_load = {
        let app_state = app_state.clone();
        Callback::from(move |shipment: Shipment| {
            let app_state = app_state.clone();
            let shipment = shipment.clone();
            spawn_local(async move {
                let client = Client::new();
                if let Some(user) = &app_state.user {
                    let request = ShipmentLoadingRequest {
                        LoadId: shipment.LoadId.clone(),
                    };
                    match client.post("http://localhost:8000/api/shipment_hold")
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
                                            "type": "shipment_hold",
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

    let set_trailer = {
        let app_state = app_state.clone();
        Callback::from(move |obj: (String, Shipment)| {
            app_state.dispatch(AppStateAction::SetCurrentShipment(obj.1));
            app_state.dispatch(AppStateAction::SetCurrentView(obj.0));
        })
    };

    let change = {
        let app_state = app_state.clone();
        Callback::from(move |view: String| {
            app_state.dispatch(AppStateAction::SetCurrentView(view));
        })
    };

    html! {
        <div style="margin-top: 7vh; width: 90vw;">
            <h1 style="text-align: center;">{ "Today's Shipments" }</h1>
            <div style="text-align: center; color: blue; text-decoration: underline;" onclick={change.clone().reform(move |_| "shipments".to_string())}><h5>{"Recent"}</h5></div>
            <div style="margin: 3%; display: flex; width: 100%; flex-direction: row; justify-content: space-evenly;">
            <h3 style="text-align: center;">{"Total: "} {app_state.shipments.len()}</h3><h3 style="text-align: center;">{"Complete: "} {total_expected(&app_state.shipments).4}</h3>
            </div>
            <div style="margin: 3%; display: flex; width: 100%; flex-direction: row; justify-content: space-evenly;">
            <h3 style="text-align: center;">{"AR: "} {total_by_plant(&app_state.shipments).0}</h3><h3 style="text-align: center;">{"SH: "} {total_by_plant(&app_state.shipments).1}</h3>
            </div>
            <div style="margin: 3%; display: flex; width: 100%; flex-direction: row; justify-content: space-evenly;">
            <h3>{"Not Started: "} {total_expected(&app_state.shipments).0}</h3> <h3>{"Picking: "} {total_expected(&app_state.shipments).1}</h3>
            <h3>{"Hold: "} {total_expected(&app_state.shipments).6}</h3> 
            </div>
            <div style="margin: 3%; display: flex; width: 100%; flex-direction: row; justify-content: space-evenly;">
            <h3>{"Ready To Load: "} {total_expected(&app_state.shipments).2}</h3>
            <h3>{"Loading: "} {total_expected(&app_state.shipments).3}</h3> <h3>{"Verification: "} {total_expected(&app_state.shipments).5}</h3>
            </div>
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
                    let ship3 = shipment.clone();
                    let ship4 = shipment.clone();
                    let ship5 = shipment.clone();
                    html! {
                        <tr style="text-align: center;">
                            <td>{index + 1}</td>
                            <td><a onclick={set_trailer.clone().reform(move |_| ("shipment_details".to_string(), ship5.clone()))}>{shipment.LoadId.clone()}</a></td>
                            {
                                if shipment.IsHold {
                                    html! {
                                        <td style="background-color: red;">
                                            {"HOLD"}
                                        </td>
                                    } 
                                } else {
                                        html! {
                                            <td style={format!("background-color: {};", get_background(shipment.Status.as_str()))}>{shipment.Status.clone()}</td>
                                        }
                                }
                            }
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
                            {
                                if shipment.Status == "PICKING" && user.is_authorized() {
                                    html! {
                                        <td>
                                            <a onclick={set_picker.clone().reform(move |_| ship4.clone())}>{shipment.Picker.clone()}</a>
                                        </td>
                                    }
                                } else {
                                    html! {
                                        <td>
                                            {shipment.Picker.clone()}
                                        </td>
                                    }
                                }
                            }
                            <td>{shipment.PickStartTime.clone()}</td>
                            <td>{shipment.PickFinishTime.clone()}</td>
                            <td>{shipment.VerifiedBy.clone()}</td>
                            <td>
                                <ActionButton user={user.clone()} shipment={shipment.clone()} />
                            </td>
                            {
                                if user.role == "admin".to_string() && shipment.Status != "COMPLETE".to_string() {
                                    html! {
                                        <td>
                                            <button style="
                                            background-color: yellow; 
                                            color: black; 
                                            padding: 14px 20px; 
                                            border: none; 
                                            cursor: pointer; 
                                            border-radius: 4px;
                                            " 
                                            onclick={hold_load.clone().reform(move |_| ship3.clone())}>
                                                { "HOLD" }
                                            </button>
                                        </td>
                                    }
                                } else {
                                    html! {<></>}
                                }
                            }
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
                    app_state.dispatch(AppStateAction::SetCurrentView("depart".to_string()));
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