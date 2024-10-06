use wasm_bindgen_futures::spawn_local;
use web_sys::{wasm_bindgen::{prelude::*, JsCast}, Event, FileReader, HtmlInputElement, js_sys, window};
use yew::prelude::*;
use gloo::console::log;
use std::{fmt::Write, rc::Rc};
use crate::{models::ItemDetails, ItemMaster};
use csv::ReaderBuilder;
use std::collections::HashMap;

fn remove_brn(items: &Vec<ItemDetails>) -> Vec<ItemDetails> {
    let mut removed = items.clone();
    removed.retain(|item| {
        if item.ctn_qty == 1 && item.pal_qty == 1 {
            return false;
        }
        if item.ctn_qty == 0 {
            return false;
        }
        true
    });
    removed
}

fn map_items(items: &Vec<ItemDetails>) -> HashMap<String, ItemDetails> {
    let mut item_map: HashMap<String, ItemDetails> = HashMap::new();
    let filtered_items = remove_brn(items);
    for item in filtered_items {
        if let Some(existing_item) = item_map.get(&item.item) {
            log!(format!("Item {:?} already exists", existing_item));
        } else {
            item_map.insert(item.item.clone(), item.clone());
        }
    }
    item_map
}

fn compare_items(item_master: &Vec<ItemMaster>, item_list: &Vec<ItemDetails>) -> Vec<ItemMaster> {
    let item_map = map_items(item_list);
    let mut new_list = item_master.clone();
    new_list.retain(|item| {
        if item_map.contains_key(&item.part) {
            let itm = item_map.get(&item.part).unwrap();
            if itm.ctn_qty != item.std_pk || itm.pal_qty != item.pal_qty {
                true
            } else {
                false
            }
        } else {
            log!(format!("part: {} does not exist", item.part.clone()));
            false
        }
    });
    for item in new_list.iter_mut() {
        if let Some(existing_item) = item_map.get(&item.part) {
            item.std_pk = existing_item.ctn_qty;
            item.pal_qty = existing_item.pal_qty;
        } else {
            log!(format!("part: {} does not exist", item.part.clone()));
        }
    }
    new_list.clone()
}

fn create_csv(item_list: &Vec<ItemDetails>, item_master: &Vec<ItemMaster>) -> String {
    let mut csv_string = String::new();
    let new_list = compare_items(item_master, item_list);
    for item in new_list {
        let _ = writeln!(
            csv_string,
            "{},{},GM,,{},,A,{},{},{},1,1,1,1,1,{},{},{},{},{},{},{},{},{},{},",
            item.part, item.desc, item.class, item.location, item.wide, item.size,
            item.std_pk, item.pri_len, item.pri_wid, item.pri_hei, item.pri_wt,
            item.pal_qty, item.pal_len, item.pal_wid, item.pal_hei, item.pal_wt
        );
    }
    csv_string
}

#[function_component(FixParts)]
pub fn fix_parts() -> Html {
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

                        let mut new_data = Vec::<ItemDetails>::new();
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

                        let mut new_data = Vec::<ItemMaster>::new();
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
            <h1>{ "Restore Item Master" }</h1>
            <input type="file" accept=".csv" onchange={on_file_change} />
            <ul>
                { for data.iter().map(|item| html! {
                    <li>{ format!("Part: {}, Standard Pack: {}, Pallet Quantity: {}", 
                        item.item, item.ctn_qty, item.pal_qty)}</li>
                }) }
            </ul>
            <input type="file" accept=".csv" onchange={on_file_change2} />
            <ul>
                { for data2.iter().map(|item| html! {
                    <li>{
                        format!(
                            "Part: {}, Standard Pack: {}, Pallet Quantity: {}",
                            item.part, item.std_pk, item.pal_qty
                        )}</li>
                }) }
            </ul>
            <button onclick={download_csv}>{ "Download CSV" }</button>
        </div>
    }
}