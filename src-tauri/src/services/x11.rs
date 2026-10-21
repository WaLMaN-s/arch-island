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

/// Spawns a background thread that periodically re-applies the skip_taskbar
/// property during startup and window initialization.
pub fn start_x11_enforcer() {
    std::thread::spawn(move || {
        // Run checks frequently during the first 10 seconds of startup
        for _ in 0..40 {
            std::thread::sleep(std::time::Duration::from_millis(250));
            apply_skip_taskbar();
        }
        // Then keep checking every 5 seconds as an ongoing safeguard
        loop {
            std::thread::sleep(std::time::Duration::from_secs(5));
            apply_skip_taskbar();
        }
    });
}
