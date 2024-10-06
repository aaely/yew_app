use wasm_bindgen_futures::spawn_local;
use web_sys::{wasm_bindgen::{prelude::*, JsCast}, Event, FileReader, HtmlInputElement, js_sys, window};
use yew::prelude::*;
use gloo::console::log;
use std::{fmt::Write, rc::Rc};
use crate::{models::GmapItem, ItemCompare, ScaleItem};
use csv::ReaderBuilder;
use std::collections::HashMap;

fn merge_gmap_items(mut items: &Vec<GmapItem>) -> HashMap<String, GmapItem> {
    // Create a HashMap to store unique items by part (or other unique identifier)
    let mut item_map: HashMap<String, GmapItem> = HashMap::new();

    // Iterate over all items
    for item in items.into_iter() {
        // Check if an item with the same part already exists
        if let Some(existing_item) = item_map.get_mut(&item.part) {
            // If the plant in the new item is not the same as the existing item, update the plant and asl_bank
            if !existing_item.plant.contains(&item.plant) {
                // Add the new plant to the plant field
                existing_item.plant.push_str(&format!(", {}", item.plant));
                // Add the asl_bank value
                existing_item.asl_qty += item.asl_qty;
            }
        } else {
            // If no duplicate, insert the new item into the map
            item_map.insert(item.part.clone(), item.clone());
        }
    }

    // Convert the HashMap back into a Vec<GmapItem>
    item_map
}

fn merge_gmap_vec(mut items: &Vec<GmapItem>) -> Vec<GmapItem> {
    // Create a HashMap to store unique items by part (or other unique identifier)
    let mut item_map: HashMap<String, GmapItem> = HashMap::new();

    // Iterate over all items
    for item in items.into_iter() {
        // Check if an item with the same part already exists
        if let Some(existing_item) = item_map.get_mut(&item.part) {
            // If the plant in the new item is not the same as the existing item, update the plant and asl_bank
            if !existing_item.plant.contains(&item.plant) {
                // Add the new plant to the plant field
                existing_item.plant.push_str(&format!(", {}", item.plant));
                // Add the asl_bank value
                existing_item.asl_qty += item.asl_qty;
            }
        } else {
            // If no duplicate, insert the new item into the map
            item_map.insert(item.part.clone(), item.clone());
        }
    }

    // Convert the HashMap back into a Vec<GmapItem>
    item_map.into_values().collect()
}

fn merge_scale_items(mut items: &Vec<ScaleItem>) -> Vec<ScaleItem> {
    let mut item_map: HashMap<String, ScaleItem> = HashMap::new();

    for item in items.into_iter() {
        if let Some(existing_item) = item_map.get_mut(&item.item) {
            existing_item.quantity += item.quantity;
        } else {
            item_map.insert(item.item.clone(), item.clone());
        }
    }
    item_map.into_values().collect()
}

fn compare_items(gmap_items: &Vec<GmapItem>, scale_items: &Vec<ScaleItem>) -> Vec<ItemCompare> {
    let mut item_compare: Vec<ItemCompare> = vec![];
    let gmap = merge_gmap_items(gmap_items);
    let scale = merge_scale_items(&scale_items);
    for item in scale {
        if let Some(asl) = gmap.get(&item.item.clone()) {
            let item = ItemCompare {
                part: item.item.clone(),
                scale_quantity: item.quantity.clone(),
                asl_quantity: asl.asl_qty,
                dif: item.quantity.clone() - asl.asl_qty,
                in_transit: asl.in_transit_asl_to_plant.unwrap_or(0)
            };
            item_compare.push(item);
        } else {
            // Handle the case where the value is None (optional, based on your logic)
            println!("No matching ASL found for item: {}", item.item);
        }
    }
    item_compare
}

fn create_csv(gmap_data: &Vec<GmapItem>, scale_data: &Vec<ScaleItem>) -> String {
    let mut csv_string = String::new();
    let _ = writeln!(csv_string, "Part Number, Scale, GMAP, Dif, In Transit");
    let compared = compare_items(gmap_data, scale_data);
    for item in compared {
        let _ = writeln!(csv_string, "{},{},{},{}, {}", item.part, item.scale_quantity, item.asl_quantity, item.scale_quantity - item.asl_quantity, item.in_transit);
    }
    csv_string
}

#[function_component(Gmap)]
pub fn gmap() -> Html {
    let data = use_state(|| vec![]);
    let data2 = use_state(|| vec![]);

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

                        let mut new_data = Vec::<GmapItem>::new();
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

    let on_file_change2 = {
        let data2 = data2.clone();
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Some(files) = input.files() {
                let infile = files.get(0).unwrap();
                let file = Rc::new(infile);
                
                let onload = {
                    let data2 = data2.clone();
                    Callback::from(move |file_content: String| {
                        let mut rdr = ReaderBuilder::new()
                            .has_headers(true) // Skip the headers row
                            .from_reader(file_content.as_bytes()); // Reading as bytes

                        let mut new_data = Vec::<ScaleItem>::new();
                        for result in rdr.deserialize() {
                            match result {
                                Ok(record) => new_data.push(record), // Correct type deserialization
                                Err(err) => log!(format!("Error deserializing row: {:?}", err)),
                            }
                        }

                        data2.set(new_data); // Store parsed data in state
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

    let merge = {
        let data = data.clone();
        Callback::from(move |_: MouseEvent| {
            let new_data = merge_gmap_vec(&data);
            data.set(new_data);
        })
    };

    let merge_scale = {
        let data2 = data2.clone();
        Callback::from(move |_: MouseEvent| {
            let new_data = merge_scale_items(&data2);
            data2.set(new_data);
        })
    };

    let download_csv = {
        let data = data.clone();
        let data2 = data2.clone();
        Callback::from(move |_: MouseEvent| {
            let csv_string = create_csv(&data, &data2);
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
            <h1>{ "Item ASL CSV" }</h1>
            <input type="file" accept=".csv" onchange={on_file_change} />
            <button onclick={merge}>{ "Merge GMAP" }</button>
            <ul>
                { for data.iter().map(|item| html! {
                    <li>{ format!("Part: {}, Part Name: {}, Plant: {}, Qty: {}", 
                        item.part, item.part_name, item.plant, item.asl_qty)}</li>
                }) }
            </ul>
            <input type="file" accept=".csv" onchange={on_file_change2} />
            <button onclick={merge_scale}>{ "Merge Scale" }</button>
            <ul>
                { for data2.iter().map(|item| html! {
                    <li>{
                        format!(
                            "Part: {}, Plant: {}, Quantity: {}",
                            item.item, item.plant, item.quantity
                        )}</li>
                }) }
            </ul>
            <button onclick={download_csv}>{ "Download CSV" }</button>
        </div>
    }
}