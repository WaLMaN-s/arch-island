use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BluetoothDevice {
    pub mac: String,
    pub name: String,
    pub connected: bool,
    pub paired: bool,
    pub icon: Option<String>,
    pub battery_percent: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BluetoothState {
    pub enabled: bool,
    pub connected_device: Option<String>,
    pub connected_mac: Option<String>,
    pub battery_percent: Option<u8>,
    pub devices: Vec<BluetoothDevice>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsbDriveInfo {
    pub name: String,
    pub size: String,
    pub label: Option<String>,
    pub model: Option<String>,
    pub mountpoint: Option<String>,
}

pub fn get_bluetooth_state() -> BluetoothState {
    let mut enabled = false;
    let mut connected_device = None;
    let mut connected_mac = None;
    let mut battery_percent = None;
    let mut devices = Vec::new();

    if let Ok(output) = Command::new("bluetoothctl").arg("show").output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("Powered:") && trimmed.contains("yes") {
                enabled = true;
                break;
            }
        }
    }

    if enabled {
        if let Ok(output) = Command::new("bluetoothctl").arg("devices").output() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                let trimmed = line.trim();
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if parts.len() >= 3 && parts[0] == "Device" {
                    let mac = parts[1].to_string();
                    let name = parts[2..].join(" ");

                    let mut dev_connected = false;
                    let mut dev_paired = false;
                    let mut dev_icon = None;
                    let mut dev_battery = None;

                    if let Ok(info_out) = Command::new("bluetoothctl").args(["info", &mac]).output() {
                        let info_str = String::from_utf8_lossy(&info_out.stdout);
                        for l in info_str.lines() {
                            let t = l.trim();
                            if t.starts_with("Connected:") && t.contains("yes") {
                                dev_connected = true;
                            }
                            if t.starts_with("Paired:") && t.contains("yes") {
                                dev_paired = true;
                            }
                            if t.starts_with("Icon:") {
                                dev_icon = Some(t.replace("Icon:", "").trim().to_string());
                            }
                            if t.starts_with("Battery Percentage:") {
                                if let Some(start) = t.rfind('(') {
                                    if let Some(end) = t.rfind(')') {
                                        if let Ok(pct) = t[start + 1..end].trim().parse::<u8>() {
                                            dev_battery = Some(pct);
                                        }
                                    }
                                }
                            }
                        }
                    }

                    if dev_connected {
                        connected_device = Some(name.clone());
                        connected_mac = Some(mac.clone());
                        if dev_battery.is_some() {
                            battery_percent = dev_battery;
                        }
                    }

                    devices.push(BluetoothDevice {
                        mac,
                        name,
                        connected: dev_connected,
                        paired: dev_paired,
                        icon: dev_icon,
                        battery_percent: dev_battery,
                    });
                }
            }
        }
    }

    BluetoothState {
        enabled,
        connected_device,
        connected_mac,
        battery_percent,
        devices,
    }
}

pub fn toggle_bluetooth(enabled: bool) -> Result<(), String> {
    let arg = if enabled { "on" } else { "off" };
    let output = Command::new("bluetoothctl")
        .args(["power", arg])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

pub fn connect_bluetooth(mac: String) -> Result<(), String> {
    let _ = Command::new("bluetoothctl").args(["trust", &mac]).output();
    let output = Command::new("bluetoothctl")
        .args(["connect", &mac])
        .output()
        .map_err(|e| e.to_string())?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    if output.status.success() || stdout.contains("successful") {
        Ok(())
    } else {
        Err(stdout.to_string())
    }
}

pub fn disconnect_bluetooth(mac: String) -> Result<(), String> {
    let output = Command::new("bluetoothctl")
        .args(["disconnect", &mac])
        .output()
        .map_err(|e| e.to_string())?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    if output.status.success() || stdout.contains("successful") {
        Ok(())
    } else {
        Err(stdout.to_string())
    }
}

pub fn get_airplane_mode() -> bool {
    if let Ok(output) = Command::new("rfkill").arg("list").output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut has_blocked = false;
        let mut total = 0;
        for line in stdout.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("Soft blocked: yes") {
                has_blocked = true;
            }
            if trimmed.starts_with("Soft blocked:") {
                total += 1;
            }
        }
        return has_blocked && total > 0;
    }
    false
}

pub fn toggle_airplane_mode(enabled: bool) -> Result<(), String> {
    let arg = if enabled { "block" } else { "unblock" };
    let output = Command::new("rfkill")
        .args([arg, "all"])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

pub fn get_usb_drives() -> Vec<UsbDriveInfo> {
    let mut drives = Vec::new();

    if let Ok(output) = Command::new("lsblk")
        .args(["-J", "-o", "NAME,SIZE,TYPE,MOUNTPOINTS,HOTPLUG,LABEL,MODEL,TRAN"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(&stdout) {
            if let Some(blocks) = val.get("blockdevices").and_then(|b| b.as_array()) {
                for dev in blocks {
                    let tran = dev.get("tran").and_then(|t| t.as_str()).unwrap_or("");
                    let hotplug = dev.get("hotplug").and_then(|h| h.as_bool()).unwrap_or(false);
                    let name = dev.get("name").and_then(|n| n.as_str()).unwrap_or("").to_string();

                    // If disk is USB or hotpluggable (and not loop or zram)
                    if (tran == "usb" || hotplug) && !name.starts_with("loop") && !name.starts_with("zram") {
                        let size = dev.get("size").and_then(|s| s.as_str()).unwrap_or("").to_string();
                        let model = dev.get("model").and_then(|m| m.as_str()).map(|s| s.trim().to_string());
                        let label = dev.get("label").and_then(|l| l.as_str()).map(|s| s.trim().to_string());
                        
                        // Find mountpoint from parent or children
                        let mut mountpoint: Option<String> = None;
                        if let Some(mps) = dev.get("mountpoints").and_then(|m| m.as_array()) {
                            for mp in mps {
                                if let Some(p) = mp.as_str() {
                                    if !p.is_empty() && p != "[SWAP]" {
                                        mountpoint = Some(p.to_string());
                                        break;
                                    }
                                }
                            }
                        }

                        if mountpoint.is_none() {
                            if let Some(children) = dev.get("children").and_then(|c| c.as_array()) {
                                for child in children {
                                    if let Some(mps) = child.get("mountpoints").and_then(|m| m.as_array()) {
                                        for mp in mps {
                                            if let Some(p) = mp.as_str() {
                                                if !p.is_empty() && p != "[SWAP]" {
                                                    mountpoint = Some(p.to_string());
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                    if mountpoint.is_some() {
                                        break;
                                    }
                                }
                            }
                        }

                        drives.push(UsbDriveInfo {
                            name,
                            size,
                            label,
                            model,
                            mountpoint,
                        });
                    }
                }
            }
        }
    }

    drives
}

pub fn open_file_manager(target_path: Option<String>) -> Result<(), String> {
    let mut cmd = Command::new("thunar");
    if let Some(path) = target_path {
        cmd.arg(path);
    }

    let _ = cmd.spawn().or_else(|_| Command::new("xdg-open").arg(std::env::var("HOME").unwrap_or_else(|_| "/home/man".to_string())).spawn());
    Ok(())
}
