use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU32, Ordering};
use tauri::{AppHandle, Emitter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationPayload {
    pub id: u32,
    pub app_name: String,
    pub summary: String,
    pub body: String,
    pub icon: String,
}

static NOTIF_COUNTER: AtomicU32 = AtomicU32::new(1);

pub fn start_notification_monitor(app_handle: AppHandle) {
    std::thread::spawn(move || {
        let child = match Command::new("dbus-monitor")
            .args(["--session", "interface='org.freedesktop.Notifications',member='Notify'"])
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
        {
            Ok(c) => c,
            Err(e) => {
                eprintln!("[arch-island] Failed to start dbus-monitor for notifications: {}", e);
                return;
            }
        };

        if let Some(stdout) = child.stdout {
            let reader = BufReader::new(stdout);
            let mut in_notify = false;
            let mut string_count = 0;
            let mut app_name = String::new();
            let mut icon = String::new();
            let mut summary = String::new();
            let mut body = String::new();

            for line_res in reader.lines() {
                let line = match line_res {
                    Ok(l) => l,
                    Err(_) => break,
                };
                let trimmed = line.trim();

                if trimmed.contains("interface=org.freedesktop.Notifications; member=Notify") {
                    in_notify = true;
                    string_count = 0;
                    app_name.clear();
                    icon.clear();
                    summary.clear();
                    body.clear();
                    continue;
                }

                if in_notify {
                    if trimmed.starts_with("string \"") && trimmed.ends_with('"') {
                        let inner = &trimmed[8..trimmed.len() - 1];
                        string_count += 1;
                        match string_count {
                            1 => app_name = inner.to_string(),
                            2 => icon = inner.to_string(),
                            3 => summary = inner.to_string(),
                            4 => {
                                body = inner.to_string();
                                in_notify = false; // captured all 4 strings!

                                let id = NOTIF_COUNTER.fetch_add(1, Ordering::SeqCst);
                                let payload = NotificationPayload {
                                    id,
                                    app_name: app_name.clone(),
                                    summary: summary.clone(),
                                    body: body.clone(),
                                    icon: icon.clone(),
                                };

                                let _ = app_handle.emit("notification_received", payload);
                            }
                            _ => {}
                        }
                    } else if trimmed.starts_with("method call") || trimmed.starts_with("signal") {
                        // next message started before 4 strings completed
                        in_notify = false;
                    }
                }
            }
        }
    });
}
