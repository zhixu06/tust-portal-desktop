use serde::{Deserialize, Serialize};
use std::process::Command;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
use std::time::Duration;

const CREATE_NO_WINDOW: u32 = 0x08000000;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStatus {
    pub wifi_ssid: Option<String>,
    pub local_ipv4: Option<String>,
    pub local_ipv6: Option<String>,
    pub is_tust_network: bool,
}

// -- Wi-Fi SSID ----------------------------------------------------------------

#[cfg(target_os = "macos")]
pub(crate) fn get_wifi_ssid() -> Option<String> {
    if let Some(ssid) = get_ssid_via_ipconfig("en0") {
        let trimmed = ssid.trim().to_string();
        if !trimmed.is_empty() && trimmed != "<redacted>" {
            return Some(trimmed);
        }
    }
    let port_name = get_wifi_port_name()?;
    if !is_interface_active(&port_name) {
        return None;
    }
    get_current_ssid(&port_name)
}

#[cfg(target_os = "windows")]
pub(crate) fn get_wifi_ssid() -> Option<String> {
    let mut cmd = Command::new("netsh");
    cmd.args(["wlan", "show", "interfaces"]);
    #[cfg(target_os = "windows")]
    {
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
    let output = cmd.output().ok()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        let trimmed = line.trim();
        if trimmed.to_lowercase().starts_with("ssid") {
            let ssid = trimmed.splitn(2, ':').nth(1)?.trim().to_string();
            if !ssid.is_empty() {
                return Some(ssid);
            }
        }
    }
    None
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub(crate) fn get_wifi_ssid() -> Option<String> {
    None
}

// -- macOS Wi-Fi helpers -------------------------------------------------------

#[cfg(target_os = "macos")]
fn get_ssid_via_ipconfig(interface: &str) -> Option<String> {
    let output = Command::new("ipconfig")
        .args(["getsummary", interface])
        .output()
        .ok()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if line.contains("SSID") {
            return line.split(" : ").nth(1).map(|s| s.to_string());
        }
    }
    None
}

#[cfg(target_os = "macos")]
fn get_wifi_port_name() -> Option<String> {
    let output = Command::new("networksetup")
        .args(["-listallhardwareports"])
        .output()
        .ok()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut lines = stdout.lines();
    while let Some(line) = lines.next() {
        if line.contains("Wi-Fi") || line.contains("AirPort") {
            if let Some(next) = lines.next() {
                return next.split_whitespace().last().map(|s| s.to_string());
            }
        }
    }
    None
}

#[cfg(target_os = "macos")]
fn is_interface_active(port_name: &str) -> bool {
    if let Ok(output) = Command::new("ipconfig")
        .args(["getsummary", port_name])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        return !stdout.lines().any(|l| l.trim() == "Active : FALSE");
    }
    false
}

#[cfg(target_os = "macos")]
fn get_current_ssid(port_name: &str) -> Option<String> {
    let output = Command::new("networksetup")
        .args(["-listpreferredwirelessnetworks", port_name])
        .output()
        .ok()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.lines().nth(1).map(|s| s.trim().to_string())
}

// -- Local IP addresses --------------------------------------------------------

pub(crate) fn get_local_ipv4() -> Option<String> {
    local_ip_address::local_ip().ok().map(|ip| ip.to_string())
}

#[cfg(target_os = "macos")]
pub(crate) fn get_local_ipv6() -> Option<String> {
    if let Ok(output) = Command::new("ifconfig").arg("en0").output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("inet6 ") && !trimmed.contains("fe80") {
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if parts.len() >= 2 {
                    let ip = parts[1].split('%').next().unwrap_or(parts[1]);
                    return Some(ip.to_string());
                }
            }
        }
    }
    None
}

#[cfg(target_os = "windows")]
pub(crate) fn get_local_ipv6() -> Option<String> {
    let mut cmd = Command::new("ipconfig");
    #[cfg(target_os = "windows")]
    {
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
    let output = cmd.output().ok()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        let trimmed = line.trim();
        if trimmed.to_lowercase().starts_with("ipv6") {
            let addr_part = trimmed.split(':').skip(1).collect::<Vec<_>>().join(":").trim().to_string();
            let ip = addr_part.split('%').next()?.trim().to_string();
            if !ip.is_empty() && !ip.to_lowercase().starts_with("fe80") {
                return Some(ip);
            }
        }
    }
    None
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub(crate) fn get_local_ipv6() -> Option<String> {
    None
}

pub(crate) async fn needs_login() -> bool {
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

#[tauri::command]
pub(crate) fn check_network_status() -> NetworkStatus {
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
