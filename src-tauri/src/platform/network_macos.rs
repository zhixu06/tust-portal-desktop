use std::process::Command;

pub(crate) fn get_wifi_ssid() -> Option<String> {
    // First attempt: ipconfig getsummary en0 (faster, works on newer macOS)
    if let Some(ssid) = get_ssid_via_ipconfig("en0") {
        let trimmed = ssid.trim().to_string();
        if !trimmed.is_empty() && trimmed != "<redacted>" {
            return Some(trimmed);
        }
    }

    // Fallback: discover Wi-Fi port name and query preferred networks
    let port_name = get_wifi_port_name()?;
    if !is_interface_active(&port_name) {
        return None;
    }
    get_current_ssid(&port_name)
}

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

fn get_current_ssid(port_name: &str) -> Option<String> {
    let output = Command::new("networksetup")
        .args(["-listpreferredwirelessnetworks", port_name])
        .output()
        .ok()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    // First line is header, second line is current SSID (indented with tab)
    stdout.lines().nth(1).map(|s| s.trim().to_string())
}

pub fn get_local_ipv4() -> Option<String> {
    local_ip_address::local_ip().ok().map(|ip| ip.to_string())
}

pub fn get_local_ipv6() -> Option<String> {
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
