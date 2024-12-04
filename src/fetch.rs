use std::process::Command;
use std::io::BufRead;
use std::fs::{read_to_string, self};
use sysinfo::{System, RefreshKind, CpuRefreshKind};

pub fn get_cpu() -> Option<String> {
    if let Ok(cpuinfo) = read_to_string("/proc/cpuinfo") {
        for line in cpuinfo.lines() {
            if line.starts_with("model name") {
                return line.split(':').nth(1).map(|s| s.trim().to_string());
            }
        }
    }
    None
}



pub fn get_shell() -> String {
    let output = Command::new("ps")
        .arg("-p")
        .arg(std::process::id().to_string())
        .arg("-o")
        .arg("ppid=")
        .output()
        .expect("Failed to execute ps");

    let parent_pid = String::from_utf8_lossy(&output.stdout).trim().to_string();

    let output = Command::new("ps")
        .arg("-p")
        .arg(parent_pid)
        .arg("-o")
        .arg("comm=")
        .output()
        .expect("Failed to execute ps");

    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

pub fn detect_package_manager() -> &'static str {
    let managers = [
        ("pacman", "pacman"),
        ("dpkg", "dpkg"),
        ("rpm", "rpm"),
        ("zypper", "zypper"),
    ];

    for (name, cmd) in &managers {
        if Command::new(cmd).output().is_ok() {
            return *name;
        }
    }
    
    "None"
}

pub fn get_uptime() -> Option<String> {
    if let Ok(content) = fs::read_to_string("/proc/uptime") {
        if let Some(uptime_seconds) = content.split_whitespace().next()?.parse::<f64>().ok() {
            let hours = (uptime_seconds / 3600.0).floor();
            let minutes = ((uptime_seconds % 3600.0) / 60.0).floor();
            return Some(format!("{}h {}m", hours, minutes));
        }
    }
    None
}

pub fn get_hostname() -> Option<String> {
    fs::read_to_string("/etc/hostname")
        .ok()
        .map(|s| s.trim().to_string())
}

pub fn get_desktop() -> String {
    std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_else(|_| "Unknown".to_string())
}

pub fn get_user() -> String {
    std::env::var("USER").unwrap_or_else(|_| "Unknown".to_string())
}
