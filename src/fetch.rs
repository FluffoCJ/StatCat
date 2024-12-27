use regex::{bytes, Regex};
use std::fs::{self, read_to_string};
use std::io::{self, BufRead};
use std::net::{IpAddr, UdpSocket};

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

pub fn get_local_ip() -> String {
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.connect("8.8.8.8:80").unwrap();
    if let IpAddr::V4(ip) = socket.local_addr().unwrap().ip() {
        ip.to_string()
    } else {
        String::from("Unable to determine IP")
    }
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
