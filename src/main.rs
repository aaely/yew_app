mod models;
mod state;
mod trucks;
mod daily_csv;
mod user_local_storage;
mod trailers_date_range;
mod recent_local_storage;
mod upload;
mod gmap;
mod new_shipment;
mod verified_by;
mod float_button;
mod set_door;
mod set_picker;
mod trailer_arrive;
mod shipments;
//mod server;
mod load_details;
mod fix_parts;
mod recent;
mod nav;
mod todays_schedule;
mod edit_trailer;
use std::rc::Rc;
use models::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use reqwest::Client;
use gloo::console::log;
use web_sys::{js_sys::{self}, wasm_bindgen::{self, closure::Closure, prelude::wasm_bindgen, JsCast}, HtmlInputElement, KeyboardEvent, MessageEvent, WebSocket};
use trucks::Trucks;
use state::*;
use load_details::*;
use nav::Nav;
use todays_schedule::TodaysSchedule;
use edit_trailer::EditTrailer;
use trailers_date_range::TrailersDateRange;
use recent::Recent;
use upload::Upload;
use set_picker::SetPicker;
use trailer_arrive::SetTrailer;
use set_door::SetDoor;
use shipments::Shipments;
use new_shipment::NewShipment;
use verified_by::VerifiedBy;

#[wasm_bindgen]
extern "C" {
    fn saveCredentials(username: &str, password: &str) -> js_sys::Promise;
}

#[function_component(App)]
fn app() -> Html {
    
    let app_state = use_reducer(|| AppState::default());
    let app_state_rc = Rc::new(app_state.clone());
    let app_st = Rc::new(app_state.clone());

    {
        let app_state_rc = app_state_rc.clone();
        use_effect_with((), move |_| {
            let ws = WebSocket::new("ws://localhost:9001").unwrap();
            let app_state_rc = app_state_rc.clone();
            log!(format!("{:?}", ws.clone()));

            let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
                if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                    let incoming_message: IncomingMessage = serde_json::from_str(&txt.as_string().unwrap()).unwrap();
                    match incoming_message.r#type.as_str() {
                        "hot_trailer" => {
                            app_state_rc.dispatch(AppStateAction::HandleHotTrailer(incoming_message.data));
                        }
                        "schedule_trailer" => {
                            app_state_rc.dispatch(AppStateAction::HandleScheduleTrailer(incoming_message.data));
                        }
                        "set_door" => {
                            app_state_rc.dispatch(AppStateAction::HandleSetDoor(incoming_message.data));
                        }
                        "trailer_arrived" => {
                            app_state_rc.dispatch(AppStateAction::HandleTrailerArrived(incoming_message.data));
                        }
                        "set_shipment_trailer" => {
                            app_state_rc.dispatch(AppStateAction::HandleShipmentTrailer(incoming_message.data));
                        }
                        "set_shipment_door" => {
                            app_state_rc.dispatch(AppStateAction::HandleShipmentDoor(incoming_message.data));
                        }
                        "start_shipment_pick" => {
                            app_state_rc.dispatch(AppStateAction::HandlePickStart(incoming_message.data));
                        }
                        "finish_shipment_pick" => {
                            app_state_rc.dispatch(AppStateAction::HandlePickFinish(incoming_message.data));
                        }
                        "shipment_loading" => {
                            app_state_rc.dispatch(AppStateAction::HandleShipmentTrailer(incoming_message.data));
                        }
                        "shipment_trailer_arrival" => {
                            app_state_rc.dispatch(AppStateAction::HandleShipmentTrailer(incoming_message.data));
                        }
                        "new_shipment" => {
                            app_state_rc.dispatch(AppStateAction::HandleNewShipment(incoming_message.data));
                        }
                        "shipment_depart" => {
                            app_state_rc.dispatch(AppStateAction::HandleShipmentDepart(incoming_message.data));
                        }
                        "shipment_start_loading" => {
                            app_state_rc.dispatch(AppStateAction::HandleShipmentLoading(incoming_message.data));
                        }
                        "verified_by" => {
                            app_state_rc.dispatch(AppStateAction::HandleVerifiedBy(incoming_message.data));
                        }
                        _ => {
                            log!(format!("Unknown event type: {:?}", incoming_message.r#type));
                        }
                    }
                }
            }) as Box<dyn FnMut(MessageEvent)>);
            ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
            onmessage_callback.forget();

            let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
                log!(&format!("Error: {:?}", e));
            }) as Box<dyn FnMut(ErrorEvent)>);
            ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
            onerror_callback.forget();
            let app_st_clone = app_st.clone();
            app_st.dispatch(AppStateAction::ConnectWebSocket(ws));

            move || {
                app_st_clone.dispatch(AppStateAction::DisconnectWebSocket);
            }
        });
    }

    if let Some(_user) = &app_state.user {
        html! {
            <ContextProvider<AppStateContext> context={app_state.clone()}>
                <Nav />
                <div style="
                display: flex;
                flex-direction: column;
                align-items: center;
                justify-content: space-evenly;
                height: 100vh;
                width: 100vw;
                ">
                {
                    match app_state.current_view.as_str() {
                        "landing" => html! { <Trucks /> },
                        "load_details" => html! { <LoadDetails />},
                        "todays_schedule" => html! { <TodaysSchedule /> },
                        "edit_trailer" => html! { <EditTrailer /> },
                        "trailers_date_range" => html! { <TrailersDateRange /> },
                        "recent" => html! { <Recent /> },
                        "upload" => html! { <Upload /> },
                        "shipments" => html! { <Shipments /> },
                        "set_picker" => html! { <SetPicker /> },
                        "set_trailer" => html! { <SetTrailer /> },
                        "new_shipment" => html! { <NewShipment /> },
                        "set_door" => html! { <SetDoor /> },
                        "verified_by" => html! { <VerifiedBy /> },
                        _ => html! { <p>{ "Page not found" }</p> },
                    }
                }
                </div>
            </ContextProvider<AppStateContext>>
        }
    } else {
        html! {
            <ContextProvider<AppStateContext> context={app_state.clone()}>
                <div style="
                display: flex;
                flex-direction: column;
                align-items: center;
                justify-content: space-evenly;
                height: 100vh;
                width: 100vw;
                margin-top: 7vh;">
                    <Login />
                </div>
            </ContextProvider<AppStateContext>>
        }
    }
}

