use std::sync::Mutex;
use tauri::Manager;
use tauri::State;

mod background;
mod js_bridge;
mod log_system;
mod network_info;
mod platform;
mod sign_in;
mod store;
mod tray;

use js_bridge::{
    check_network_status, get_auto_login_paused, get_ignore_ssid, get_logs, load_credentials,
    load_settings, save_credentials, set_auto_login_paused, set_ignore_ssid, try_login,
};
use log_system::{create_log_buffer, init_tracing, LogBuffer};
use store::settings::read_settings;

pub struct AppState {
    pub auto_login_paused: Mutex<bool>,
    pub ignore_ssid: Mutex<bool>,
    pub logs: LogBuffer,
    pub quitting: Mutex<bool>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let log_buffer = create_log_buffer();
    init_tracing(log_buffer.clone());

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            auto_login_paused: Mutex::new(false),
            ignore_ssid: Mutex::new(false),
            logs: log_buffer,
            quitting: Mutex::new(false),
        })
        .invoke_handler(tauri::generate_handler![
            try_login,
            check_network_status,
            save_credentials,
            load_credentials,
            load_settings,
            get_logs,
            get_auto_login_paused,
            set_auto_login_paused,
            get_ignore_ssid,
            set_ignore_ssid,
        ])
        .setup(|app| {
            // Restore persisted settings
            if let Some(stored) = read_settings(app.handle()) {
                let state = app.state::<AppState>();
                *state.ignore_ssid.lock().unwrap() = stored.ignore_ssid;
            }

            tray::build_tray(app)?;

            background::start_background_loop(app.handle().clone());

            // Keep the initial window hidden — shown on demand via tray
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.hide();
            }

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app, event| {
            if let tauri::RunEvent::ExitRequested { api, .. } = event {
                let state: State<'_, AppState> = app.state();
                if !*state.quitting.lock().unwrap() {
                    api.prevent_exit();
                }
            }
        });
}
