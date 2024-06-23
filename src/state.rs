use std::rc::Rc;

use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew::context::ContextHandle;
use crate::requests::*;

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct User {
    pub username: String,
    pub role: String,
    pub token: String,
    pub refresh_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AppState {
    pub user: Option<User>,
    pub current_trailer: Option<TrailerResponse>,
}

pub enum AppStateAction {
    SetUser(User),
    ClearUser,
    SetCurrentTrailer(TrailerResponse),
    ClearCurrentTrailer
}

impl Reducible for AppState {
    type Action = AppStateAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            AppStateAction::SetUser(user) => Rc::new(Self { user: Some(user), ..(*self).clone() }),
            AppStateAction::ClearUser => Rc::new(Self { user: None, ..(*self).clone() }),
            AppStateAction::SetCurrentTrailer(trailer) => Rc::new(Self { current_trailer: Some(trailer), ..(*self).clone() }),
            AppStateAction::ClearCurrentTrailer => Rc::new(Self { current_trailer: None, ..(*self).clone() }),
        }
    }
}

pub type AppStateContext = UseReducerHandle<AppState>;