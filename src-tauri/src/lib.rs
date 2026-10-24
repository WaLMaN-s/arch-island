pub mod commands;
pub mod services;

use tauri::{LogicalPosition, LogicalSize, Manager, Position, Size};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let handle = app.handle().clone();

            // Start D-Bus notification monitor background task
            services::notifications::start_notification_monitor(handle.clone());

            // Start hardware plug/unplug monitor (Charger, USB, Headphones)
            services::hardware::start_hardware_monitor(handle);

            // Configure window position initially - starts tucked at y=0, height=2
            if let Some(window) = app.get_webview_window("main") {
                let width = 180.0;
                let height = 4.0;
                let x = (1920.0 - width) / 2.0;
                let y = 0.0;

                let _ = window.set_size(Size::Logical(LogicalSize { width, height }));
                let _ = window.set_position(Position::Logical(LogicalPosition { x, y }));
                let _ = window.set_always_on_top(true);
                let _ = window.set_skip_taskbar(true);

                // Start X11 taskbar/pager exclusion enforcer
                services::x11::start_x11_enforcer();

                let i3_cmd = format!(
                    "[instance=\"arch-island\"] floating enable, border pixel 0, sticky enable, resize set {} {}, move position {} {}",
                    width as i32, height as i32, x as i32, y as i32
                );
                let _ = std::process::Command::new("i3-msg")
                    .arg(&i3_cmd)
                    .spawn();
            }



            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = tauri::Emitter::emit(window, "close-requested", ());
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::window::resize_island,
            commands::media::get_media_state,
            commands::media::media_play_pause,
            commands::media::media_next,
            commands::media::media_previous,
            commands::media::media_seek,
            commands::media::open_spotify_lyrics,
            commands::system::get_battery_state,
            commands::system::get_network_state,
            commands::system::get_system_state,
            commands::system::toggle_wifi,

            commands::system::scan_wifi_networks,
            commands::system::connect_wifi,
            commands::system::disconnect_wifi,
            commands::system::dismiss_system_notifications,
            commands::system::get_bluetooth_state,
            commands::system::toggle_bluetooth,
            commands::system::connect_bluetooth,
            commands::system::disconnect_bluetooth,
            commands::system::get_airplane_mode,
            commands::system::toggle_airplane_mode,
            commands::system::get_usb_drives,
            commands::system::open_file_manager,
            commands::system::execute_power_action,
            commands::system::send_desktop_notification,
            commands::audio::get_volume_state,
            commands::audio::set_volume,
            commands::audio::change_volume,
            commands::audio::toggle_mute,
            commands::audio::get_mic_state,
            commands::audio::set_mic_volume,
            commands::audio::toggle_mic_mute,
            commands::brightness::get_brightness_state,
            commands::brightness::set_brightness,
            commands::brightness::change_brightness,
        ])
        .run(tauri::generate_context!())
        .expect("error while running arch-island application");
}
