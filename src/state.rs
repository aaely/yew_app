use std::{error::Error, rc::Rc};
use serde::{Deserialize, Serialize};
use web_sys::WebSocket;
use yew::prelude::*;
use crate::{models::*, recent_local_storage::*, user_local_storage::*};
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
    pub last_view: String,
    pub recent_trailers:Vec<RecentTrailers>,
    pub shipments: Vec<Shipment>,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            user: load_user_from_local_storage(),
            current_trailer: None,
            current_view: load_view_from_session_storage().unwrap_or("landing".to_string()),
            last_view: "".to_string(),
            ws: None,
            messages: vec![],
            trailers: vec![],
            recent_trailers: load_recent_from_local_storage().unwrap_or_default(),
            shipments: vec![],
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
    fn arrived(&mut self, msg: &str) -> Result<(), Box<dyn Error>> {
        let arrival_message: ArrivalMessage = serde_json::from_str(msg)?;
        for trailer in self.trailers.iter_mut() {
            if trailer.TrailerID == arrival_message.TrailerID {
                trailer.Schedule.ArrivalTime = arrival_message.ArrivalTime;
                break;
            }
        }
        Ok(())
    }
    fn scheduled(&mut self, msg: &str) -> Result<(), Box<dyn Error>> {
        let schedule_message: SetScheduleRequest = serde_json::from_str(msg)?;
        for trailer in self.trailers.iter_mut() {
            if trailer.TrailerID == schedule_message.TrailerID {
                trailer.Schedule.ScheduleDate = schedule_message.ScheduleDate;
                trailer.Schedule.RequestDate = schedule_message.RequestDate;
                trailer.Schedule.CarrierCode = schedule_message.CarrierCode;
                trailer.Schedule.ScheduleTime = schedule_message.ScheduleTime;
                trailer.Schedule.DoorNumber = schedule_message.Door;
                trailer.Schedule.ContactEmail = schedule_message.ContactEmail;
                trailer.Schedule.LastFreeDate = schedule_message.LastFreeDate;
                break;
            }
        }
        Ok(())
    }
    fn update_shipment(&mut self, msg: &str) -> Result<(), Box<dyn Error>> {
        let shipment_message: Shipment = serde_json::from_str(msg)?;
        for shipment in self.shipments.iter_mut() {
            if shipment.LoadId == shipment_message.LoadId {
                shipment.ScheduleDate = shipment_message.ScheduleDate;
                shipment.ScheduleTime = shipment_message.ScheduleTime;
                shipment.ArrivalTime = shipment_message.ArrivalTime;
                shipment.DepartTime = shipment_message.DepartTime;
                shipment.Dock = shipment_message.Dock;
                shipment.Door = shipment_message.Door;
                shipment.LoadNum = shipment_message.LoadNum;
                shipment.Status = shipment_message.Status;
                shipment.Picker = shipment_message.Picker;
                shipment.PickStartTime = shipment_message.PickStartTime;
                shipment.VerifiedBy = shipment_message.VerifiedBy;
                shipment.TrailerNum = shipment_message.TrailerNum;
                break;
            }
        }
        Ok(())
    }
    fn recent(&mut self, trailer: RecentTrailers) -> Result<(), Box<dyn Error>> {
        let mut found = false;
        let t = trailer.clone();
        for trl in self.recent_trailers.iter_mut() {
            if trl.trailer_id == t.trailer_id.clone() {
                trl.trailer_id = t.trailer_id.clone();
                trl.date = t.date.clone();
                trl.time = t.time.clone();
                trl.scac = t.scac.clone();
                found = true;
            }
        }
        if !found {
            self.recent_trailers.push(trailer);
            Ok(())
        } else {
            Ok(())
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
    HandleHotTrailer(serde_json::Value),
    HandleScheduleTrailer(serde_json::Value),
    HandleSetDoor(serde_json::Value),
    HandleTrailerArrived(serde_json::Value),
    SetTrailers(Vec<TrailerResponse>),
    SetLastView(String),
    AddToRecentlyScheduled(RecentTrailers),
    ClearRecentlyScheduled,
    HandleUpdateShipment(serde_json::Value),
    SetShipments(Vec<Shipment>),
}

impl Reducible for AppState {
    type Action = AppStateAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            AppStateAction::SetUser(user) => {
                save_user_to_local_storage(&user).expect("Failed to save user to local storage");
                Rc::new(Self { user: Some(user), ..(*self).clone() })
            },
            AppStateAction::ClearUser => Rc::new(Self { user: None, ..(*self).clone() }),
            AppStateAction::SetCurrentTrailer(trailer) => Rc::new(Self { current_trailer: Some(trailer), ..(*self).clone() }),
            AppStateAction::ClearCurrentTrailer => Rc::new(Self { current_trailer: None, ..(*self).clone() }),
            AppStateAction::ClearRecentlyScheduled => Rc::new(Self { recent_trailers: vec![], ..(*self).clone() }),
            AppStateAction::SetCurrentView(view) => {
                let _ = save_view_to_session_storage(&view);
                Rc::new(Self { current_view: view, ..(*self).clone() })
            },
            AppStateAction::HandleUpdateShipment(data) => {
                log!(format!("Handling updated shipment: {:?}", data));
                let mut new_state = (*self).clone();
                if let Some(message) = data.get("message").and_then(|v| v.as_str()) {
                    let _ = new_state.update_shipment(message);
                }
                Rc::new(new_state)
            },
            AppStateAction::SetShipments(shipments) => Rc::new(Self { shipments, ..(*self).clone() }),
            AppStateAction::SetLastView(view) => Rc::new(Self { last_view: view, ..(*self).clone() }),
            AppStateAction::ConnectWebSocket(ws) => Rc::new(Self { ws: Some(ws), ..(*self).clone() }),
            AppStateAction::DisconnectWebSocket => Rc::new(Self { ws: None, ..(*self).clone() }),
            AppStateAction::SetTrailers(trailers) => Rc::new(Self { trailers, ..(*self).clone()}),
            AppStateAction::AddToRecentlyScheduled(trailer) => {
                let mut new_state = (*self).clone();
                let _ = new_state.recent(trailer);
                let _ = save_recent_to_local_storage(&new_state.recent_trailers);
                Rc::new(new_state)
            }
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
                let mut new_state = (*self).clone();
                if let Some(message) = data.get("message").and_then(|v| v.as_str()) {
                    let _ = new_state.scheduled(message);
                }
                Rc::new(new_state)
            },
            AppStateAction::HandleSetDoor(data) => {
                // Handle set_door data
                log!(format!("Handling set_door: {:?}", data));
                self
            },
            AppStateAction::HandleTrailerArrived(data) => {
                // Handle trailer_arrived data
                log!(format!("Handling trailer_arrived: {:?}", data));
                let mut new_state = (*self).clone();
                if let Some(message) = data.get("message").and_then(|v| v.as_str()) {
                    let _ = new_state.arrived(message);
                }
                Rc::new(new_state)
            },
            _ => todo!(),
        }
    }
}

pub type AppStateContext = UseReducerHandle<AppState>;