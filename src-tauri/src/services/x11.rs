unsafe extern "C" {
    fn arch_island_set_skip_taskbar(target_pid: u32);
}

/// Applies _NET_WM_STATE_SKIP_TASKBAR and _NET_WM_STATE_SKIP_PAGER
/// to all windows belonging to this application process.
pub fn apply_skip_taskbar() {
    let pid = std::process::id();
    unsafe {
        arch_island_set_skip_taskbar(pid);
    }
}

/// Spawns a background thread that applies the skip_taskbar property
/// periodically during initial startup (giving WebKitGTK & i3wm time to map the window).
pub fn start_x11_enforcer() {
    std::thread::spawn(move || {
        for delay_ms in [500, 1000, 2000, 3500, 6000] {
            std::thread::sleep(std::time::Duration::from_millis(delay_ms));
            apply_skip_taskbar();
        }
    });
}
