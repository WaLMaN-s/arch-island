use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemState {
    pub cpu_percent: u8,
    pub ram_used_mb: u64,
    pub ram_total_mb: u64,
    pub ram_percent: u8,
    pub uptime_secs: u64,
}

static PREV_CPU: Mutex<(u64, u64)> = Mutex::new((0, 0)); // (idle, total)

pub fn get_system_state() -> SystemState {
    // 1. RAM from /proc/meminfo
    let mut total_kb: u64 = 0;
    let mut avail_kb: u64 = 0;

    if let Ok(meminfo) = fs::read_to_string("/proc/meminfo") {
        for line in meminfo.lines() {
            if line.starts_with("MemTotal:") {
                total_kb = parse_kb(line);
            } else if line.starts_with("MemAvailable:") {
                avail_kb = parse_kb(line);
            }
        }
    }

    let ram_total_mb = total_kb / 1024;
    let ram_used_mb = if total_kb > avail_kb {
        (total_kb - avail_kb) / 1024
    } else {
        0
    };
    let ram_percent = if ram_total_mb > 0 {
        ((ram_used_mb as f64 / ram_total_mb as f64) * 100.0) as u8
    } else {
        0
    };

    // 2. CPU from /proc/stat
    let mut cpu_percent: u8 = 0;
    if let Ok(stat) = fs::read_to_string("/proc/stat") {
        if let Some(first_line) = stat.lines().next() {
            if first_line.starts_with("cpu ") {
                let parts: Vec<u64> = first_line
                    .split_whitespace()
                    .skip(1)
                    .filter_map(|s| s.parse().ok())
                    .collect();

                if parts.len() >= 4 {
                    let idle = parts[3] + parts.get(4).unwrap_or(&0);
                    let total: u64 = parts.iter().sum();

                    if let Ok(mut prev) = PREV_CPU.lock() {
                        let (prev_idle, prev_total) = *prev;
                        if prev_total > 0 && total > prev_total {
                            let total_delta = total - prev_total;
                            let idle_delta = idle.saturating_sub(prev_idle);
                            if total_delta > 0 {
                                let usage = (1.0 - (idle_delta as f64 / total_delta as f64)) * 100.0;
                                cpu_percent = usage.clamp(0.0, 100.0) as u8;
                            }
                        }
                        *prev = (idle, total);
                    }
                }
            }
        }
    }

    // 3. Uptime from /proc/uptime
    let uptime_secs: u64 = fs::read_to_string("/proc/uptime")
        .ok()
        .and_then(|s| {
            s.split_whitespace()
                .next()
                .and_then(|v| v.parse::<f64>().ok())
                .map(|f| f as u64)
        })
        .unwrap_or(0);

    SystemState {
        cpu_percent,
        ram_used_mb,
        ram_total_mb,
        ram_percent,
        uptime_secs,
    }
}

fn parse_kb(line: &str) -> u64 {
    line.split_whitespace()
        .nth(1)
        .and_then(|v| v.parse().ok())
        .unwrap_or(0)
}
