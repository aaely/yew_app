use std::rc::Rc;

use serde::{Deserialize, Serialize};
use web_sys::WebSocket;
use yew::prelude::*;
use crate::requests::*;
use gloo::console::log;

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct User {
    pub username: String,
    pub role: String,
    pub token: String,
    pub refresh_token: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct IncomingMessage {
    pub r#type: String,
    pub data: serde_json::Value,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AppState {
    pub user: Option<User>,
    pub current_trailer: Option<TrailerResponse>,
    pub current_view: String,
    pub ws: Option<WebSocket>,
    pub messages: Vec<String>,
    pub trailers: Vec<TrailerResponse>,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            user: None,
            current_trailer: None,
            current_view: "landing".to_string(),
            ws: None,
            messages: vec![],
            trailers: vec![],
        }
    }
}

impl AppState {
    pub fn send_ws_message(&self, message: &str) {
        if let Some(ws) = &self.ws {
            ws.send_with_str(message).unwrap();
        }
    }
    fn toggle_hot_trailer(&mut self, trailer_id: &str) {
        for trailer in self.trailers.iter_mut() {
            if trailer.TrailerID == trailer_id {
                trailer.Schedule.IsHot = !trailer.Schedule.IsHot;
                break;
            }
        }
    }
}

pub enum AppStateAction {
    SetUser(User),
    ClearUser,
    SetCurrentTrailer(TrailerResponse),
    ClearCurrentTrailer,
    SetCurrentView(String),
    ConnectWebSocket(WebSocket),
    DisconnectWebSocket,
    AddMessage(String),
    HandleHotTrailer(serde_json::Value),
    HandleScheduleTrailer(serde_json::Value),
    HandleSetDoor(serde_json::Value),
    HandleTrailerArrived(serde_json::Value),
    SetTrailers(Vec<TrailerResponse>),
}

impl Reducible for AppState {
    type Action = AppStateAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            AppStateAction::SetUser(user) => Rc::new(Self { user: Some(user), ..(*self).clone() }),
            AppStateAction::ClearUser => Rc::new(Self { user: None, ..(*self).clone() }),
            AppStateAction::SetCurrentTrailer(trailer) => Rc::new(Self { current_trailer: Some(trailer), ..(*self).clone() }),
            AppStateAction::ClearCurrentTrailer => Rc::new(Self { current_trailer: None, ..(*self).clone() }),
            AppStateAction::SetCurrentView(view) => Rc::new(Self { current_view: view, ..(*self).clone() }),
            AppStateAction::ConnectWebSocket(ws) => Rc::new(Self { ws: Some(ws), ..(*self).clone() }),
            AppStateAction::DisconnectWebSocket => Rc::new(Self { ws: None, ..(*self).clone() }),
            AppStateAction::SetTrailers(trailers) => Rc::new(Self { trailers, ..(*self).clone()}),
            AppStateAction::AddMessage(msg) => {
                let mut new_state = (*self).clone();
                new_state.messages.push(msg);
                Rc::new(new_state)
            },
            AppStateAction::HandleHotTrailer(data) => {
                // Handle hot_trailer data
                log!(format!("Handling hot_trailer: {:?}", data));
                let mut new_state = (*self).clone();
                if let Some(message) = data.get("message").and_then(|v| v.as_str()) {
                    new_state.toggle_hot_trailer(message);
                }
                Rc::new(new_state)
            },
            AppStateAction::HandleScheduleTrailer(data) => {
                // Handle schedule_trailer data
                log!(format!("Handling schedule_trailer: {:?}", data));
                self
            },
            AppStateAction::HandleSetDoor(data) => {
                // Handle set_door data
                log!(format!("Handling set_door: {:?}", data));
                self
            },
            AppStateAction::HandleTrailerArrived(data) => {
                // Handle trailer_arrived data
                log!(format!("Handling trailer_arrived: {:?}", data));
                self
            },
        }
    }
}

pub type AppStateContext = UseReducerHandle<AppState>;