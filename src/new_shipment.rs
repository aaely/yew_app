use serde_json::json;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use reqwest::Client;
use gloo::console::log;
use crate::{models::*, state::AppStateContext, AppStateAction, float_button::FloatingIcon};
use chrono::prelude::*;

// Formats a NaiveDate to a string like "MMDDYYYY"
fn format_date(date: NaiveDate) -> String {
    format!("{:02}{:02}{}", date.month(), date.day(), date.year())
}

// Returns today’s date formatted as "YYYY-MM-DD", suitable for an <input type="date" />
fn format_date_form() -> String {
    let local = Local::now();
    format!("{:04}-{:02}-{:02}", local.year(), local.month(), local.day())
}

fn time(t: &str) -> String {
    if t.len() < 1 { 
        return "00:00".to_string();
    } else { 
        return t.to_string(); 
    };
}

/*async fn get_load_id(date: &str, dock: &str) -> String {
    let prefix = get_load_prefix(dock, date.to_string());
    let client = Client::new();
    let request = LoadCountRequest {
        prefix: prefix.clone(),
    };

    match client
        .post("https://10.192.208.6:8443/api/get_load_count")
        .json(&request)
        .send()
        .await
    {
        Ok(resp) => {
            if resp.status().is_success() {
                match resp.json::<serde_json::Value>().await {
                    Ok(json) => {
                        let count = json.get("count")
                            .and_then(|c| c.as_u64())
                            .unwrap_or(0) as u32;
                        let next = count + 1;
                        format!("{}V{}", prefix, next)
                    },
                    Err(err) => {
                        log!(format!("Error parsing JSON: {:?}", err));
                        format!("{}V0", prefix)
                    }
                }
            } else {
                format!("{}V0", prefix)
            }
        }
        Err(err) => {
            log!(format!("Error fetching load count: {:?}", err));
            format!("{}V0", prefix)
        }
    }
}

fn get_load_prefix(dock: &str, date: String) -> String {
    // Parse the date or default to today's date if parsing fails.
    let dt = NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .unwrap_or_else(|_| Local::today().naive_local());
    match dock.to_lowercase().as_str() {
        "uuu"   => format!("ARM131LY_{}_", format_date(dt)),
        "vaa"   => format!("ARM131VY_{}_", format_date(dt)),
        "173y"  => format!("40M173Y_{}_", format_date(dt)),
        "174y"  => format!("40M174Y_{}_", format_date(dt)),
        _       => "".to_string(),
    }
}*/

#[function_component(NewShipment)]
pub fn new_shipment() -> Html {

    let app_state = use_context::<AppStateContext>().expect("no state found"); 

    let form = use_state(|| ShipmentFormData {
        door: "".to_string(),
        dock: "".to_string(),
        schedule_date: format_date_form(),
        schedule_time: "".to_string(),
        trailer: "".to_string(),
        picker: "".to_string(),
        verified_by: "".to_string(),
        load_num: "".to_string(),
        load_id: "".to_string(),
    });

    let on_change = {
        let form = form.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            let id = input.id();
            let value = input.value();

            form.set({
                let mut form_data = (*form).clone();
                match id.as_str() {
                    "door" => form_data.door = value,
                    "dock" => form_data.dock = value,
                    "schedule_date" => form_data.schedule_date = value,
                    "schedule_time" => form_data.schedule_time = value,
                    "trailer" => form_data.trailer = value,
                    "picker" => form_data.picker = value,
                    "verified_by" => form_data.verified_by = value,
                    "load_num" => form_data.load_num = value,
                    "load_id" => form_data.load_id = value,
                    _ => (),
                }
                form_data
            });
        })
    };

    let create_shipment = {
        let app_state = app_state.clone();
        let form = (*form).clone();
        Callback::from(move |_| {
            let app_state = app_state.clone();
            let form = form.clone();
            spawn_local(async move {
                if let Some(user) = &app_state.user {
                    let request = Shipment {
                        ScheduleDate: form.schedule_date,
                        ScheduleTime: form.schedule_time,
                        ArrivalTime: "".to_string(),
                        DepartTime: "".to_string(),
                        Dock: form.dock,
                        Door: form.door,
                        LoadId: form.load_id,
                        LoadNum: form.load_num,
                        Status: "".to_string(),
                        Picker: "".to_string(),
                        PickStartTime: "".to_string(),
                        PickFinishTime: "".to_string(),
                        VerifiedBy: "".to_string(),
                        TrailerNum: "".to_string(),
                        IsHold: false,
                        Seal: "".to_string(),
                    };             
                    let client: Client = Client::new();
                    match client
                        .post("http://localhost:8000/api/new_shipment")
                        .header("Authorization", format!("Bearer {}", user.token))
                        .json(&request)
                        .send()
                        .await 
                    {
                        Ok(response) => {
                            match response.json::<Shipment>().await {
                                Ok(shipment) => {
                                    let json_string = serde_json::to_string(&shipment).unwrap();
                                    let message = json!({
                                        "type": "new_shipment",
                                        "data": {
                                            "message": json_string
                                        }
                                    }).to_string();
                                    log!(format!("{:?}", message.clone()));
                                    app_state.send_ws_message(&message);
                                    app_state.dispatch(AppStateAction::SetCurrentView("shipments".to_string()));
                                },
                                Err(error) => log!(format!("{:?}", error)),
                            }
                        },
                        Err(e) => {
                            log!(format!("Error sending shipment request: {:?}", e));
                        }
                    }
                }
            });
        })
    };

    html! {
        <div>
            <h2>{ "New Shipment" }</h2>
            <form>
                <div>
                    <label for="load_id">{ "Load Id" }</label>
                    <input type="text" id="load_id" value={form.load_id.clone()} oninput={on_change.clone()} />
                </div>
                <div>
                    <label for="schedule_date">{ "Schedule Date" }</label>
                    <input type="date" id="schedule_date" value={form.schedule_date.clone()} oninput={on_change.clone()} />
                </div>
                <div>
                    <label for="schedule_time">{ "Schedule Time" }</label>
                    <input type="text" id="schedule_time" value={form.schedule_time.clone()} oninput={on_change.clone()} />
                </div>
                <div>
                    <label for="dock">{ "Dock" }</label>
                    <input type="text" id="dock" value={form.dock.clone()} oninput={on_change.clone()} />
                </div>
                <div>
                    <label for="door">{ "Door" }</label>
                    <input type="text" id="door" value={form.door.clone()} oninput={on_change.clone()} />
                </div>
                <div>
                    <label for="load_num">{ "Load Number (optional)" }</label>
                    <input type="text" id="load_num" value={form.load_num.clone()} oninput={on_change.clone()} />
                </div>
                <button type="button" onclick={create_shipment}>{ "Create Shipment" }</button>
            </form>
            <FloatingIcon />
        </div>
    }
}