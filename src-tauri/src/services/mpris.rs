use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaState {
    pub player: String,
    pub status: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub art_url: String,
    pub position_sec: f64,
    pub duration_sec: f64,
}

pub fn get_media_state() -> Option<MediaState> {
    // Check if playerctl exists and any player is running
    let status_output = Command::new("playerctl")
        .arg("status")
        .output()
        .ok()?;

    if !status_output.status.success() {
        return None;
    }

    let status_str = String::from_utf8_lossy(&status_output.stdout).trim().to_string();
    if status_str.is_empty() {
        return None;
    }

    // Query player name
    let player = Command::new("playerctl")
        .args(["metadata", "--format", "{{playerName}}"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "Unknown".to_string());

    // Query title
    let title = Command::new("playerctl")
        .args(["metadata", "--format", "{{markup_escape(title)}}"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default();

    if title.is_empty() {
        return None;
    }

    // Query artist
    let artist = Command::new("playerctl")
        .args(["metadata", "--format", "{{markup_escape(artist)}}"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default();

    // Query album
    let album = Command::new("playerctl")
        .args(["metadata", "--format", "{{markup_escape(album)}}"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default();

    // Query art url
    let art_url = Command::new("playerctl")
        .args(["metadata", "--format", "{{mpris:artUrl}}"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default();

    // Query position (seconds)
    let position_sec: f64 = Command::new("playerctl")
        .arg("position")
        .output()
        .ok()
        .and_then(|o| String::from_utf8_lossy(&o.stdout).trim().parse().ok())
        .unwrap_or(0.0);

    // Query length (microseconds in mpris:length)
    let duration_sec: f64 = Command::new("playerctl")
        .args(["metadata", "--format", "{{mpris:length}}"])
        .output()
        .ok()
        .and_then(|o| {
            let str_val = String::from_utf8_lossy(&o.stdout).trim().to_string();
            let micro: f64 = str_val.parse().ok()?;
            Some(micro / 1_000_000.0)
        })
        .unwrap_or(0.0);

    Some(MediaState {
        player,
        status: status_str,
        title,
        artist,
        album,
        art_url,
        position_sec,
        duration_sec,
    })
}

pub fn play_pause() -> Result<(), String> {
    Command::new("playerctl")
        .arg("play-pause")
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn next() -> Result<(), String> {
    Command::new("playerctl")
        .arg("next")
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn previous() -> Result<(), String> {
    Command::new("playerctl")
        .arg("previous")
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn seek(offset_sec: f64) -> Result<(), String> {
    let arg = if offset_sec >= 0.0 {
        format!("{}+", offset_sec)
    } else {
        format!("{}-", offset_sec.abs())
    };
    Command::new("playerctl")
        .args(["position", &arg])
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}
