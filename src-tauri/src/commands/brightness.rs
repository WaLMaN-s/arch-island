use crate::services::brightness::{self, BrightnessState};

#[tauri::command]
pub fn get_brightness_state() -> BrightnessState {
    brightness::get_brightness_state()
}

#[tauri::command]
pub fn set_brightness(percent: u8) -> Result<(), String> {
    brightness::set_brightness(percent)
}

#[tauri::command]
pub fn change_brightness(delta: i8) -> Result<(), String> {
    brightness::change_brightness(delta)
}
