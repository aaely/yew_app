use wasm_bindgen_futures::spawn_local;
use web_sys::{wasm_bindgen::{prelude::*, JsCast}, Event, FileReader, HtmlInputElement, js_sys};
use yew::prelude::*;
use reqwest::Client;
use gloo::console::log;

#[function_component(Upload)]
pub fn upload() -> Html {
    let file = use_state(|| None);
    let file_name = use_state(|| None);

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
                            .post("http://192.168.4.162:8888/upload")
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
        </div>
    }
}