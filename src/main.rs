#![allow(unused)]
use std::io::Read;
use sysinfo::System;
use toml;
use std::path::PathBuf;
use std::fs::File;
use nixinfo;
use crate::config::*;
use chrono::Local;
use battery::Manager;


mod fetch;
mod config;

fn main() {
    let system = System::new_all();
    let config = load_config();

    let mut border_char = "";
    let mut corner_char = "";
    let mut end_corner_char = "";
    let mut bottom_left = "";
    let mut bottom_right = "";
    let mut separator = config.general.separator.to_string();
    let r = "\x1b[0m";
    let mut pad = 0;
    let mut side = "";

    if config.general.decoration == "nitch" {
        border_char = "─";
        corner_char = "╭";
        end_corner_char = "╮";
        bottom_left = "╰";
        bottom_right = "╯"; 
        separator = "│".to_string();
        pad = 10;
        side = "│  ";
        println!("╭────────────────╮");
    }


    for field in &config.order.fields {
        if let Some((text, icon, color)) = get_icon_text(&config, field) {
            let color_code = match color.as_deref() {
                Some("black") => "\x1b[30m",
                Some("red") => "\x1b[31m",
                Some("green") => "\x1b[32m",
                Some("yellow") => "\x1b[33m",
                Some("blue") => "\x1b[34m",
                Some("magenta") => "\x1b[35m",
                Some("cyan") => "\x1b[36m",
                Some("white") => "\x1b[37m",
                Some("bright_black") => "\x1b[90m",
                Some("bright_red") => "\x1b[91m",
                Some("bright_green") => "\x1b[92m",
                Some("bright_yellow") => "\x1b[93m",
                Some("bright_blue") => "\x1b[94m",
                Some("bright_magenta") => "\x1b[95m",
                Some("bright_cyan") => "\x1b[96m",
                Some("bright_white") => "\x1b[97m",
                _ => "\x1b[0m", 
            };


            match field.as_str() {
                "memory" => {
                    let used_memory = bytes_to_gb(system.used_memory());
                    let total_memory = bytes_to_gb(system.total_memory());

                    if config.memory.display_mb {
                        let used_memory = gb_to_mb(used_memory);
                        let total_memory = gb_to_mb(total_memory);
                        println!(
                        "{side}{color_code}{icon} {:<pad$}{r}│ MB/{total_memory}MB\x1b[0m",
                        text,
                        );
                    }
                    else if config.memory.display_percent {
                        let used_memory = (used_memory / total_memory) * 100.0;
                        println!(
                        "{side}{color_code}{icon} {:<pad$}{r}{separator} {used_memory}%\x1b[0m",
                        text,
                        );
                    }
                    else {
                        println!(
                        "{side}{color_code}{icon} {:<pad$}{r}{separator} {used_memory}GB/{total_memory}GB\x1b[0m",
                        text,
                        );
                    }
                }
                "time_date" => {
                    let now = Local::now();
                    let formatted = now.format(config.time_date.format.as_str()).to_string(); 
                    println!(
                    "{side}{color_code}{icon} {:<pad$}{r}{separator} {formatted}",
                    text, 
                    );
 
                }
                "os" => {
                    let distro = nixinfo::distro().unwrap_or_default().trim_matches('"').to_string();
                    println!(
                        "{side}{color_code}{icon} {:<pad$}{r}{separator} {distro}\x1b[0m",
                        text,
                    );
                }
                "battery" => {
                    let manager = Manager::new().unwrap();
                    for (idx, battery) in manager.batteries().unwrap().enumerate() {
                        let battery = battery.unwrap();
                        if config.battery.percentage {
                            // Percentage
                            println!(
                            "{side}{color_code}{icon} {:<pad$}{r}{separator} {:.2}", battery.state_of_charge().value * 100.0,
                            text,
                            );

                        }
                        if config.battery.charging_state {
                            // Charging state
                            println!(
                            "{side}{color_code}{icon} {:<pad$}{r}{separator} {:?}", battery.state(),
                            text,
                        );
                        }
                    }
                }
                "colors" => {
                    let color_icon = &config.colors.color_icon;
                    let colors = [
                        "\x1b[90m", // Bright black
                        "\x1b[31m", // Red
                        "\x1b[33m", // Yellow
                        "\x1b[32m", // Green
                        "\x1b[36m", // Cyan
                        "\x1b[34m", // Blue
                        "\x1b[35m", // Magenta
                        "\x1b[30m", // Black
                    ];
                
                    print!("{side}{color_code}{icon} {:<pad$}{r}{separator}", text);
                
                    for element in colors {
                        print!("{element} {color_icon}\x1b[0m ");
                    }
                    println!();
                }
                _ => {
                    let value = match field.as_str() {
                        "hostname" => fetch::get_hostname().unwrap_or_else(|| "Unknown Host Name".to_string()),
                        "cpu" => fetch::get_cpu().unwrap_or_else(|| "Unknown CPU".to_string()),
                        "packages" => nixinfo::packages(fetch::detect_package_manager()).unwrap_or_default(),
                        "shell" => fetch::get_shell(),
                        "gpu" => nixinfo::gpu().unwrap_or_default(),
                        "terminal" => nixinfo::terminal().unwrap_or_default(),
                        "uptime" => fetch::get_uptime().unwrap_or_else(|| "Unknown uptime".to_string()),
                        "desktop" => nixinfo::environment().unwrap_or_default(),
                        "username" => fetch::get_user().to_string(),
                        _ => continue,
                    };
                    println!(
                    "{side}{color_code}{icon} {:<pad$}{r}{separator} {value}\x1b[0m",
                    text,
                    );
                }
            }
        } 
        else {
            println!("Unknown field: {}", field);
        }

        
    }
    if config.general.decoration == "nitch" {
        println!("╰────────────────╯");
    }
}


