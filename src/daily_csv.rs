use crate::models::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{js_sys, window};
use yew::prelude::*;
use reqwest::Client;
use gloo::console::log;
use std::{fmt::Write, rc::Rc};
use chrono::prelude::*;

use crate::state::*;

fn render_location(location: &String) -> String {
    match location.as_str() {
        "18008" => "AR".to_string(),
        "18044" => "FF".to_string(),
        "22010" => "40".to_string(),
        _ => "".to_string()  
    }
}

fn format_current_date() -> String {
    let local: DateTime<Local> = Local::now();
    let year = local.year();
    let month = format!("{:02}", local.month());
    let day = format!("{:02}", local.day());
    format!("{}{}{}", year, month, day)
}

fn format_date() -> String {
    let local: DateTime<Local> = Local::now();
    let year = local.year();
    let month = format!("{:02}", local.month());
    let day = format!("{:02}", local.day());
    format!("{}-{}-{}", year, month, day)
}

fn create_csv(data: &Vec<Sids>) -> String {
    let mut csv_string = String::new();
    for trailer_sid in data {
        for sid_part in trailer_sid.Sids.iter() {
            let s = format!("{}{}", trailer_sid.TrailerID, render_location(&sid_part.Cisco));
            let _ = writeln!(csv_string, "{},{},{},DAL,P, ,{},{},{},1", s, sid_part.Part, sid_part.Quantity, render_location(&sid_part.Cisco), format_current_date(), trailer_sid.TrailerID);
        }
    }
    csv_string
}

#[function_component(DailyCsv)]
pub fn daily_csv() -> Html {

    let details = use_state(|| Rc::new(Vec::<Sids>::new()));
    let app_state = use_context::<AppStateContext>().expect("no state found");
    {
        let details = details.clone();
        let app_state = app_state.clone();
        use_effect_with((), move |_| {
            let details = details.clone();
            let app_state = app_state.clone();
            spawn_local(async move {
                let client = Client::new();
                let request = TodaysTrucksRequest {
                    date: format_date()
                };
                if let Some(user) = &app_state.user {
                    match client.post("http://192.168.4.122:8000/api/trailers")
                        .json(&request)
                        .header("Authorization", format!("Bearer {}", user.token))
                        .send()
                        .await {
                            Ok(resp) => {
                                match resp.json::<Vec<Sids>>().await {
                                    Ok(csv_response) => {
                                        details.set(Rc::new(csv_response));
                                    },
                                    Err(error) => log!(format!("{:?}", error))
                                }
                            },
                            Err(error) => {
                                log!(format!("Failed to login: {:?}", error));
                                app_state.dispatch(AppStateAction::ClearUser);
                            },
                        }
                }
            });
            || ()
        })
    }

    let download_csv = {
        let data = details.clone();
        Callback::from(move |_: MouseEvent| {
            let csv_string = create_csv(&data);
            let filename = "daily.csv";
            let window = window().unwrap();
            let document = window.document().unwrap();
            let element = document.create_element("a").unwrap();
            element.set_attribute("href", &format!("data:text/csv;charset=utf-8,{}", js_sys::encode_uri_component(&csv_string))).unwrap();
            element.set_attribute("download", filename).unwrap();
            let body = document.body().unwrap();
            body.append_child(&element).unwrap();
            let event = document.create_event("MouseEvent").unwrap();
            event.init_event("click");
            element.dispatch_event(&event).unwrap();
            body.remove_child(&element).unwrap();
        })
    };

    html! {
        <div style="margin: 3%; display: flex; flex-direction: row; justify-content: space-evenly;">
            <button style="background-color: green; color: white; padding: 14px 20px; border: none; cursor: pointer; border-radius: 4px;" onclick={download_csv}>{"Download All Receipts"}</button>
        </div>
    }
}