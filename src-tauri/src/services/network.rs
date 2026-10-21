use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkState {
    pub connected: bool,
    pub network_type: String, // "wifi", "ethernet", "disconnected"
    pub ssid: Option<String>,
    pub signal_percent: u8,
    pub ethernet_connected: bool,
    pub wifi_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WifiNetworkInfo {
    pub ssid: String,
    pub signal_percent: u8,
    pub security: String,
    pub in_use: bool,
}

pub fn is_wifi_enabled() -> bool {
    if let Ok(output) = Command::new("nmcli").args(["radio", "wifi"]).output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        return stdout.trim().to_lowercase().starts_with("enabled");
    }
    false
}

pub fn toggle_wifi(enabled: bool) -> Result<(), String> {
    let arg = if enabled { "on" } else { "off" };
    let output = Command::new("nmcli")
        .args(["radio", "wifi", arg])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

pub fn scan_wifi_networks() -> Vec<WifiNetworkInfo> {
    let mut map: HashMap<String, WifiNetworkInfo> = HashMap::new();

    if let Ok(output) = Command::new("nmcli")
        .args(["-t", "-f", "SSID,SIGNAL,SECURITY,IN-USE", "dev", "wifi", "list", "--rescan", "yes"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 4 {
                let ssid = parts[0].replace("\\:", ":").trim().to_string();
                if ssid.is_empty() || ssid == "--" {
                    continue;
                }
                let signal_percent = parts[1].parse().unwrap_or(0);
                let security = parts[2].trim().to_string();
                let in_use = parts[3].trim() == "*";

                let info = WifiNetworkInfo {
                    ssid: ssid.clone(),
                    signal_percent,
                    security,
                    in_use,
                };

                // Keep the one with highest signal or in_use
                if let Some(existing) = map.get(&ssid) {
                    if in_use || (!existing.in_use && signal_percent > existing.signal_percent) {
                        map.insert(ssid, info);
                    }
                } else {
                    map.insert(ssid, info);
                }
            }
        }
    }

    let mut list: Vec<WifiNetworkInfo> = map.into_values().collect();
    list.sort_by(|a, b| {
        if a.in_use != b.in_use {
            b.in_use.cmp(&a.in_use)
        } else {
            b.signal_percent.cmp(&a.signal_percent)
        }
    });
    list
}

pub fn connect_wifi(ssid: &str, password: Option<&str>) -> Result<(), String> {
    let mut cmd = Command::new("nmcli");
    cmd.args(["dev", "wifi", "connect", ssid]);

    if let Some(pwd) = password {
        if !pwd.trim().is_empty() {
            cmd.args(["password", pwd.trim()]);
        }
    }

    let output = cmd.output().map_err(|e| e.to_string())?;
    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

pub fn disconnect_wifi() -> Result<(), String> {
    // Find wifi interface
    if let Ok(output) = Command::new("nmcli").args(["-t", "-f", "DEVICE,TYPE", "device"]).output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 2 && parts[1] == "wifi" {
                let dev = parts[0];
                let _ = Command::new("nmcli").args(["device", "disconnect", dev]).output();
                return Ok(());
            }
        }
    }
    Ok(())
}

pub fn get_network_state() -> NetworkState {
    let mut connected = false;
    let mut network_type = "disconnected".to_string();
    let mut ssid: Option<String> = None;
    let mut signal_percent: u8 = 0;
    let mut ethernet_connected = false;
    let wifi_enabled = is_wifi_enabled();

    // Check wifi
    if wifi_enabled {
        if let Ok(output) = Command::new("nmcli")
            .args(["-t", "-f", "active,ssid,signal", "dev", "wifi"])
            .output()
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                if line.starts_with("yes:") {
                    let parts: Vec<&str> = line.split(':').collect();
                    if parts.len() >= 3 {
                        connected = true;
                        network_type = "wifi".to_string();
                        let raw_ssid = parts[1].replace("\\:", ":");
                        if !raw_ssid.is_empty() {
                            ssid = Some(raw_ssid);
                        }
                        signal_percent = parts[2].parse().unwrap_or(0);
                        break;
                    }
                }
            }
        }
    }

    // Check ethernet
    if let Ok(output) = Command::new("nmcli")
        .args(["-t", "-f", "TYPE,STATE", "device"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 2 && parts[0] == "ethernet" && parts[1] == "connected" {
                ethernet_connected = true;
                if !connected {
                    connected = true;
                    network_type = "ethernet".to_string();
                }
            }
        }
    }

    NetworkState {
        connected,
        network_type,
        ssid,
        signal_percent,
        ethernet_connected,
        wifi_enabled,
    }
}