#[function_component(Login)]
fn login() -> Html {
    let local_view = use_state(|| "login".to_string());
    let username = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());
    let app_state = use_context::<AppStateContext>().expect("no app state found");

    let on_login = {
        let username = username.clone();
        let password = password.clone();
        let app_state = app_state.clone();

        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            let username = username.clone();
            let password = password.clone();
            let app_state = app_state.clone();

            spawn_local(async move {
                let client = Client::new();
                let request = LoginRequest {
                    username: (*username).clone(),
                    password: (*password).clone(),
                };

                match client.post("http://localhost:8000/login")
                    .json(&request)
                    .send()
                    .await {
                    Ok(resp) => {
                        match resp.json::<LoginResponse>().await {
                            Ok(login_response) => {
                                let user = User {
                                    username: login_response.user.username,
                                    role: login_response.user.role,
                                    token: login_response.token,
                                    refresh_token: login_response.refresh_token,
                                };
                                app_state.dispatch(AppStateAction::SetUser(user));
                                app_state.dispatch(AppStateAction::SetCurrentView("landing".to_string()));
                                unsafe { let _promise = saveCredentials(&username, &password)
                                    .then(&Closure::once(|_result| {
                                        log!(format!("Credentials stored successfully!"));
                                    }))
                                    .catch(&Closure::once(|err| {
                                        log!(format!("Failed to store credentials: {:?}", err));
                                    }));}
                            },
                            Err(error) => log!(format!("Failed to parse JSON: {:?}", error)),
                        }
                    },
                    Err(error) => log!(format!("Failed to login: {:?}", error)),
                }
            });
        })
    };

    let on_key_press = {
        let username = username.clone();
        let password = password.clone();
        let local_view = local_view.clone();
        let app_state = app_state.clone();

        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                if *local_view == "login" {
                    let username = username.clone();
                    let password = password.clone();
                    let app_state = app_state.clone();
                    spawn_local(async move {
                        let client = Client::new();
                        let request = LoginRequest {
                            username: (*username).clone(),
                            password: (*password).clone(),
                        };

                        match client.post("http://localhost:8000/login")
                            .json(&request)
                            .send()
                            .await {
                            Ok(resp) => {
                                match resp.json::<LoginResponse>().await {
                                    Ok(login_response) => {
                                        let user = User {
                                            username: login_response.user.username,
                                            role: login_response.user.role,
                                            token: login_response.token,
                                            refresh_token: login_response.refresh_token,
                                        };
                                        app_state.dispatch(AppStateAction::SetUser(user));
                                        app_state.dispatch(AppStateAction::SetCurrentView("landing".to_string()));
                                    },
                                    Err(error) => log!(format!("Failed to parse JSON: {:?}", error)),
                                }
                            },
                            Err(error) => log!(format!("Failed to parse JSON: {:?}", error)),
                        }
                    });
                } else {
                    let username = username.clone();
                    let password = password.clone();
                    let local_view = local_view.clone();
                    spawn_local(async move {
                        let client = Client::new();
                        let request = LoginRequest {
                            username: (*username).clone(),
                            password: (*password).clone(),
                        };

                        match client.post("http://localhost:8000/register")
                            .json(&request)
                            .send()
                            .await {
                            Ok(resp) => {
                                match resp.json::<String>().await {
                                    Ok(_registration_response) => {
                                        local_view.set("login".to_string());
                                    },
                                    Err(error) => log!(format!("Failed to parse JSON: {:?}", error)),
                                }
                            },
                            Err(error) => log!(format!("Failed to parse JSON: {:?}", error)),
                        }
                    });
                }
            }
        })
    };

    let on_register = {
        let username = username.clone();
        let password = password.clone();
        let app_state = app_state.clone();
        let local_view = local_view.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            let username = username.clone();
            let password = password.clone();
            let app_state = app_state.clone();
            let local_view = local_view.clone();
            spawn_local(async move {
                let client = Client::new();
                let request = LoginRequest {
                    username: (*username).clone(),
                    password: (*password).clone(),
                };

                match client.post("http://localhost:8000/register")
                    .json(&request)
                    .send()
                    .await {
                    Ok(resp) => {
                        match resp.json::<String>().await {
                            Ok(_registration_response) => {
                                local_view.set("login".to_string());
                            },
                            Err(error) => log!(format!("Failed to parse JSON: {:?}", error)),
                        }
                    },
                    Err(error) => log!(format!("Failed to login: {:?}", error)),
                }
            });
        })
    };

    let on_username_input = {
        let username = username.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            username.set(input.value());
        })
    };

    let on_password_input = {
        let password = password.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            password.set(input.value());
        })
    };

    let set_register = {
        let local_view = local_view.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            local_view.set("register".to_string());
        })
    };

    let set_login = {
        let local_view = local_view.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            local_view.set("login".to_string());
        })
    };

    html! {
        <div style="text-align: center;">
            { match local_view.as_str() {
                "register" => html! {
                    <>
                        <h1>{ "Register" }</h1>
                        <input style="text-align: center;" type="text" placeholder="Username" autocomplete="username" value={(*username).clone()} oninput={on_username_input} onkeypress={on_key_press.clone()} />
                        <input style="text-align: center;" type="password" placeholder="Password" autocomplete="password" value={(*password).clone()} oninput={on_password_input} onkeypress={on_key_press.clone()} />
                        <div style="margin: 3%; display: flex; width: 30vw; flex-direction: row; justify-content: space-evenly;">
                            <button type="submit" style="background-color: green; color: white; padding: 14px 20px; border: none; cursor: pointer; border-radius: 4px;"  onclick={on_register}>{ "Register" }</button>
                            <button style="background-color: blue; color: white; padding: 14px 20px; border: none; cursor: pointer; border-radius: 4px;" onclick={set_login}>{ "Login" }</button>
                        </div>
                    </>
                },
                "login" => html! {
                    <>
                        <h1>{ "Login" }</h1>
                        <input style="text-align: center;" type="text" placeholder="Username" autocomplete="username" value={(*username).clone()} oninput={on_username_input} onkeypress={on_key_press.clone()} />
                        <input style="text-align: center;" type="password" placeholder="Password" autocomplete="password" value={(*password).clone()} oninput={on_password_input} onkeypress={on_key_press.clone()} />
                        <div style="margin: 3%; display: flex; width: 30vw; flex-direction: row; justify-content: space-evenly;">
                            <button type="submit" style="background-color: green; color: white; padding: 14px 20px; border: none; cursor: pointer; border-radius: 4px;" onclick={on_login}>{ "Login" }</button>
                            <button style="background-color: blue; color: white; padding: 14px 20px; border: none; cursor: pointer; border-radius: 4px;" onclick={set_register}>{ "Register" }</button>
                        </div>
                    </>
                },
                _ => html! {
                    <>
                    {"Not Found"}
                    </>
                }
            }}
        </div>
    }
}



fn main() {
    yew::Renderer::<App>::new().render();
}