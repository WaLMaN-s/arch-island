use serde::{Deserialize, Serialize};
use std::fs;
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatteryState {
    pub percentage: u8,
    pub state: String,
    pub is_charging: bool,
    pub time_remaining: Option<String>,
    pub power_watts: f64,
    pub warning_level: String,
}

pub fn get_battery_state() -> Option<BatteryState> {
    let bat_path = "/sys/class/power_supply/BAT0";
    if !std::path::Path::new(bat_path).exists() {
        return None;
    }

    let capacity_str = fs::read_to_string(format!("{}/capacity", bat_path)).ok()?;
    let percentage: u8 = capacity_str.trim().parse().unwrap_or(0);

    let status_str = fs::read_to_string(format!("{}/status", bat_path))
        .unwrap_or_else(|_| "Discharging".to_string())
        .trim()
        .to_lowercase();

    let is_charging = status_str == "charging" || status_str == "not charging";

    let warning_level = if percentage <= 10 && !is_charging {
        "critical".to_string()
    } else if percentage <= 20 && !is_charging {
        "low".to_string()
    } else {
        "none".to_string()
    };

    // Quick upower query for time remaining and power rate
    let mut time_remaining: Option<String> = None;
    let mut power_watts: f64 = 0.0;

    if let Ok(output) = Command::new("upower")
        .args(["-i", "/org/freedesktop/UPower/devices/battery_BAT0"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            let line = line.trim();
            if line.starts_with("time to empty:") {
                time_remaining = Some(line.replace("time to empty:", "").trim().to_string());
            } else if line.starts_with("time to full:") {
                time_remaining = Some(line.replace("time to full:", "").trim().to_string());
            } else if line.starts_with("energy-rate:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    power_watts = parts[1].parse().unwrap_or(0.0);
                }
            }
        }
    }

    Some(BatteryState {
        percentage,
        state: status_str,
        is_charging,
        time_remaining,
        power_watts,
        warning_level,
    })
}
