use std::sync::Mutex;
use tauri::Manager;
use tauri::State;

mod background;
mod log_system;
mod network_info;
mod sign_in;
mod store;

use log_system::{create_log_buffer, get_logs, init_tracing, LogBuffer};
use network_info::check_network_status;
use sign_in::try_login;
use store::credentials::{load_credentials, read_credentials, save_credentials};

pub struct AppState {
    pub auto_login_paused: Mutex<bool>,
    pub ignore_ssid: Mutex<bool>,
    pub logs: LogBuffer,
    pub quitting: Mutex<bool>,
}

#[tauri::command]
fn get_auto_login_paused(state: State<'_, AppState>) -> bool {
    *state.auto_login_paused.lock().unwrap()
}

#[tauri::command]
fn set_auto_login_paused(paused: bool, app_handle: tauri::AppHandle, state: State<'_, AppState>) {
    *state.auto_login_paused.lock().unwrap() = paused;
    rebuild_tray_menu(&app_handle, paused);
    log_system::add_log(if paused {
        "自动登录已暂停"
    } else {
        "自动登录已恢复"
    });
}

#[tauri::command]
fn get_ignore_ssid(state: State<'_, AppState>) -> bool {
    *state.ignore_ssid.lock().unwrap()
}

#[tauri::command]
fn set_ignore_ssid(ignore: bool, app_handle: tauri::AppHandle, state: State<'_, AppState>) {
    *state.ignore_ssid.lock().unwrap() = ignore;
    store::settings::save_ignore_ssid(&app_handle, ignore);
}

fn show_or_create_settings(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
    } else {
        tauri::WebviewWindowBuilder::new(app, "main", tauri::WebviewUrl::App(Default::default()))
            .title("天科大校园网自动登录")
            .inner_size(520.0, 640.0)
            .resizable(true)
            .center()
            .build()
            .unwrap();
    }
}

fn rebuild_tray_menu(app_handle: &tauri::AppHandle, paused: bool) {
    use tauri::menu::{CheckMenuItemBuilder, MenuBuilder, MenuItemBuilder};

    let trigger_item = MenuItemBuilder::with_id("trigger_login", "触发登录")
        .build(app_handle)
        .unwrap();
    let pause_item = CheckMenuItemBuilder::with_id("pause", "暂停自动登录")
        .checked(paused)
        .build(app_handle)
        .unwrap();
    let settings_item = MenuItemBuilder::with_id("settings", "设置...")
        .build(app_handle)
        .unwrap();
    let sep1 = tauri::menu::PredefinedMenuItem::separator(app_handle).unwrap();
    let quit_item = MenuItemBuilder::with_id("quit", "退出")
        .build(app_handle)
        .unwrap();

    let menu = MenuBuilder::new(app_handle)
        .item(&trigger_item)
        .item(&pause_item)
        .item(&settings_item)
        .item(&sep1)
        .item(&quit_item)
        .build()
        .unwrap();

    if let Some(tray) = app_handle.tray_by_id("main-tray") {
        let _ = tray.set_menu(Some(menu));
    }
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
            get_logs,
            get_auto_login_paused,
            set_auto_login_paused,
            get_ignore_ssid,
            set_ignore_ssid,
        ])
        .setup(|app| {
            // Restore persisted settings
            let stored = store::settings::load_settings(app.handle());
            let state = app.state::<AppState>();
            *state.ignore_ssid.lock().unwrap() = stored.ignore_ssid;

            use tauri::menu::{CheckMenuItemBuilder, MenuBuilder, MenuItemBuilder};
            use tauri::tray::TrayIconBuilder;
            #[cfg(target_os = "macos")]
            {
                use tauri::ActivationPolicy;
                app.set_activation_policy(ActivationPolicy::Accessory);
            }

            let trigger_item = MenuItemBuilder::with_id("trigger_login", "触发登录").build(app)?;
            let pause_item = CheckMenuItemBuilder::with_id("pause", "暂停自动登录")
                .checked(false)
                .build(app)?;
            let settings_item = MenuItemBuilder::with_id("settings", "设置...").build(app)?;
            let sep1 = tauri::menu::PredefinedMenuItem::separator(app)?;
            let quit_item = MenuItemBuilder::with_id("quit", "退出").build(app)?;

            let menu = MenuBuilder::new(app)
                .item(&trigger_item)
                .item(&pause_item)
                .item(&settings_item)
                .item(&sep1)
                .item(&quit_item)
                .build()?;

            let _tray = TrayIconBuilder::with_id("main-tray")
                .icon(app.default_window_icon().unwrap().clone())
                .icon_as_template(cfg!(target_os = "macos"))
                .tooltip("天科大校园网自动登录")
                .menu(&menu)
                .on_menu_event(|app, event| {
                    match event.id().as_ref() {
                        "trigger_login" => {
                            let handle = app.clone();
                            tauri::async_runtime::spawn(async move {
                                let creds = read_credentials(&handle);
                                match creds {
                                    Some(ref c) if !c.username.is_empty() && !c.password.is_empty() => {
                                        log_system::add_log("菜单触发登录");
                                        background::try_auto_login(c).await;
                                    }
                                    _ => {
                                        log_system::add_log("触发登录失败: 未保存凭据");
                                    }
                                }
                            });
                        }
                        "pause" => {
                            let state = app.state::<AppState>();
                            let mut paused = state.auto_login_paused.lock().unwrap();
                            *paused = !*paused;
                            log_system::add_log(if *paused {
                                "自动登录已暂停"
                            } else {
                                "自动登录已恢复"
                            });
                        }
                        "settings" => {
                            show_or_create_settings(app);
                        }
                        "quit" => {
                            let state = app.state::<AppState>();
                            *state.quitting.lock().unwrap() = true;
                            app.exit(0);
                        }
                        _ => {}
                    }
                })
                .build(app)?;

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
