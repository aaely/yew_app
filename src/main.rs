mod requests;
mod state;
mod trucks;

use requests::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use reqwest::Client;
use gloo::console::log;
use web_sys::HtmlInputElement;
use trucks::{Trucks};
use state::*;


#[function_component(App)]
fn app() -> Html {
    let app_state = use_reducer(|| AppState::default());

    html! {
        <ContextProvider<AppStateContext> context={app_state}>
            <Login />
            <UserInfo />
            <Trucks />
        </ContextProvider<AppStateContext>>
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

                match client.post("http://192.168.4.92:8000/login")
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
        <div>
            <h1>{ "Login" }</h1>
            <input type="text" placeholder="Username" value={(*username).clone()} oninput={on_username_input} />
            <input type="password" placeholder="Password" value={(*password).clone()} oninput={on_password_input} />
            <button onclick={on_login}>{ "Login" }</button>
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