fn gb_to_mb(gb: f64) -> f64 {
    (gb as f64 * 1024.0).round()
}

fn bytes_to_gb(bytes: u64) -> f64 {
    (bytes as f64 / 1_073_741_824.0 * 10.0).round() / 10.0
}


fn get_icon_text<'a>(config: &'a Config, field: &'a str) -> Option<(&'a str, String, Option<String>)> {
    match field {
        "os" => Some((&config.os.text, config.os.icon.clone(), config.os.color.clone())),
        "cpu" => Some((&config.cpu.text, config.cpu.icon.clone(), config.cpu.color.clone())),
        "memory" => Some((&config.memory.text, config.memory.icon.clone(), config.memory.color.clone())),
        "hostname" => Some((&config.hostname.text, config.hostname.icon.clone(), config.hostname.color.clone())),
        "packages" => Some((&config.packages.text, config.packages.icon.clone(), config.packages.color.clone())),
        "shell" => Some((&config.shell.text, config.shell.icon.clone(), config.shell.color.clone())),
        "gpu" => Some((&config.gpu.text, config.gpu.icon.clone(), config.gpu.color.clone())),
        "terminal" => Some((&config.terminal.text, config.terminal.icon.clone(), config.terminal.color.clone())),
        "uptime" => Some((&config.uptime.text, config.uptime.icon.clone(), config.uptime.color.clone())),
        "desktop" => Some((&config.desktop.text, config.desktop.icon.clone(), config.desktop.color.clone())),
        "username" => Some((&config.username.text, config.username.icon.clone(), config.username.color.clone())),
        "time_date" => Some((&config.time_date.text, config.time_date.icon.clone(), config.time_date.color.clone())),
        "battery" => Some((&config.battery.text, config.battery.icon.clone(), config.battery.color.clone())),
        "colors" => Some((&config.colors.text, config.colors.icon.clone(), config.colors.color.clone())),

        _ => None,
    }
}




fn get_config_path() -> PathBuf {
    let config_dir = dirs::config_dir().expect("Unable to determine the config directory");
    config_dir.join("statcat").join("config.toml")
}

fn load_config() -> Config {
    let path = get_config_path();

    if !path.exists() {
        println!("Config file not found. Creating a default one at {:?}", path);

        let default_config = Config {
            ..Default::default() 
        };

        std::fs::create_dir_all(path.parent().unwrap()).expect("Unable to create config directory");
        let mut file = File::create(&path).expect("Unable to create config file");
        let toml_str = toml::to_string(&default_config).expect("Error serializing config");
        use std::io::Write;
        file.write_all(toml_str.as_bytes()).expect("Error writing default config");

        return default_config;
    }

    let mut file = File::open(&path).expect("Unable to open config file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read config file");

    toml::from_str(&contents).expect("Error parsing config file")
}




