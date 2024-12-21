use regex::Regex;
use std::fs::{self, read_to_string};
use std::io::{self, BufRead};

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
    let pid = std::process::id().to_string();
    let ppid_path = format!("/proc/{}/status", pid);

    let status = fs::read_to_string(ppid_path).expect("Failed to read process status");

    let ppid_line = status
        .lines()
        .find(|line| line.starts_with("PPid"))
        .expect("Failed to find PPid line");

    let ppid: u32 = ppid_line
        .split_whitespace()
        .nth(1)
        .expect("Failed to extract PPid")
        .parse()
        .expect("Failed to parse PPid");

    let cmd_path = format!("/proc/{}/comm", ppid);
    fs::read_to_string(cmd_path)
        .expect("Failed to read command")
        .trim()
        .to_string()
}

pub struct Memory {
    pub total_kb: u64,
    pub free_kb: u64,
    pub used_kb: u64,
    pub total_mb: f64,
    pub free_mb: f64,
    pub used_mb: f64,
    pub total_gb: f64,
    pub free_gb: f64,
    pub used_gb: f64,
}

pub fn get_memory() -> Memory {
    let contents = fs::read_to_string("/proc/meminfo").expect("Failed to read /proc/meminfo");

    let mut total = 0;
    let mut free = 0;
    let mut buffers = 0;
    let mut cached = 0;

    for line in contents.lines() {
        if line.starts_with("MemTotal:") {
            total = line.split_whitespace().nth(1).unwrap().parse().unwrap();
        }
        if line.starts_with("MemFree:") {
            free = line.split_whitespace().nth(1).unwrap().parse().unwrap();
        }
        if line.starts_with("Buffers:") {
            buffers = line.split_whitespace().nth(1).unwrap().parse().unwrap();
        }
        if line.starts_with("Cached:") {
            cached = line.split_whitespace().nth(1).unwrap().parse().unwrap();
        }
    }

    let used = total - free - buffers - cached;

    let total_mb = total as f64 / 1024.0;
    let free_mb = free as f64 / 1024.0;
    let used_mb = used as f64 / 1024.0;

    let total_gb = total_mb / 1024.0;
    let free_gb = free_mb / 1024.0;
    let used_gb = used_mb / 1024.0;

    Memory {
        total_kb: total,
        free_kb: free,
        used_kb: used,
        total_mb: (total_mb * 10.0).round() / 10.0,
        free_mb: (free_mb * 10.0).round() / 10.0,
        used_mb: (used_mb * 10.0).round() / 10.0,
        total_gb: (total_gb * 10.0).round() / 10.0,
        free_gb: (free_gb * 10.0).round() / 10.0,
        used_gb: (used_gb * 10.0).round() / 10.0,
    }
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

pub fn get_kernel() -> String {
    let version = fs::read_to_string("/proc/version")
        .unwrap_or_else(|_| "Unknown Kernel Version".to_string());

    let re = Regex::new(r"Linux version (\S+)").unwrap();

    if let Some(caps) = re.captures(&version) {
        caps[1].to_string()
    } else {
        "Unknown Kernel Version".to_string()
    }
}

pub fn get_desktop() -> String {
    std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_else(|_| "Unknown".to_string())
}

pub fn get_user() -> String {
    std::env::var("USER").unwrap_or_else(|_| "Unknown".to_string())
}

pub fn get_distro() -> Option<String> {
    let file = fs::File::open("/etc/os-release").ok()?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            if line.starts_with("NAME=") {
                return Some(line.split('=').nth(1)?.trim_matches('"').to_string());
            }
        }
    }

    None
}
