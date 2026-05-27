use std::os::windows::process::CommandExt;
use std::process::Command;

const CREATE_NO_WINDOW: u32 = 0x08000000;

pub(crate) fn get_wifi_ssid() -> Option<String> {
    let mut cmd = Command::new("netsh");
    cmd.args(["wlan", "show", "interfaces"]);
    cmd.creation_flags(CREATE_NO_WINDOW);
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

pub fn get_local_ipv4() -> Option<String> {
    local_ip_address::local_ip().ok().map(|ip| ip.to_string())
}

pub fn get_local_ipv6() -> Option<String> {
    let mut cmd = Command::new("ipconfig");
    cmd.creation_flags(CREATE_NO_WINDOW);
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
