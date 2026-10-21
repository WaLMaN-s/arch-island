use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrightnessState {
    pub brightness: u8,
}

pub fn get_brightness_state() -> BrightnessState {
    let mut brightness: u8 = 50;

    if let Ok(output) = Command::new("brightnessctl")
        .arg("-m")
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        // e.g. intel_backlight,backlight,7200,30%,24000
        let parts: Vec<&str> = stdout.trim().split(',').collect();
        if parts.len() >= 4 {
            let percent_str = parts[3].replace('%', "");
            if let Ok(val) = percent_str.trim().parse::<u8>() {
                brightness = val;
            }
        }
    }

    BrightnessState { brightness }
}

pub fn set_brightness(percent: u8) -> Result<(), String> {
    let clamped = percent.max(1).min(100);
    Command::new("brightnessctl")
        .args(["set", &format!("{}%", clamped)])
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn change_brightness(delta: i8) -> Result<(), String> {
    let arg = if delta >= 0 {
        format!("+{}%", delta)
    } else {
        format!("{}%-", delta.abs())
    };
    Command::new("brightnessctl")
        .args(["set", &arg])
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}
