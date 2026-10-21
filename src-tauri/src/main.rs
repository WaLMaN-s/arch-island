// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process;

fn check_single_instance() -> bool {
    let lock_path = "/tmp/arch-island.lock";
    if Path::new(lock_path).exists() {
        if let Ok(content) = fs::read_to_string(lock_path) {
            if let Ok(pid) = content.trim().parse::<i32>() {
                if Path::new(&format!("/proc/{}", pid)).exists() {
                    eprintln!("arch-island is already running (PID: {}). Exiting.", pid);
                    return false;
                }
            }
        }
    }

    let my_pid = process::id();
    if let Ok(mut file) = OpenOptions::new().create(true).write(true).truncate(true).open(lock_path) {
        let _ = writeln!(file, "{}", my_pid);
    }
    true
}

fn main() {
    if !check_single_instance() {
        return;
    }

    arch_island_lib::run()
}
