use serde::{Deserialize, Serialize};
use std::fs;
use tauri::Manager;

const SETTINGS_FILE: &str = "settings.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub ignore_ssid: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self { ignore_ssid: false }
    }
}

fn settings_path(app_handle: &tauri::AppHandle) -> Option<std::path::PathBuf> {
    let dir = app_handle.path().app_data_dir().ok()?;
    Some(dir.join(SETTINGS_FILE))
}

pub(crate) fn load_settings(app_handle: &tauri::AppHandle) -> Settings {
    let path = match settings_path(app_handle) {
        Some(p) => p,
        None => return Settings::default(),
    };
    if !path.exists() {
        return Settings::default();
    }
    let json = match fs::read_to_string(&path) {
        Ok(s) => s,
        Err(_) => return Settings::default(),
    };
    serde_json::from_str(&json).unwrap_or_default()
}

pub(crate) fn save_ignore_ssid(app_handle: &tauri::AppHandle, ignore: bool) {
    let path = match settings_path(app_handle) {
        Some(p) => p,
        None => return,
    };
    let settings = Settings { ignore_ssid: ignore };
    if let Ok(json) = serde_json::to_string(&settings) {
        let _ = fs::create_dir_all(path.parent().unwrap());
        let _ = fs::write(&path, json);
    }
}
