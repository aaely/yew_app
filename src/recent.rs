use chrono::NaiveDate;
use web_sys::{js_sys, window};
use yew::prelude::*;
use crate::AppStateAction;
use crate::AppStateContext;
use crate::models::*;
use std::fmt::Write;

fn format_date(date_str: &str) -> String {
    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .expect("Failed to parse date");

    // Format the date as mm-dd-yyyy
    date.format("%m-%d-%Y").to_string()
}

fn create_csv(data: &Vec<RecentTrailers>) -> String {
    let mut csv_string = String::new();
    let _ = writeln!(csv_string, "Trailer, Scheduled Date, Scheduled Time, Carrier");
    for trailer in data {
        let _ = writeln!(csv_string, "{},{},{},{}", trailer.trailer_id, format_date(&trailer.date), trailer.time, trailer.scac);
    }
    csv_string
}

#[function_component(Recent)]
pub fn recent() -> Html {
    let app_state = use_context::<AppStateContext>().expect("no state available");

    let download_csv = {
        let data = app_state.recent_trailers.clone();
        Callback::from(move |_: MouseEvent| {
            let csv_string = create_csv(&data);
            let filename = "recent_trailers.csv";
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

    let clear = {
        let app_state = app_state.clone();
        Callback::from(move |_| {
            app_state.dispatch(AppStateAction::ClearRecentlyScheduled);
        })
    };

    html! {
        <div style="margin-top: 7vh; width: 70vw;">
           <h1 style="text-align: center;"> {"Recent Trailers"} </h1>
           <div style="
            text-align: center;
            width: 30vw;
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: space-evenly;
            margin-left: auto;
            margin-right: auto;
            margin-bottom: 3%">
           { if app_state.recent_trailers.len() > 0 {
                html! {
                    <a style="margin-top: 3%;" onclick={download_csv}>{"Download CSV"}</a>
                }
           } else {
            html! { <></>}
           }}
           </div>
           <table>
                <thead>
                    <tr>
                        <td>
                            {"Trailer"}
                        </td>
                        <td>
                            {"Date"}
                        </td>
                        <td>
                            {"Time"}
                        </td>
                        <td>
                            {"Carrier"}
                        </td>
                    </tr>
                </thead>
                <tbody>
                    { for app_state.recent_trailers.iter().map(|trailer|
                        html! {
                            <tr>
                                <td>
                                    {trailer.trailer_id.clone()}
                                </td>
                                <td>
                                    {trailer.date.clone()}
                                </td>
                                <td>
                                    {trailer.time.clone()}
                                </td>
                                <td>
                                    {trailer.scac.clone()}
                                </td>
                            </tr>
                        }
                    )}
                </tbody>
           </table>
           <div style="
            text-align: center;
            width: 30vw;
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: space-evenly;
            margin-left: auto;
            margin-right: auto;
            margin-bottom: 3%">
           { if app_state.recent_trailers.len() > 0 {
                html! {
                    <a style="margin-top: 3%; color=red;" onclick={clear}>{"Clear Recent"}</a>
                }
           } else {
            html! { <></>}
           }}
           </div>
        </div>
    }
}