use std::rc::Rc;

use crate::{models::*, AppStateAction};
use wasm_bindgen_futures::spawn_local;
use web_sys::{wasm_bindgen::{prelude::*, JsCast}, Event, FileReader, HtmlInputElement, js_sys, window};
use yew::prelude::*;
use reqwest::Client;
use gloo::console::log;
use std::fmt::Write;
use csv::ReaderBuilder;

use crate::{AppState, AppStateContext};


fn create_csv() -> String {
    let mut csv_string = String::new();
    let _ = writeln!(csv_string, "item,quantity,ip");
    csv_string
}

#[function_component(ShipmentDetails)]
pub fn shipment_details() -> Html {
    let app_state = use_context::<AppStateContext>().expect("no app state found");
    let details = use_state(|| Rc::new(Vec::<ShipmentLine>::new()));
    let data = use_state(|| vec![]);
    
    let download_csv = {
        let data = details.clone();
        let app_state = app_state.clone();
        Callback::from(move |_: MouseEvent| {
            let csv_string = create_csv();
            let filename = "upload_template.csv";
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
                    if let Some(shp) = &app_state.current_shipment{
                        let request = ShipmentLoadingMessage {
                            LoadId: shp.LoadId.clone()
                        };
                        if let Some(user) = &app_state.user {
                            match client.post("http://localhost:8000/api/get_shipment_details")
                                .json(&request)
                                .header("Authorization", format!("Bearer {}", user.token))
                                .send()
                                .await {
                                    Ok(resp) => {
                                        match resp.json::<Vec<ShipmentLine>>().await {
                                            Ok(load_response) => {
                                                details.set(Rc::new(load_response.clone()));
                                            },
                                            Err(error) => {
                                                log!(format!("{:?}", error));
                                                app_state.dispatch(AppStateAction::ClearUser);
                                            },
                                        }
                                    },
                                    Err(error) => {
                                        log!(format!("Failed to login: {:?}", error));
                                        app_state.dispatch(AppStateAction::ClearUser);
                                    },
                                }
                        }
                    }
                });
    
                || ()
            });
        }
    }

    let on_file_change = {
        let data = data.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Some(files) = input.files() {
                let infile = files.get(0).unwrap();
                let file = Rc::new(infile);
                
                let onload = {
                    let data = data.clone();
                    Callback::from(move |file_content: String| {
                        let mut rdr = ReaderBuilder::new()
                            .has_headers(true) // Skip the headers row
                            .from_reader(file_content.as_bytes()); // Reading as bytes

                        let mut new_data = Vec::<ShipmentLine>::new();
                        for result in rdr.deserialize() {
                            match result {
                                Ok(record) => new_data.push(record), // Correct type deserialization
                                Err(err) => log!(format!("Error deserializing row: {:?}", err)),
                            }
                        }

                        data.set(new_data); // Store parsed data in state
                    })
                };

                let file = Rc::clone(&file);
                spawn_local(async move {
                    let file_reader = FileReader::new().unwrap();
                    let onload_clone = onload.clone();
                    
                    // Set up the file reader to read as text
                    let reader_clone = file_reader.clone();
                    file_reader.set_onload(Some(Closure::once_into_js(move || {
                        let content = reader_clone.result().unwrap();
                        if let Some(text) = content.as_string() {
                            onload_clone.emit(text);
                        }
                    }).unchecked_ref()));
                    
                    file_reader.read_as_text(&file).unwrap();
                });
            }
        })
    };

    let upload_csv = {
        let app_state = app_state.clone();
        let data = (*data).clone();
        Callback::from(move |_| {
            let app_state = app_state.clone();
            let data = data.clone();
            spawn_local(async move {
                let client = Client::new();
                if let Some(user) = &app_state.user {
                    let request = ShipmentLineUploadRequest {
                        LoadId: app_state.current_shipment.as_ref().unwrap().LoadId.clone(),
                        Lines: data,
                    };
                    match client.post("http://localhost:8000/api/shipment_lines")
                        .header("Authorization", format!("Bearer {}", user.token))
                        .json(&request)
                        .send()
                        .await {
                            Ok(resp) => {
                                match resp.json::<Vec<ShipmentLine>>().await {
                                    Ok(load_response) => {
                                        log!(format!("{:?}", load_response));
                                        app_state.dispatch(AppStateAction::SetCurrentView("shipments".to_string()));
                                    },
                                    Err(error) => {
                                        log!(format!("{:?}", error));
                                        app_state.dispatch(AppStateAction::ClearUser);
                                    },
                                }
                            },
                            Err(e) => {
                                log!(format!("Error sending request: {:?}", e));
                            }
                        }
                }
            })
        })
    };

    let shipment = app_state.current_shipment.clone();

    html! {
        <>
            {
                if let Some(shipment) = shipment {
                    html! {
                        <div style="margin-top: 7vh;">
                            <h1 style="text-align: center;">{"Load Details: "} {shipment.LoadId}</h1>
                            <div style="margin: 3%; display: flex; width: 70vw; flex-direction: row; justify-content: space-evenly;">
                                <a onclick={download_csv}>{"Download Upload Template"}</a>
                            </div>
                            <table>
                                <thead>
                                    <tr>
                                        <td style="text-align: center;">{"Part"}</td>
                                        <td style="text-align: center;">{"Quantity"}</td>
                                        <td style="text-align: center;">{"IP"}</td>
                                    </tr>
                                </thead>
                                <tbody>
                                    { for details.iter().map(|line|
                                        html! {
                                            <tr style="text-align: center;">
                                                <td style="text-align: center;">{line.item.clone()}</td>
                                                <td style="text-align: center;">{line.quantity.clone()}</td>
                                                <td style="text-align: center;">{line.ip.clone()}</td>
                                            </tr>
                                        }
                                    )}
                                </tbody>
                            </table>
                            <h1 style="text-align: center;">{ "Shipment Line Upload" }</h1>
                            <div style="margin: 3%; display: flex; width: 70vw; flex-direction: row; justify-content: space-evenly;">
                                <input type="file" accept=".csv" onchange={on_file_change} />
                                <button onclick={upload_csv}>{"Upload"}</button>
                                <ul>
                                    { for data.iter().map(|item| html! {
                                        <li>{ format!("Part: {}, Quantity: {}, Ip: {}", 
                                            item.item, item.quantity, item.ip)}</li>
                                    }) }
                                </ul>
                            </div>
                        </div>
                    }
                } else {
                    html! {
                        <h1 style="text-align: center;">{"No Shipment Selected"}</h1>
                    }
                }
            }
        </>
    }
}