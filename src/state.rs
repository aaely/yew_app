use std::{error::Error, rc::Rc};
use serde::{Deserialize, Serialize};
use web_sys::WebSocket;
use yew::prelude::*;
use crate::{models::*, recent_local_storage::*, user_local_storage::*};
use gloo::console::log;

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
    pub current_shipment: Option<Shipment>,
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
            current_shipment: None,
        }
    }
}

impl User {
    pub fn is_authorized(&self) -> bool {
        if self.role == "admin".to_string() || self.role == "write".to_string() {
            true
        } else {
            false
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
    fn set_shipment_trailer(&mut self, msg: &str) -> Result<(), Box<dyn Error>> {
        let shipment_message: TrailerArrivalMessage = serde_json::from_str(msg)?;
        for shipment in self.shipments.iter_mut() {
            if shipment.LoadId == shipment_message.LoadId {
                shipment.ArrivalTime = shipment_message.ArrivalTime;
                shipment.TrailerNum = shipment_message.TrailerNum;
                break;
            }
        }
        Ok(())
    }
    fn set_shipment_door(&mut self, msg: &str) -> Result<(), Box<dyn Error>> {
        let shipment_message: SetShipmentDoorMessage = serde_json::from_str(msg)?;
        for shipment in self.shipments.iter_mut() {
            if shipment.LoadId == shipment_message.LoadId {
                shipment.Door = shipment_message.Door;
                break;
            }
        }
        Ok(())
    }
    fn shipment_depart(&mut self, msg: &str) -> Result<(), Box<dyn Error>> {
        let shipment_message: SetShipmentDepartMessage = serde_json::from_str(msg)?;
        for shipment in self.shipments.iter_mut() {
            if shipment.LoadId == shipment_message.LoadId {
                shipment.DepartTime = shipment_message.DepartTime;
                shipment.Status = "COMPLETE".to_string();
                break;
            }
        }
        Ok(())
    }
    fn shipment_pick_start(&mut self, msg: &str) -> Result<(), Box<dyn Error>> {
        let shipment_message: PickStartMessage = serde_json::from_str(msg)?;
        for shipment in self.shipments.iter_mut() {
            if shipment.LoadId == shipment_message.LoadId {
                shipment.Picker = shipment_message.Picker;
                shipment.PickStartTime = shipment_message.StartTime;
                shipment.Status = "PICKING".to_string();
                break;
            }
        }
        Ok(())
    }
    fn shipment_pick_finish(&mut self, msg: &str) -> Result<(), Box<dyn Error>> {
        let shipment_message: PickFinishMessage = serde_json::from_str(msg)?;
        for shipment in self.shipments.iter_mut() {
            if shipment.LoadId == shipment_message.LoadId {
                shipment.PickFinishTime = shipment_message.FinishTime;
                shipment.Status = "VERIFICATION".to_string();
                break;
            }
        }
        Ok(())
    }
    fn shipment_start_loading(&mut self, msg: &str) -> Result<(), Box<dyn Error>> {
        let shipment_message: StartLoadingMessage = serde_json::from_str(msg)?;
        for shipment in self.shipments.iter_mut() {
            if shipment.LoadId == shipment_message.LoadId {
                shipment.Status = "LOADING".to_string();
                break;
            }
        }
        Ok(())
    }
    fn shipment_hold(&mut self, msg: &str) -> Result<(), Box<dyn Error>> {
        let shipment_message: StartLoadingMessage = serde_json::from_str(msg)?;
        for shipment in self.shipments.iter_mut() {
            if shipment.LoadId == shipment_message.LoadId {
                shipment.IsHold = !shipment.IsHold;
                break;
            }
        }
        Ok(())
    }
    fn verified_by(&mut self, msg: &str) -> Result<(), Box<dyn Error>> {
        let shipment_message: VerifiedByMessage = serde_json::from_str(msg)?;
        for shipment in self.shipments.iter_mut() {
            if shipment.LoadId == shipment_message.LoadId {
                shipment.VerifiedBy = shipment_message.VerifiedBy;
                shipment.Status = "READY TO LOAD".to_string();
                break;
            }
        }
        Ok(())
    }
    fn new_shipment(&mut self, msg: &str) -> Result<(), Box<dyn Error>> {
        let shipment_message: Shipment = serde_json::from_str(msg)?;
        self.shipments.insert(0, shipment_message);
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
    HandleNewShipment(serde_json::Value),
    HandlePickFinish(serde_json::Value),
    HandleShipmentDoor(serde_json::Value),
    HandleShipmentTrailer(serde_json::Value),
    HandleShipmentDepart(serde_json::Value),
    HandlePickStart(serde_json::Value),
    HandleShipmentHold(serde_json::Value),
    HandleVerifiedBy(serde_json::Value),
    HandleShipmentLoading(serde_json::Value),
    SetShipments(Vec<Shipment>),
    SetCurrentShipment(Shipment),
}

impl Reducible for AppState {
    type Action = AppStateAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            AppStateAction::SetUser(user) => {
                save_user_to_local_storage(&user).expect("Failed to save user to local storage");
                Rc::new(Self { user: Some(user), ..(*self).clone() })
            },
            AppStateAction::SetCurrentShipment(shipment) => Rc::new(Self { current_shipment: Some(shipment), ..(*self).clone() }),
            AppStateAction::ClearUser => Rc::new(Self { user: None, ..(*self).clone() }),
            AppStateAction::SetCurrentTrailer(trailer) => Rc::new(Self { current_trailer: Some(trailer), ..(*self).clone() }),
            AppStateAction::ClearCurrentTrailer => Rc::new(Self { current_trailer: None, ..(*self).clone() }),
            AppStateAction::ClearRecentlyScheduled => Rc::new(Self { recent_trailers: vec![], ..(*self).clone() }),
            AppStateAction::SetCurrentView(view) => {
                let _ = save_view_to_session_storage(&view);
                Rc::new(Self { current_view: view, ..(*self).clone() })
            },
            AppStateAction::HandleShipmentDoor(data) => {
                log!(format!("Handling shipment door: {:?}", data));
                let mut new_state = (*self).clone();
                if let Some(message) = data.get("message").and_then(|v| v.as_str()) {
                    let _ = new_state.set_shipment_door(message);
                }
                Rc::new(new_state)
            },
            AppStateAction::HandleShipmentHold(data) => {
                log!(format!("Handling shipment hold: {:?}", data));
                let mut new_state = (*self).clone();
                if let Some(message) = data.get("message").and_then(|v| v.as_str()) {
                    let _ = new_state.shipment_hold(message);
                }
                Rc::new(new_state)
            },
            AppStateAction::HandleNewShipment(data) => {
                log!(format!("Handling new shipment: {:?}", data));
                let mut new_state = (*self).clone();
                if let Some(message) = data.get("message").and_then(|v| v.as_str()) {
                    let _ = new_state.new_shipment(message);
                }
                Rc::new(new_state)
            },
            AppStateAction::HandleShipmentLoading(data) => {
                log!(format!("Handling loading: {:?}", data));
                let mut new_state = (*self).clone();
                if let Some(message) = data.get("message").and_then(|v| v.as_str()) {
                    let _ = new_state.shipment_start_loading(message);
                }
                Rc::new(new_state)
            },
            AppStateAction::HandlePickFinish(data) => {
                log!(format!("Handling pick finish: {:?}", data));
                let mut new_state = (*self).clone();
                if let Some(message) = data.get("message").and_then(|v| v.as_str()) {
                    let _ = new_state.shipment_pick_finish(message);
                }
                Rc::new(new_state)
            },
            AppStateAction::HandleVerifiedBy(data) => {
                log!(format!("Handling verified by: {:?}", data));
                let mut new_state = (*self).clone();
                if let Some(message) = data.get("message").and_then(|v| v.as_str()) {
                    let _ = new_state.verified_by(message);
                }
                Rc::new(new_state)
            },
            AppStateAction::HandlePickStart(data) => {
                log!(format!("Handling pick start: {:?}", data));
                let mut new_state = (*self).clone();
                if let Some(message) = data.get("message").and_then(|v| v.as_str()) {
                    let _ = new_state.shipment_pick_start(message);
                }
                Rc::new(new_state)
            },
            AppStateAction::HandleShipmentDepart(data) => {
                log!(format!("Handling complete shipment: {:?}", data));
                let mut new_state = (*self).clone();
                if let Some(message) = data.get("message").and_then(|v| v.as_str()) {
                    let _ = new_state.shipment_depart(message);
                }
                Rc::new(new_state)
            },
            AppStateAction::HandleShipmentTrailer(data) => {
                log!(format!("Handling shipment trailer: {:?}", data));
                let mut new_state = (*self).clone();
                if let Some(message) = data.get("message").and_then(|v| v.as_str()) {
                    let _ = new_state.set_shipment_trailer(message);
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