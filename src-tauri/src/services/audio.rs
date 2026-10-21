use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeState {
    pub volume: u8,
    pub muted: bool,
}

pub fn get_volume_state() -> VolumeState {
    let mut volume: u8 = 50;
    let mut muted = false;

    // Get volume
    if let Ok(output) = Command::new("pactl")
        .args(["get-sink-volume", "@DEFAULT_SINK@"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        // Look for percentage, e.g. "60%"
        if let Some(pos) = stdout.find('%') {
            let before = &stdout[..pos];
            if let Some(space_pos) = before.rfind(|c: char| !c.is_numeric()) {
                let num_str = &before[space_pos + 1..];
                if let Ok(val) = num_str.trim().parse::<u8>() {
                    volume = val;
                }
            }
        }
    }

    // Get mute
    if let Ok(output) = Command::new("pactl")
        .args(["get-sink-mute", "@DEFAULT_SINK@"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
        if stdout.contains("yes") {
            muted = true;
        }
    }

    VolumeState { volume, muted }
}

pub fn set_volume(percent: u8) -> Result<(), String> {
    let clamped = percent.min(100);
    Command::new("pactl")
        .args(["set-sink-volume", "@DEFAULT_SINK@", &format!("{}%", clamped)])
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn change_volume(delta: i8) -> Result<(), String> {
    let arg = if delta >= 0 {
        format!("+{}%", delta)
    } else {
        format!("-{}%", delta.abs())
    };
    Command::new("pactl")
        .args(["set-sink-volume", "@DEFAULT_SINK@", &arg])
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn toggle_mute() -> Result<(), String> {
    Command::new("pactl")
        .args(["set-sink-mute", "@DEFAULT_SINK@", "toggle"])
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn get_mic_state() -> VolumeState {
    let mut volume: u8 = 50;
    let mut muted = false;

    if let Ok(output) = Command::new("pactl")
        .args(["get-source-volume", "@DEFAULT_SOURCE@"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if let Some(pos) = stdout.find('%') {
            let before = &stdout[..pos];
            if let Some(space_pos) = before.rfind(|c: char| !c.is_numeric()) {
                let num_str = &before[space_pos + 1..];
                if let Ok(val) = num_str.trim().parse::<u8>() {
                    volume = val;
                }
            }
        }
    }

    if let Ok(output) = Command::new("pactl")
        .args(["get-source-mute", "@DEFAULT_SOURCE@"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout).to_lowercase();
        if stdout.contains("yes") {
            muted = true;
        }
    }

    VolumeState { volume, muted }
}

pub fn set_mic_volume(percent: u8) -> Result<(), String> {
    let clamped = percent.min(100);
    Command::new("pactl")
        .args(["set-source-volume", "@DEFAULT_SOURCE@", &format!("{}%", clamped)])
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn toggle_mic_mute() -> Result<(), String> {
    Command::new("pactl")
        .args(["set-source-mute", "@DEFAULT_SOURCE@", "toggle"])
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}
