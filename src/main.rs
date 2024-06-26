mod requests;
mod state;
mod trucks;
//mod server;
mod load_details;
use std::rc::Rc;
use requests::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use reqwest::Client;
use gloo::console::log;
use web_sys::{js_sys, wasm_bindgen::{closure::Closure, JsCast}, HtmlInputElement, MessageEvent, WebSocket};
use trucks::Trucks;
use state::*;
use load_details::*;


#[function_component(App)]
fn app() -> Html {
    
    let app_state = use_reducer(|| AppState::default());
    let app_state_rc = Rc::new(app_state.clone());
    let app_st = Rc::new(app_state.clone());
    {
        let app_state_rc = app_state_rc.clone();
        use_effect_with((), move |_| {
            let ws = WebSocket::new("ws://192.168.4.97:9001").unwrap();
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

            app_st.dispatch(AppStateAction::ConnectWebSocket(ws));

            || ()
        });
    }

    if let Some(user) = &app_state.user {
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
                {
                    match app_state.current_view.as_str() {
                        "landing" => html! { <Trucks /> },
                        "load_details" => html! { <LoadDetails />},
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
    let username = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());
    let app_state = use_context::<AppStateContext>().expect("no app state found");

    let on_login = {
        let username = username.clone();
        let password = password.clone();
        let app_state = app_state.clone();

        Callback::from(move |_| {
            let username = username.clone();
            let password = password.clone();
            let app_state = app_state.clone();

            spawn_local(async move {
                let client = Client::new();
                let request = LoginRequest {
                    username: (*username).clone(),
                    password: (*password).clone(),
                };

                match client.post("http://192.168.4.97:8000/login")
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

    html! {
        <div style="text-align: center;">
            <h1>{ "Login" }</h1>
            <input type="text" placeholder="Username" value={(*username).clone()} oninput={on_username_input} />
            <input type="password" placeholder="Password" value={(*password).clone()} oninput={on_password_input} />
            <button onclick={on_login}>{ "Login" }</button>
            <UserInfo />
        </div>
    }
}

#[function_component(UserInfo)]
fn user_info() -> Html {
    let app_state = use_context::<AppStateContext>().expect("no app state found");

        if let Some(user) = &app_state.user {
            html! {
                <div>
                    <p>{ format!("Logged in as: {}", user.username) }</p>
                    <p>{ format!("Role: {}", user.role) }</p>
                    <p>{ format!("Token: {}", user.token) }</p>
                </div>
            }
        } else {
            html! {
                <p>{ "Not logged in" }</p>
            }
        }

}




fn main() {
    yew::Renderer::<App>::new().render();
}