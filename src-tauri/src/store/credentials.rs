use serde::{Deserialize, Serialize};
use std::fs;
use tauri::Manager;

pub(crate) const CREDS_FILE: &str = "credentials.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
    #[serde(default = "default_network_type")]
    pub network_type: String,
}

fn default_network_type() -> String {
    "校园网".to_string()
}

pub(crate) fn read_credentials(app_handle: &tauri::AppHandle) -> Option<Credentials> {
    let path = app_handle
        .path()
        .app_data_dir()
        .ok()?
        .join(CREDS_FILE);
    if !path.exists() {
        return None;
    }
    let json = fs::read_to_string(&path).ok()?;
    serde_json::from_str::<Credentials>(&json).ok()
}
