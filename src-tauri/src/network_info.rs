use serde::{Deserialize, Serialize};
use std::time::Duration;

pub(crate) use crate::platform::get_wifi_ssid;
pub use crate::platform::{get_local_ipv4, get_local_ipv6};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStatus {
    pub wifi_ssid: Option<String>,
    pub local_ipv4: Option<String>,
    pub local_ipv6: Option<String>,
    pub is_tust_network: bool,
}

pub async fn needs_login() -> bool {
    let client = match reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
    {
        Ok(c) => c,
        Err(_) => return true,
    };

    match client
        .get("http://connectivitycheck.gstatic.com/generate_204")
        .timeout(Duration::from_secs(5))
        .send()
        .await
    {
        Ok(resp) => resp.status().as_u16() != 204,
        Err(_) => true,
    }
}

pub fn check_network_status() -> NetworkStatus {
    let wifi_ssid = get_wifi_ssid();
    let local_ipv4 = get_local_ipv4();
    let local_ipv6 = get_local_ipv6();

    let is_tust = wifi_ssid
        .as_ref()
        .map(|s| s.to_uppercase().starts_with("TUST") || s.to_uppercase().starts_with("CU_TUST"))
        .unwrap_or(false)
        && local_ipv4
            .as_ref()
            .map(|s| s.starts_with("10."))
            .unwrap_or(false);

    NetworkStatus {
        wifi_ssid,
        local_ipv4,
        local_ipv6,
        is_tust_network: is_tust,
    }
}
