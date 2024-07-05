use web_sys::window;
use serde::{Deserialize, Serialize};
use serde_json::{self, Error};
use crate::state::User;

const USER_KEY: &str = "user";

pub fn save_user_to_local_storage(user: &User) -> Result<(), Error> {
    let window = window().unwrap();
    let storage = window.local_storage().unwrap().unwrap();
    let user_json = serde_json::to_string(user)?;
    storage.set_item(USER_KEY, &user_json).unwrap();
    Ok(())
}

pub fn load_user_from_local_storage() -> Option<User> {
    let window = window().unwrap();
    let storage = window.local_storage().unwrap().unwrap();
    storage.get_item(USER_KEY).ok().flatten().and_then(|user_json| {
        serde_json::from_str(&user_json).ok()
    })
}