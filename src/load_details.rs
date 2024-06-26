use std::rc::Rc;

use crate::{requests::*, state::*};
use wasm_bindgen_futures::spawn_local;
use web_sys::{js_sys, window};
use yew::prelude::*;
use reqwest::Client;
use gloo::console::log;
use std::fmt::Write;
use chrono::prelude::*;

use crate::{AppState, AppStateContext};

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

fn create_csv(data: &Vec<SidParts>, app_state: &AppState) -> String {
    let mut csv_string = String::new();
    for sid in data {
        let s = format!("{}{}", app_state.current_trailer.as_ref().unwrap().TrailerID, render_location(&sid.Sid.CiscoID));
        for part in &sid.Parts {
            writeln!(csv_string, "{},{},{},DAL,P, ,{},{},{},1", s, part.partNumber, part.quantity, render_location(&sid.Sid.CiscoID), format_current_date(), app_state.current_trailer.as_ref().unwrap().TrailerID);
        }
    }
    csv_string
}

#[function_component(LoadDetails)]
pub fn load_details() -> Html {
    let app_state = use_context::<AppStateContext>().expect("no app state found");
    let details = use_state(|| Rc::new(Vec::<SidParts>::new()));

    let download_csv = {
        let data = details.clone();
        let app_state = app_state.clone();
        Callback::from(move |_: MouseEvent| {
            let csv_string = create_csv(&data, &app_state);
            let filename = "data.csv";
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

    {
        let app_state = app_state.clone();
        {
            let app_state = app_state.clone();
            let details = details.clone();
            use_effect_with((), move |_| {
                let app_state = app_state.clone();
                let details = details.clone();
                spawn_local(async move {
                    let client = Client::new();
                    if let Some(trl) = &app_state.current_trailer{
                        let request = LoadInfoRequest {
                            param: trl.TrailerID.clone()
                        };
                        if let Some(user) = &app_state.user {
                            match client.post("http://192.168.4.97:8000/api/get_load_info")
                                .json(&request)
                                .header("Authorization", format!("Bearer {}", user.token))
                                .send()
                                .await {
                                    Ok(resp) => {
                                        match resp.json::<Vec<SidParts>>().await {
                                            Ok(load_response) => {
                                                details.set(Rc::new(load_response.clone()));
                                            },
                                            Err(error) => log!(format!("Error: {:?}", error)),
                                        }
                                    },
                                    Err(error) => log!(format!("{:?}", error))
                                }
                        }
                    }
                });
    
                || ()
            });
        }
    }

    let all_trailers = {
        let app_state = app_state.clone();
        Callback::from(move |_| {
            app_state.dispatch(AppStateAction::SetCurrentView("landing".to_string()));
        })
    };

    let trailer = app_state.current_trailer.clone();

    html! {
        <>
            {
                if let Some(trailer) = trailer {
                    html! {
                        <div class="container">
                        <h1 style="text-align: center;">{"Load Details: "} {trailer.TrailerID}</h1>
                        { for details.iter().map(|sids| 
                            
                            html! {
                                <>
                                <h3 style="text-align: center">{sids.Sid.id.clone()}{"  ||  "}{sids.Sid.CiscoID.clone()}</h3>
                                <table>
                                    <thead>
                                        <tr style="text-align: center;">
                                            <td>{"Part"}</td>
                                            <td>{"Quantity"}</td>
                                        </tr>
                                    </thead>
                                    <tbody>
                                { for sids.Parts.iter().map(|part|
                                    html! {
                                        <tr style="text-align: center;">
                                            <td>{part.partNumber.clone()}</td>
                                            <td>{part.quantity}</td>
                                        </tr>
                                    }
                                )}
                                    </tbody>
                                </table>
                                </>
                            }
                        )}
                        <button style="background-color: #F44336; color: white; padding: 14px 20px; border: none; cursor: pointer; border-radius: 4px; text-align: center;" onclick={download_csv}>{"Download CSV"}</button>

                        <button style="background-color: #F44336; color: white; padding: 14px 20px; border: none; cursor: pointer; border-radius: 4px; text-align: center;" onclick={all_trailers.clone()}>{"All Trailers"}</button>
                        </div>
                    }
                } else {
                    html! {
                        <h1 style="text-align: center;">{"No Trailer Selected"}</h1>
                    }
                }
            }
        </>
    }
}