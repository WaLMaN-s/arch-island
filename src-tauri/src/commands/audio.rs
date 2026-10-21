use crate::services::audio::{self, VolumeState};

#[tauri::command]
pub fn get_volume_state() -> VolumeState {
    audio::get_volume_state()
}

#[tauri::command]
pub fn set_volume(percent: u8) -> Result<(), String> {
    audio::set_volume(percent)
}

#[tauri::command]
pub fn change_volume(delta: i8) -> Result<(), String> {
    audio::change_volume(delta)
}

#[tauri::command]
pub fn toggle_mute() -> Result<(), String> {
    audio::toggle_mute()
}

#[tauri::command]
pub fn get_mic_state() -> VolumeState {
    audio::get_mic_state()
}

#[tauri::command]
pub fn set_mic_volume(percent: u8) -> Result<(), String> {
    audio::set_mic_volume(percent)
}

#[tauri::command]
pub fn toggle_mic_mute() -> Result<(), String> {
    audio::toggle_mic_mute()
}
