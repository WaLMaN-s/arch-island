use crate::services::mpris::{self, MediaState};

#[tauri::command]
pub fn get_media_state() -> Option<MediaState> {
    mpris::get_media_state()
}

#[tauri::command]
pub fn media_play_pause() -> Result<(), String> {
    mpris::play_pause()
}

#[tauri::command]
pub fn media_next() -> Result<(), String> {
    mpris::next()
}

#[tauri::command]
pub fn media_previous() -> Result<(), String> {
    mpris::previous()
}

#[tauri::command]
pub fn media_seek(offset_sec: f64) -> Result<(), String> {
    mpris::seek(offset_sec)
}

#[tauri::command]
pub fn open_spotify_lyrics() -> Result<(), String> {
    // 1. Try to focus existing Spotify window in i3wm
    let res = std::process::Command::new("i3-msg")
        .arg("[class=\"(?i)spotify\"] focus")
        .output();

    let focused = match res {
        Ok(out) => String::from_utf8_lossy(&out.stdout).contains("\"success\":true"),
        Err(_) => false,
    };

    if !focused {
        // Also try xdotool
        let _ = std::process::Command::new("xdotool")
            .args(["search", "--class", "spotify", "windowactivate"])
            .spawn();

        // If spotify is not running, launch flatpak
        let check_running = std::process::Command::new("pgrep")
            .arg("-i")
            .arg("spotify")
            .output();

        let is_running = check_running.map(|o| o.status.success()).unwrap_or(false);
        if !is_running {
            let _ = std::process::Command::new("flatpak")
                .args(["run", "com.spotify.Client"])
                .spawn();
        }
    }

    Ok(())
}
