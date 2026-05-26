use tauri::{
    menu::{CheckMenuItemBuilder, MenuBuilder, MenuItemBuilder},
    tray::TrayIconBuilder,
    AppHandle, Manager,
};

use crate::store::credentials::read_credentials;
use crate::{background, AppState};

fn bring_to_front(window: &tauri::WebviewWindow) {
    let _ = window.unminimize();
    let _ = window.show();
    let _ = window.set_always_on_top(true);
    let _ = window.set_focus();
    let _ = window.set_always_on_top(false);
}

fn show_or_create_settings(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        bring_to_front(&window);
    } else {
        tauri::WebviewWindowBuilder::new(
            app,
            "main",
            tauri::WebviewUrl::App("index.html#/settings".into()),
        )
        .title("天科大校园网自动登录")
        .inner_size(520.0, 400.0)
        .resizable(true)
        .center()
        .build()
        .unwrap();
    }
}

fn show_or_create_logs(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("logs") {
        bring_to_front(&window);
    } else {
        let w = tauri::WebviewWindowBuilder::new(
            app,
            "logs",
            tauri::WebviewUrl::App("index.html".into()),
        )
        .title("日志")
        .inner_size(520.0, 480.0)
        .resizable(true)
        .visible(false)
        .center()
        .build()
        .unwrap();
        w.eval("window.location.hash = '#/logs'").ok();
        w.show().ok();
    }
}

fn handle_menu_event(app: &AppHandle, event: tauri::menu::MenuEvent) {
    match event.id().as_ref() {
        "trigger_login" => {
            let handle = app.clone();
            tauri::async_runtime::spawn(async move {
                let creds = read_credentials(&handle);
                match creds {
                    Some(ref c) if !c.username.is_empty() && !c.password.is_empty() => {
                        tracing::info!(frontend = true, message = "菜单触发登录");
                        background::try_auto_login(c).await;
                    }
                    _ => {
                        tracing::info!(frontend = true, message = "触发登录失败: 未保存凭据");
                    }
                }
            });
        }
        "pause" => {
            let state = app.state::<AppState>();
            let mut paused = state.auto_login_paused.lock().unwrap();
            *paused = !*paused;
            let msg = if *paused {
                "自动登录已暂停"
            } else {
                "自动登录已恢复"
            };
            tracing::info!(frontend = true, message = msg);
        }
        "logs" => {
            show_or_create_logs(app);
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
}

pub(crate) fn build_tray(app: &mut tauri::App) -> tauri::Result<()> {
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);

    let trigger_item = MenuItemBuilder::with_id("trigger_login", "触发登录").build(app)?;
    let pause_item = CheckMenuItemBuilder::with_id("pause", "暂停自动登录")
        .checked(false)
        .build(app)?;
    let logs_item = MenuItemBuilder::with_id("logs", "日志").build(app)?;
    let settings_item = MenuItemBuilder::with_id("settings", "设置...").build(app)?;
    let sep1 = tauri::menu::PredefinedMenuItem::separator(app)?;
    let quit_item = MenuItemBuilder::with_id("quit", "退出").build(app)?;

    let menu = MenuBuilder::new(app)
        .item(&trigger_item)
        .item(&pause_item)
        .item(&logs_item)
        .item(&settings_item)
        .item(&sep1)
        .item(&quit_item)
        .build()?;

    TrayIconBuilder::with_id("main-tray")
        .icon(app.default_window_icon().unwrap().clone())
        .icon_as_template(true)
        .tooltip("天科大校园网自动登录")
        .menu(&menu)
        .on_menu_event(handle_menu_event)
        .build(app)?;

    Ok(())
}

pub(crate) fn rebuild_tray_menu(app_handle: &AppHandle, paused: bool) {
    let trigger_item = MenuItemBuilder::with_id("trigger_login", "触发登录")
        .build(app_handle)
        .unwrap();
    let pause_item = CheckMenuItemBuilder::with_id("pause", "暂停自动登录")
        .checked(paused)
        .build(app_handle)
        .unwrap();
    let logs_item = MenuItemBuilder::with_id("logs", "日志")
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
        .item(&logs_item)
        .item(&settings_item)
        .item(&sep1)
        .item(&quit_item)
        .build()
        .unwrap();

    if let Some(tray) = app_handle.tray_by_id("main-tray") {
        let _ = tray.set_menu(Some(menu));
    }
}
