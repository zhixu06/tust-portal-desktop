use serde::{Deserialize, Serialize};
use std::fs;
use tauri::Manager;

pub(crate) const SETTINGS_FILE: &str = "settings.json";

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

pub(crate) fn read_settings(app_handle: &tauri::AppHandle) -> Option<Settings> {
    let path = app_handle
        .path()
        .app_data_dir()
        .ok()?
        .join(SETTINGS_FILE);
    if !path.exists() {
        return None;
    }
    let json = fs::read_to_string(&path).ok()?;
    serde_json::from_str::<Settings>(&json).ok()
}

pub(crate) fn save_ignore_ssid(
    app_handle: &tauri::AppHandle,
    ignore: bool,
) -> Result<(), String> {
    let settings = Settings { ignore_ssid: ignore };
    let json = serde_json::to_string(&settings).map_err(|e| e.to_string())?;
    let path = app_handle
        .path()
        .app_data_dir()
        .map_err(|e: tauri::Error| e.to_string())?
        .join(SETTINGS_FILE);
    fs::create_dir_all(path.parent().unwrap()).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())?;
    tracing::info!(frontend = true, message = "设置已保存");
    Ok(())
}
