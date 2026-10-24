use crate::services::battery::{self, BatteryState};
use crate::services::network::{self, NetworkState, WifiNetworkInfo};
use crate::services::system_stats::{self, SystemState};
use crate::services::system_toggles::{self, BluetoothState, UsbDriveInfo};
use std::process::Command;

#[tauri::command]
pub fn get_battery_state() -> Option<BatteryState> {
    battery::get_battery_state()
}

#[tauri::command]
pub fn get_network_state() -> NetworkState {
    network::get_network_state()
}

#[tauri::command]
pub fn get_system_state() -> SystemState {
    system_stats::get_system_state()
}

#[tauri::command]
pub fn toggle_wifi(enabled: bool) -> Result<(), String> {
    network::toggle_wifi(enabled)
}

#[tauri::command]
pub fn scan_wifi_networks() -> Result<Vec<WifiNetworkInfo>, String> {
    Ok(network::scan_wifi_networks())
}

#[tauri::command]
pub fn connect_wifi(ssid: String, password: Option<String>) -> Result<(), String> {
    network::connect_wifi(&ssid, password.as_deref())
}

#[tauri::command]
pub fn disconnect_wifi() -> Result<(), String> {
    network::disconnect_wifi()
}

#[tauri::command]
pub fn dismiss_system_notifications() -> Result<(), String> {
    let _ = Command::new("dunstctl").arg("close-all").spawn();
    Ok(())
}

#[tauri::command]
pub fn get_bluetooth_state() -> BluetoothState {
    system_toggles::get_bluetooth_state()
}

#[tauri::command]
pub fn toggle_bluetooth(enabled: bool) -> Result<(), String> {
    system_toggles::toggle_bluetooth(enabled)
}

#[tauri::command]
pub fn connect_bluetooth(mac: String) -> Result<(), String> {
    system_toggles::connect_bluetooth(mac)
}

#[tauri::command]
pub fn disconnect_bluetooth(mac: String) -> Result<(), String> {
    system_toggles::disconnect_bluetooth(mac)
}

#[tauri::command]
pub fn get_airplane_mode() -> bool {
    system_toggles::get_airplane_mode()
}

#[tauri::command]
pub fn toggle_airplane_mode(enabled: bool) -> Result<(), String> {
    system_toggles::toggle_airplane_mode(enabled)
}

#[tauri::command]
pub fn get_usb_drives() -> Vec<UsbDriveInfo> {
    system_toggles::get_usb_drives()
}

#[tauri::command]
pub fn open_file_manager(target_path: Option<String>) -> Result<(), String> {
    system_toggles::open_file_manager(target_path)
}

#[tauri::command]
pub fn execute_power_action(action: String) -> Result<(), String> {
    match action.as_str() {
        "lock" => {
            if Command::new("loginctl").args(["lock-session"]).spawn().is_err() {
                let _ = Command::new("i3lock").spawn();
            }
        }
        "suspend" => {
            Command::new("systemctl")
                .arg("suspend")
                .spawn()
                .map_err(|e| e.to_string())?;
        }
        "reboot" => {
            Command::new("systemctl")
                .arg("reboot")
                .spawn()
                .map_err(|e| e.to_string())?;
        }
        "shutdown" => {
            Command::new("systemctl")
                .arg("poweroff")
                .spawn()
                .map_err(|e| e.to_string())?;
        }
        _ => return Err("Invalid power action".into()),
    }
    Ok(())
}

#[tauri::command]
pub fn send_desktop_notification(title: String, body: String, icon: Option<String>) -> Result<(), String> {
    let mut cmd = Command::new("notify-send");
    cmd.arg(&title).arg(&body);
    if let Some(i) = icon {
        cmd.arg("-i").arg(i);
    }
    let _ = cmd.spawn();
    Ok(())
}
