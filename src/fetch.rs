use crate::config::*;
use regex::Regex;
use std::fs::File;
use std::fs::{self, read_to_string};
use std::io::Read;
use std::path::PathBuf;
use std::process::Command;

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

//pub fn get_figlet() -> Result<String, String> {
//    let config = load_config();
//    let output = Command::new("figlet")
//        .arg(config.general.figlet_text)
//        .arg(config.general.figlet_arg)
//        .output();
//
//    match output {
//        Ok(output) => {
//            if !output.stdout.is_empty() {
//                Ok(String::from_utf8_lossy(&output.stdout).to_string())
//            } else {
//                Err("No output from the command".to_string())
//            }
//        }
//        Err(e) => Err(format!("Error running command: {}", e)),
//    }
//}

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

fn get_config_path() -> PathBuf {
    let config_dir = dirs::config_dir().expect("Unable to determine the config directory");
    config_dir.join("statcat").join("config.toml")
}

pub fn load_config() -> Config {
    let path = get_config_path();

    if !path.exists() {
        println!(
            "Config file not found. Creating a default one at {:?}",
            path
        );

        let default_config = Config {
            ..Default::default()
        };

        std::fs::create_dir_all(path.parent().unwrap()).expect("Unable to create config directory");
        let mut file = File::create(&path).expect("Unable to create config file");
        let toml_str = toml::to_string(&default_config).expect("Error serializing config");
        use std::io::Write;
        file.write_all(toml_str.as_bytes())
            .expect("Error writing default config");

        return default_config;
    }

    let mut file = File::open(&path).expect("Unable to open config file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read config file");

    toml::from_str(&contents).expect("Error parsing config file")
}
