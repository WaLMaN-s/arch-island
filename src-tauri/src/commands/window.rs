use tauri::{LogicalPosition, LogicalSize, Position, Size, Window};

#[tauri::command]
pub fn resize_island(window: Window, mode: String, custom_width: Option<f64>, custom_height: Option<f64>) -> Result<(), String> {
    let (width, height) = if let (Some(w), Some(h)) = (custom_width, custom_height) {
        (w, h)
    } else {
        match mode.as_str() {
            "expanded" => (480.0, 385.0),
            "compact" => (330.0, 32.0),
            "tucked" => (180.0, 4.0),
            _ => (180.0, 4.0),
        }
    };

    eprintln!("[RESIZE] mode: {}, size: {}x{}", mode, width, height);

    // Calculate top-center position based on 1920 width, flush against top edge (y = 0)
    let x = (1920.0 - width) / 2.0;
    let y = 0.0;

    let _ = window.set_size(Size::Logical(LogicalSize { width, height }));
    let _ = window.set_position(Position::Logical(LogicalPosition { x, y }));

    // Instant i3wm floating repositioning & resizing (single command string)
    let focus_cmd = if mode == "expanded" { ", focus" } else { "" };
    let i3_cmd = format!(
        "[instance=\"arch-island\"] floating enable, border pixel 0, sticky enable, resize set {} {}, move position {} {}{}",
        width as i32, height as i32, x as i32, y as i32, focus_cmd
    );
    let _ = std::process::Command::new("i3-msg")
        .arg(&i3_cmd)
        .spawn();

    if mode == "expanded" {
        let _ = window.set_focus();
    }

    Ok(())
}

