use yew::prelude::*;
use crate::{AppStateAction, AppStateContext};

#[function_component(Nav)]
pub fn nav() -> Html {
    let app_state = use_context::<AppStateContext>().expect("no state found");

    let update_view = {
        let app_state = app_state.clone();
        Callback::from(move |view: String| {
            app_state.dispatch(AppStateAction::SetCurrentView(view));
        })
    };

    let logout = {
        let app_state = app_state.clone();
        Callback::from(move |_| {
            app_state.dispatch(AppStateAction::ClearUser);
        })
    };

    html! {
        <>
            <div style="
            display: flex;
            position: fixed;
            justify-content: space-around;
            align-content: center;
            width: 100vw;
            height: 7vh;
            background-color: #333;
            color: limegreen;
            flex-wrap: wrap;
            ">
                <div onclick={update_view.clone().reform(move |_| "landing".to_string())}>
                    <p>{"All Trailers"}</p>
                </div>
                <div onclick={update_view.clone().reform(move |_| "trailers_date_range".to_string())}>
                    <p>{"Date Range"}</p>
                </div>
                <div onclick={update_view.clone().reform(move |_| "todays_schedule".to_string())}>
                    <p>{"Today's Schedule"}</p>
                </div>
                <div onclick={logout}>
                    <p>{"Logout"}</p>
                </div>
            </div>
        </>
    }
}
