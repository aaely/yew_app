use wasm_bindgen_futures::spawn_local;
use web_sys::{wasm_bindgen::{prelude::*, JsCast}, Event, FileReader, HtmlInputElement, js_sys, window};
use yew::prelude::*;
use reqwest::Client;
use gloo::console::log;
use std::{fmt::Write, rc::Rc};
use crate::{models::Item, gmap::Gmap, fix_parts::FixParts};
use csv::ReaderBuilder;
use std::collections::HashSet;



fn plant_code(code: &str) -> String {
    match code {
        "AR" => "ARLINGTON".to_string(),
        "FF" => "FAIRFAX".to_string(),
        "40" => "SPRING HILL".to_string(),
        _ => "MULTI".to_string(),
    }
}

fn pallet_size(size: f64) -> String {
    match size {
        40.0..=49.99 => "MD".to_string(), // Match size in the range [40.0, 49.99]
        50.0..=f64::MAX => "LG".to_string(), // Match size 50.0 and above
        _ => "SM".to_string(), // Match anything below 40.0
    }
}

fn pallet_wide(size: f64) -> String {
    match size {
        0.0..=48.0 => "1 WIDE".to_string(),
        48.01..=86.0 => "2 WIDE".to_string(),
        86.01..=f64::MAX => "3 WIDE".to_string(),
        _ => "".to_string(),
    }
}

fn pallet_class(stack: u32, size: f64) -> String {
    format!("{}STACK-{}", stack, pallet_size(size))
}

fn filter_primary_length(items: &Vec<Item>) -> Vec<Item> {
    let mut seen_parts = HashSet::new(); // To track part numbers we have encountered
    let mut duplicate_parts = HashSet::new(); // To track duplicate part numbers
    // Remove items where primary_length_in is 0.0
    let mut i = items.clone();
    i.retain(|item| item.primary_length_in != Some(0.0));
    i.retain(|item| item.primary_length_in != Some(1.0) && item.primary_height_in != Some(1.0) && item.primary_width_in != Some(1.0));

    for item in i.iter_mut() {
        if item.secondary_length_in == Some(0.0) {
            item.secondary_length_in = item.primary_length_in;
        }
        if item.secondary_width_in == Some(0.0) {
            item.secondary_width_in = item.primary_width_in;
        }
        if item.secondary_height_in == Some(0.0) {
            item.secondary_height_in = Some(5.0);
        }
    }

    i.retain(|item| {
        if seen_parts.contains(&item.part) {
            duplicate_parts.insert(item.part.clone());
            false // Remove duplicates
        } else {
            seen_parts.insert(item.part.clone());
            true // Keep the first occurrence
        }
    });

    for item in i.iter_mut() {
        if duplicate_parts.contains(&item.part) {
            item.plant.clear(); // Clear the plant field for first occurrence of a duplicate
        }
    }

    // Return the modified vector
    i.clone()  // Clone to return the filtered vector while keeping original reference mutable
}



fn create_csv(data: &Vec<Item>) -> String {
    let mut csv_string = String::new();
    let filtered = filter_primary_length(data);
    for item in filtered {
        let _ = writeln!(
            csv_string,
            "{},{},GM,,{},,A,{},{},{},1,1,1,1,1,{},{},{},{},{},{},{},{},{},{},",
            item.part, item.part_name,
            pallet_class(item.warehouse_stack.unwrap_or(0), item.secondary_length_in.unwrap_or(0.0)),
            plant_code(&item.plant), pallet_wide(item.secondary_width_in.unwrap_or(0.0)),
            pallet_size(item.secondary_length_in.unwrap_or(0.0)), item.std_pk,
            item.primary_length_in.unwrap_or(0.0), item.primary_width_in.unwrap_or(0.0), item.primary_height_in.unwrap_or(0.0),
            item.primary_container_weight_lbs.unwrap_or(0.0), item.pieces_per_pallet.unwrap_or(0), item.secondary_length_in.unwrap_or(0.0),
            item.secondary_width_in.unwrap_or(0.0), item.secondary_height_in.unwrap_or(0.0),
            item.pallet_weight.unwrap_or(0.0)
        );
    }
    csv_string
}

#[function_component(Upload)]
pub fn upload() -> Html {
    let file = use_state(|| None);
    let file_name = use_state(|| None);
    let is_error = use_state(|| false);
    let msg = use_state(|| "".to_string());

    let on_file_change = {
        let file = file.clone();
        let file_name = file_name.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Some(files) = input.files() {
                let infile = files.get(0).unwrap();
                file_name.set(Some(infile.name()));
                file.set(Some(infile));
            }
        })
    };

    let on_submit = {
        let file = file.clone();
        let file_name = file_name.clone();
        Callback::from(move |_| {
            if let Some(file) = (*file).clone() {
                let file_name = (*file_name).clone().unwrap();
                let reader = FileReader::new().unwrap();
                let reader_clone = reader.clone();
                let file_name_clone = file_name.clone(); // Clone here to avoid move

                let closure = Closure::wrap(Box::new(move |_e: Event| {
                    let array_buffer = reader_clone.result().unwrap();
                    let uint8_array = js_sys::Uint8Array::new(&array_buffer);
                    let bytes = uint8_array.to_vec();

                    let file_name_clone = file_name_clone.clone(); // Clone here to avoid move
                    spawn_local(async move {
                        let client = Client::new();
                        let part = reqwest::multipart::Part::bytes(bytes)
                            .file_name(file_name_clone.clone());
                        let form = reqwest::multipart::Form::new()
                            .part("file", part);

                        let response = client
                            .post("http://192.168.4.172:8888/upload")
                            .multipart(form)
                            .send()
                            .await;

                        match response {
                            Ok(res) => log!(format!("Response: {:?}", res.text().await.unwrap())),
                            Err(err) => log!(format!("Error: {:?}", err)),
                        }
                    });
                }) as Box<dyn FnMut(_)>);

                reader.set_onloadend(Some(closure.as_ref().unchecked_ref()));
                closure.forget(); // Keeps the closure alive
                reader.read_as_array_buffer(&file).unwrap();
            }
        })
    };

    html! {
        <div>
            <h1>{ "Upload CSV File" }</h1>
            <input type="file" accept=".csv" onchange={on_file_change} />
            <button onclick={on_submit}>{ "Upload" }</button>
            {
                if let Some(ref content) = *file_name {
                    html! { <p>{ content }</p> }
                } else {
                    html! { <p>{ "No file chosen" }</p> }
                }
            }
            <PartsUpload />
            <br />
            <Gmap />
            <br />
            <FixParts />
        </div>
    }
}

#[function_component(PartsUpload)]
pub fn parts_upload() -> Html {
    let data = use_state(|| vec![]);

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

                        let mut new_data = Vec::<Item>::new();
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

    let download_csv = {
        let data = data.clone();
        Callback::from(move |_: MouseEvent| {
            let csv_string = create_csv(&data);
            let filename = "ITEMS.csv";
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
        <div>
            <h1>{ "Download Parts CSV" }</h1>
            <input type="file" accept=".csv" onchange={on_file_change} />
            <button onclick={download_csv}>{ "Download CSV" }</button>
            <ul>
                { for data.iter().map(|item| html! {
                    <li>{ format!("Part: {}, Part Name: {}, Plant: {}, Country: {}, Std Pk: {}, Primary Length: {}",
                        item.part, item.part_name, item.plant, item.country, item.std_pk, item.primary_length_in.unwrap_or(0.0)) }</li>
                }) }
            </ul>
        </div>
    }
}