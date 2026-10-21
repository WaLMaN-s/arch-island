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
    let mut title = Command::new("playerctl")
        .args(["metadata", "--format", "{{title}}"])
        .output()
        .map(|o| clean_meta(&String::from_utf8_lossy(&o.stdout)))
        .unwrap_or_default();

    if title.is_empty() {
        // Fallback: extract filename from xesam:url
        let url = Command::new("playerctl")
            .args(["metadata", "--format", "{{xesam:url}}"])
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_default();

        if !url.is_empty() {
            if let Some(filename) = url.split('/').last() {
                let decoded = percent_decode(filename);
                title = clean_meta(&decoded);
            }
        }
    }

    let extensions = [".mp3", ".flac", ".wav", ".m4a", ".aac", ".ogg", ".opus", ".wma"];
    for ext in extensions {
        if title.to_lowercase().ends_with(ext) {
            title = title[..title.len() - ext.len()].trim().to_string();
        }
    }

    if title.is_empty() {
        return None;
    }

    // Query artist
    let mut artist = Command::new("playerctl")
        .args(["metadata", "--format", "{{artist}}"])
        .output()
        .map(|o| clean_meta(&String::from_utf8_lossy(&o.stdout)))
        .unwrap_or_default();

    // If artist is empty but title has "Artist - Song", split them
    if artist.is_empty() && title.contains(" - ") {
        let parts: Vec<&str> = title.splitn(2, " - ").collect();
        if parts.len() == 2 && !parts[0].trim().is_empty() {
            artist = parts[0].trim().to_string();
            title = parts[1].trim().to_string();
        }
    }

    if artist.is_empty() {
        artist = "USB / Local Audio".to_string();
    }

    // Query album
    let album = Command::new("playerctl")
        .args(["metadata", "--format", "{{album}}"])
        .output()
        .map(|o| clean_meta(&String::from_utf8_lossy(&o.stdout)))
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

fn percent_decode(input: &str) -> String {
    let mut result = String::new();
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(byte) = u8::from_str_radix(&input[i + 1..i + 3], 16) {
                result.push(byte as char);
                i += 3;
                continue;
            }
        } else if bytes[i] == b'+' {
            result.push(' ');
            i += 1;
            continue;
        }
        result.push(bytes[i] as char);
        i += 1;
    }
    result
}

fn clean_meta(s: &str) -> String {
    s.replace("&apos;", "'")
        .replace("&#39;", "'")
        .replace("&quot;", "\"")
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .trim()
        .to_string()
}
