pub mod crypto;
pub mod settings;
use uuid;

use std::sync::{Arc, Mutex};
pub type SettingsMutex = Arc<Mutex<settings::SettingsWrapper>>;

pub fn create_uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}