use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareEventPayload {
    pub kind: String,        // "charger", "usb", "audio"
    pub status: String,      // "connected", "disconnected"
    pub title: String,
    pub subtitle: String,
    pub icon: String,
    pub percentage: Option<u8>,
}

fn play_sound(sound_name: &str) {
    let sound_path = format!("/usr/share/sounds/freedesktop/stereo/{}.oga", sound_name);
    if Path::new(&sound_path).exists() {
        let _ = Command::new("pw-play")
            .arg(&sound_path)
            .spawn()
            .or_else(|_| Command::new("paplay").arg(&sound_path).spawn())
            .or_else(|_| Command::new("canberra-gtk-play").args(["-i", sound_name]).spawn());
    }
}

fn read_ac_online() -> bool {
    let possible_paths = [
        "/sys/class/power_supply/AC0/online",
        "/sys/class/power_supply/AC/online",
        "/sys/class/power_supply/ADP1/online",
    ];

    for path in &possible_paths {
        if let Ok(content) = fs::read_to_string(path) {
            return content.trim() == "1";
        }
    }

    // Fallback: check battery status
    if let Ok(status) = fs::read_to_string("/sys/class/power_supply/BAT0/status") {
        let st = status.trim().to_lowercase();
        return st == "charging" || st == "full";
    }

    false
}

fn read_bat_percentage() -> u8 {
    fs::read_to_string("/sys/class/power_supply/BAT0/capacity")
        .ok()
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(70)
}

fn get_active_usb_devices() -> HashSet<String> {
    let mut devices = HashSet::new();
    let usb_dir = Path::new("/sys/bus/usb/devices");
    if let Ok(entries) = fs::read_dir(usb_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            // Filter out root hubs and interface descriptors (which contain ':')
            if !name.contains(':') && !name.starts_with("usb") {
                devices.insert(name);
            }
        }
    }
    devices
}

fn get_usb_product_name(dev: &str) -> String {
    let path = format!("/sys/bus/usb/devices/{}/product", dev);
    fs::read_to_string(path)
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|_| "USB Device".to_string())
}

fn get_audio_port() -> String {
    if let Ok(output) = Command::new("pactl").args(["list", "sinks"]).output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("Active Port:") {
                return trimmed.replace("Active Port:", "").trim().to_string();
            }
        }
    }
    String::new()
}

pub fn start_hardware_monitor(app_handle: AppHandle) {
    thread::spawn(move || {
        let mut last_ac = read_ac_online();
        let mut last_usb = get_active_usb_devices();
        let mut last_audio_port = get_audio_port();

        // Initial delay to let the app start up cleanly
        thread::sleep(Duration::from_millis(1500));

        loop {
            thread::sleep(Duration::from_millis(800));

            // 1. Check AC / Charger
            let current_ac = read_ac_online();
            if current_ac != last_ac {
                last_ac = current_ac;
                let bat_pct = read_bat_percentage();

                if current_ac {
                    play_sound("device-added");
                    let payload = HardwareEventPayload {
                        kind: "charger".to_string(),
                        status: "connected".to_string(),
                        title: "Charging".to_string(),
                        subtitle: "Charger connected".to_string(),
                        icon: "charging".to_string(),
                        percentage: Some(bat_pct),
                    };
                    let _ = app_handle.emit("hardware_event", payload);
                } else {
                    play_sound("device-removed");
                    let payload = HardwareEventPayload {
                        kind: "charger".to_string(),
                        status: "disconnected".to_string(),
                        title: "Charger Disconnected".to_string(),
                        subtitle: "Running on battery".to_string(),
                        icon: "battery".to_string(),
                        percentage: Some(bat_pct),
                    };
                    let _ = app_handle.emit("hardware_event", payload);
                }
            }

            // 2. Check USB Devices
            let current_usb = get_active_usb_devices();
            // Detected new device
            for added in current_usb.difference(&last_usb) {
                let product = get_usb_product_name(added);
                play_sound("device-added");
                let payload = HardwareEventPayload {
                    kind: "usb".to_string(),
                    status: "connected".to_string(),
                    title: "USB Connected".to_string(),
                    subtitle: product,
                    icon: "usb".to_string(),
                    percentage: None,
                };
                let _ = app_handle.emit("hardware_event", payload);
            }
            // Detected removed device
            for _removed in last_usb.difference(&current_usb) {
                play_sound("device-removed");
                let payload = HardwareEventPayload {
                    kind: "usb".to_string(),
                    status: "disconnected".to_string(),
                    title: "USB Disconnected".to_string(),
                    subtitle: "Device removed".to_string(),
                    icon: "usb".to_string(),
                    percentage: None,
                };
                let _ = app_handle.emit("hardware_event", payload);
            }
            last_usb = current_usb;

            // 3. Check Audio Jack / Headphones
            let current_audio_port = get_audio_port();
            if !current_audio_port.is_empty() && current_audio_port != last_audio_port {
                if current_audio_port.contains("headphones") {
                    play_sound("device-added");
                    let payload = HardwareEventPayload {
                        kind: "audio".to_string(),
                        status: "connected".to_string(),
                        title: "Headphones Connected".to_string(),
                        subtitle: "Audio output switched".to_string(),
                        icon: "headphones".to_string(),
                        percentage: None,
                    };
                    let _ = app_handle.emit("hardware_event", payload);
                } else if last_audio_port.contains("headphones") {
                    play_sound("device-removed");
                    let payload = HardwareEventPayload {
                        kind: "audio".to_string(),
                        status: "disconnected".to_string(),
                        title: "Headphones Disconnected".to_string(),
                        subtitle: "Speaker audio restored".to_string(),
                        icon: "headphones".to_string(),
                        percentage: None,
                    };
                    let _ = app_handle.emit("hardware_event", payload);
                }
                last_audio_port = current_audio_port;
            }
        }
    });
}
