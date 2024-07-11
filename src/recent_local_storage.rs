use web_sys::window;
use serde::{Deserialize, Serialize};
use serde_json::{self, Error};
use crate::{state::User, RecentTrailers};

const RECENT_KEY: &str = "recent";
const VIEW_KEY: &str = "view";

pub fn save_recent_to_local_storage(recent: &Vec<RecentTrailers>) -> Result<(), Error> {
    let window = window().unwrap();
    let storage = window.local_storage().unwrap().unwrap();
    let recent_json = serde_json::to_string(recent)?;
    storage.set_item(RECENT_KEY, &recent_json).unwrap();
    Ok(())
}

pub fn load_recent_from_local_storage() -> Option<Vec<RecentTrailers>> {
    let window = window().unwrap();
    let storage = window.local_storage().unwrap().unwrap();
    storage.get_item(RECENT_KEY).ok().flatten().and_then(|recent_json| {
        serde_json::from_str(&recent_json).ok()
    })
}

pub fn save_view_to_session_storage(view: &String) -> Result<(), Error> {
    let window = window().unwrap();
    let storage = window.session_storage().unwrap().unwrap();
    let view_json = serde_json::to_string(view)?;
    storage.set_item(VIEW_KEY, &view_json).unwrap();
    Ok(())
}

pub fn load_view_from_session_storage() -> Option<String> {
    let window = window().unwrap();
    let storage = window.session_storage().unwrap().unwrap();
    storage.get_item(VIEW_KEY).ok().flatten().and_then(|view_json| {
        serde_json::from_str(&view_json).ok()
    })
}