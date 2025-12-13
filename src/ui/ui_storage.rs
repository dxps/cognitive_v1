use log::{debug, error};
use serde::{Deserialize, Serialize};

use crate::domain::model::UserAccount;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct UiStorage {
    pub current_user: Option<UserAccount>,

    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub localstorage: Option<web_sys::Storage>,
}

impl UiStorage {
    /// Browser's localstorage key.
    const LS_KEY: &'static str = "tmc";

    pub fn new() -> Result<Self, String> {
        let window = web_sys::window().expect("No global `window` exists!");
        if let Ok(Some(storage)) = window.local_storage() {
            let state = UiStorage {
                current_user: None,
                localstorage: Some(storage),
            };
            Ok(state)
        } else {
            error!(">>> [State::new] Error: No browser's localstorage found!");
            Err("No localstorage found".into())
        }
    }

    pub fn load_from_localstorage() -> Result<Self, String> {
        let mut state = UiStorage::new()?;
        if let Ok(Some(value)) = state.localstorage.as_ref().unwrap().get(Self::LS_KEY) {
            debug!(">>> [State::load_from_localstorage] Loaded value={:?}", value);
            state.current_user = Some(serde_json::from_str(&value).unwrap());
        } else {
            error!(">>> [State::load_from_localstorage] No value exists in localstorage.");
        }
        Ok(state)
    }

    pub fn save_to_localstorage(&mut self) {
        //
        if self.current_user.is_some() {
            if self.localstorage.is_none() {
                self.init_localstorage().unwrap();
            }
            self.localstorage
                .as_ref()
                .unwrap()
                .set_item(Self::LS_KEY, &serde_json::to_string(&self.current_user).unwrap())
                .unwrap();
            debug!(">>> [save_to_localstorage] Saved {:?} to localstorage.", self.current_user);
        } else {
            self.localstorage.as_ref().unwrap().remove_item(Self::LS_KEY).unwrap();
            debug!(">>> [save_to_localstorage] Removed {:?} key from localstorage.", Self::LS_KEY);
        }
    }

    fn init_localstorage(&mut self) -> Result<(), String> {
        let window = web_sys::window().expect("No global `window` exists!");
        if let Ok(Some(storage)) = window.local_storage() {
            self.localstorage = Some(storage);
            Ok(())
        } else {
            error!(">>> [State::new] Error: No browser's localstorage found!");
            Err("No localstorage found".into())
        }
    }
}

impl std::fmt::Display for UiStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
