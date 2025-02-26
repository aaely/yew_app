use yew::prelude::*;
use crate::state::*;

#[function_component(FloatingIcon)]
pub fn floating_icon() -> Html {

    let app_state = use_context::<AppStateContext>().expect("not state found");

    let inline_style = "
        position: fixed;
        bottom: 2rem;
        right: 2rem;
        width: 56px;
        height: 56px;
        border-radius: 50%;
        background-color: #2196F3;
        color: white;
        display: flex;
        align-items: center;
        justify-content: center;
        box-shadow: 0 2px 5px rgba(0,0,0,0.3);
        cursor: pointer;
        transition: all 0.3s ease;
        z-index: 1000;
    ";

    let on_click = {
        let app_state = app_state.clone();
        Callback::from(move |view: String| {
            app_state.dispatch(AppStateAction::SetCurrentView(view));
        })
    };

    html! {
        <div style={inline_style} onclick={on_click.clone().reform(move |_| "new_shipment".to_string())}>
            {"+"}
        </div>
    }
}