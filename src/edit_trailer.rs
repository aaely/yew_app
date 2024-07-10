use serde_json::json;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use reqwest::Client;
use gloo::console::log;
use crate::{models::*, state::AppStateContext, AppStateAction};
use chrono::prelude::*;

fn format_date() -> String {
    let local = Local::now();
    let year = local.year();
    let month = local.month();
    let day = local.day();
    format!("{}/{}/{}", month, day, year)
}

#[function_component(EditTrailer)]
pub fn edit_trailer() -> Html {
    let app_state = use_context::<AppStateContext>().expect("no state found");
    let trailer = app_state.current_trailer.as_ref().unwrap().clone();
    let form = use_state(|| MyFormData {
        door: trailer.Schedule.DoorNumber,
        contact_email: trailer.Schedule.ContactEmail,
        schedule_date: trailer.Schedule.ScheduleDate,
        schedule_time: trailer.Schedule.ScheduleTime,
        last_free_date: trailer.Schedule.LastFreeDate,
        scac: trailer.Schedule.CarrierCode,
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
                    "contact_email" => form_data.contact_email = value,
                    "schedule_date" => form_data.schedule_date = value,
                    "schedule_time" => form_data.schedule_time = value,
                    "last_free_date" => form_data.last_free_date = value,
                    "scac" => form_data.scac = value,
                    _ => (),
                }
                form_data
            });
        })
    };

    let schedule_trailer = {
        let app_state = app_state.clone();
        let form = form.clone();
        Callback::from(move |_| {
            let app_state = app_state.clone();
            let form = form.clone();
            let date = format_date();
            spawn_local(async move {
                let client = Client::new();
                if let Some(trailer) = &app_state.current_trailer {
                    if let Some(user) = &app_state.user {
                        let request = SetScheduleRequest {
                            TrailerID: trailer.TrailerID.clone(),
                            ScheduleDate: form.schedule_date.clone(),
                            RequestDate: date.clone(),
                            CarrierCode: form.scac.clone(),
                            ScheduleTime: form.schedule_time.clone(),
                            LastFreeDate: form.last_free_date.clone(),
                            ContactEmail: form.contact_email.clone(),
                            Door: form.door.clone(),
                        };
                        let recent = RecentTrailers {
                            trailer_id: trailer.TrailerID.clone(),
                            date: form.schedule_date.clone(),
                            time: form.schedule_time.clone(),
                            scac: form.scac.clone(),
                        };
                        match client.post("http://192.168.4.112:8000/api/set_schedule")
                            .header("Authorization", format!("Bearer {}", user.token))
                            .json(&request)
                            .send()
                            .await {
                                Ok(resp) => {
                                    match resp.json::<Vec<TrailerSchedule>>().await {
                                        Ok(_trailer_response) => {
                                            let msg = SetScheduleRequest {
                                                TrailerID: trailer.TrailerID.clone(),
                                                ScheduleDate: form.schedule_date.clone(),
                                                RequestDate: date,
                                                CarrierCode: form.scac.clone(),
                                                ScheduleTime: form.schedule_time.clone(),
                                                LastFreeDate: form.last_free_date.clone(),
                                                ContactEmail: form.contact_email.clone(),
                                                Door: form.door.clone(),
                                            };
                                            let json_string = serde_json::to_string(&msg).unwrap();
                                            let message = json!({
                                                "type": "schedule_trailer",
                                                "data": {
                                                    "message": json_string
                                                }
                                            }).to_string();
                                            app_state.send_ws_message(&message);
                                            app_state.dispatch(AppStateAction::AddToRecentlyScheduled(recent));
                                            app_state.dispatch(AppStateAction::SetCurrentView("landing".to_string()));
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
                }
            });
        })
    };

    html!{
        <div style="text-align: center;">
            <h1>{ "Edit Trailer: " }{trailer.TrailerID.clone()}</h1>
            <div>
                <label for="scac">{"SCAC:"}</label>
                <input style="text-align: center;" id="scac" type="text" value={form.scac.clone()} oninput={on_change.clone()} />
            </div>
            <div>
                <label for="last_free_date">{"Last Free Date:"}</label>
                <input style="text-align: center;" id="last_free_date" type="date" value={form.last_free_date.clone()} oninput={on_change.clone()} />
            </div>
            <div>
                <label for="schedule_date">{"Schedule Date:"}</label>
                <input style="text-align: center;" id="schedule_date" type="date" value={form.schedule_date.clone()} oninput={on_change.clone()} />
            </div>
            <div>
                <label for="schedule_time">{"Schedule Time:"}</label>
                <input style="text-align: center;" id="schedule_time" type="text" value={form.schedule_time.clone()} oninput={on_change.clone()} />
            </div>
            <div>
                <label for="contact_email">{"Email:"}</label>
                <input style="text-align: center;" id="contact_email" type="text" value={form.contact_email.clone()} oninput={on_change.clone()} />
            </div>
            <div>
                <label for="door">{"Door:"}</label>
                <input style="text-align: center;" id="door" type="text" value={form.door.clone()} oninput={on_change.clone()} />
            </div>
            <button style="background-color: green; color: white; padding: 14px 20px; border: none; cursor: pointer; border-radius: 4px;" onclick={schedule_trailer}>{"Set Details"}</button>
        </div>
    }
}