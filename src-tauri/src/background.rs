use tauri::Manager;

use crate::network_info::{check_network_status, needs_login};
use crate::sign_in;
use crate::store::credentials::{read_credentials, Credentials};
use crate::AppState;

pub(crate) async fn try_auto_login(creds: &Credentials) -> bool {
    match sign_in::try_login(
        creds.username.clone(),
        creds.password.clone(),
        creds.network_type.clone(),
    )
    .await
    {
        Ok(result) => result.success,
        Err(_) => false,
    }
}

pub(crate) fn start_background_loop(app_handle: tauri::AppHandle) {
    tauri::async_runtime::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(20));
        interval.tick().await;
        loop {
            interval.tick().await;

            let state = app_handle.state::<AppState>();
            if *state.auto_login_paused.lock().unwrap() {
                continue;
            }

            let ignore_ssid = *state.ignore_ssid.lock().unwrap();
            let status = check_network_status();

            if !ignore_ssid && !status.is_tust_network {
                continue;
            }
            if ignore_ssid && status.local_ipv4.is_none() {
                continue;
            }

            if !needs_login().await {
                continue;
            }

            tracing::info!(frontend = true, message = "检测到需要登录");

            let creds = match read_credentials(&app_handle) {
                Some(c) => c,
                None => {
                    tracing::info!(frontend = true, message = "自动登录跳过: 未保存凭据");
                    continue;
                }
            };

            if creds.username.is_empty() || creds.password.is_empty() {
                continue;
            }

            if try_auto_login(&creds).await {
                tracing::info!(frontend = true, message = "自动登录成功");
            }
        }
    });
}
