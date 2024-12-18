use crate::config::*;
use battery::Manager;
use chrono::Local;
use nixinfo;
use std::fs;
use std::io;
use home::home_dir;
use sysinfo::System;
use std::collections::HashMap;
use std::error::Error;
use std::process::Command;

mod config;
mod fetch;
mod packages;

fn main() -> Result<(), Box<dyn Error>> {
    let mut system = System::new();
    system.refresh_memory();
    

    let config = load_config()?;
    if config.general.figlet {
        let figlet_color = config.general.figlet_color.clone().unwrap_or_default();
        let figlet_text = get_figlet(&config).unwrap_or_default();
        let figlet = figlet_text
                .lines()
                .take(figlet_text.lines().count() - 1)
                .collect::<Vec<_>>()
                .join("\n");
        println!("{figlet_color}{figlet} \x1b[0m");
    }
    print_config(&config)?;

    Ok(())
}

fn print_config(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;

    for line in &config.config.output {
        let mut line = line.clone();

        for (key, value) in &config.colors {
            let placeholder = format!("{{{}}}", key);

            if value.starts_with("#") {
                let ansi_color = hex_to_ansi(value);
                line = line.replace(&placeholder, &ansi_color);
            } else {
                line = line.replace(&placeholder, value);
            }
        }
        let mut replacements: HashMap<&str, fn() -> String> = HashMap::new();
        replacements.insert("{os}", || fetch::get_distro().unwrap_or_default());
        replacements.insert("{hostname}", || fetch::get_hostname().unwrap_or_default());
        replacements.insert("{cpu}", || fetch::get_cpu().unwrap_or_default());
        replacements.insert("{packages}", || packages::get_package_count().to_string());
        replacements.insert("{kernel}", || fetch::get_kernel());
        replacements.insert("{terminal}", || nixinfo::terminal().unwrap_or_default());
        replacements.insert("{uptime}", || fetch::get_uptime().unwrap_or_default());
        replacements.insert("{username}", || fetch::get_user());
        replacements.insert("{shell}", || fetch::get_shell());
        replacements.insert("{desktop}", || fetch::get_desktop());

        for (placeholder, fetch_func) in &replacements {
            if line.contains(placeholder) {
                line = line.replace(placeholder, &fetch_func());
            }
        }
        println!("{}", line);
    }
    Ok(())
}


fn get_figlet(config: &Config) -> Result<String, String> {
    let output = Command::new("figlet")
        .arg(&config.general.figlet_text)
        .arg(&config.general.figlet_arg)
        .output();

    match output {
        Ok(output) => {
            if !output.stdout.is_empty() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            } else {
                Err("No output from the command".to_string())
            }
        }
        Err(e) => Err(format!("Error running command: {}", e)),
    }
}

pub fn load_config() -> Result<Config, io::Error> {
    if let Some(home_path) = home_dir() {
        let config_path = home_path.join(".config/statcat/config.toml");

        let config_str = fs::read_to_string(config_path)?;

        match toml::de::from_str::<Config>(&config_str) {
            Ok(config) => Ok(config),
            Err(_) => Err(io::Error::new(io::ErrorKind::InvalidData, "Failed to parse TOML")),
        }
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "Home directory not found"))
    }
}


fn hex_to_ansi(hex: &str) -> String {
    let hex = hex.trim_start_matches('#');

    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);

    format!("\u{001b}[38;2;{};{};{}m", r, g, b)
}



