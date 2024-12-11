use sysinfo::System;
use nixinfo;
use crate::config::*;
use chrono::Local;
use battery::Manager;

mod fetch;
mod config;
mod packages;



fn main() {
    let mut system = System::new();
    system.refresh_memory();
    let config = fetch::load_config();


    let mut separator = config.general.separator.to_string();
    let r = "\x1b[0m";
    let mut pad = config.general.padding;
    let mut side = "";


    if config.general.figlet == true {
        let text = fetch::get_figlet().unwrap_or_default();
        let text = text.lines().take(text.lines().count() - 1).collect::<Vec<_>>().join("\n");
        let color = config.general.figlet_color.clone().unwrap_or_default(); 
        let color_code = get_color_code(&color);
        println!("{color_code}{text}\x1b[0m");
    }


    if config.general.decoration == "border" {

        separator = "│".to_string();
        pad = 10;
        side = "│  ";
        println!("╭────────────────╮");
    }

    
    

    for field in &config.order.fields {
        if let Some((text, icon, color)) = get_icon_text(&config, field) {
            let color_code = if let Some(color) = color {
               get_color_code(&color)
            } 
            else {
                "\x1b[0m".to_string()
            };




            match field.as_str() {
                "memory" => {
                    let used_memory = bytes_to_gb(system.used_memory());
                    let total_memory = bytes_to_gb(system.total_memory());

                    if config.memory.display_mb {
                        let used_memory = gb_to_mb(used_memory);
                        let total_memory = gb_to_mb(total_memory);
                        println!(
                        "{side}{color_code}{icon} {:<pad$}{r}│ {used_memory}MB/{total_memory}MB\x1b[0m",
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
                        "packages" => packages::get_package_count().to_string(),
                        "shell" => fetch::get_shell(),
                        "gpu" => nixinfo::gpu().unwrap_or_default(),
                        "terminal" => nixinfo::terminal().unwrap_or_default(),
                        "uptime" => fetch::get_uptime().unwrap_or_else(|| "Unknown uptime".to_string()),
                        "desktop" => nixinfo::environment().unwrap_or_default(),
                        "username" => fetch::get_user().to_string(),
                        "kernel" => nixinfo::kernel().unwrap_or_default(),
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
    if config.general.decoration == "border" {
        println!("╰────────────────╯");
    }
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
        "kernel" => Some((&config.kernel.text, config.kernel.icon.clone(), config.kernel.color.clone())),
        _ => None,
    }
}


fn parse_hex_color(hex: &str) -> Result<(u8, u8, u8), &'static str> {
    if hex.len() == 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "Invalid hex format")?;
        let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "Invalid hex format")?;
        let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "Invalid hex format")?;
        Ok((r, g, b))
    } else {
        Err("Hex color must be 6 characters")
    }
}


fn gb_to_mb(gb: f64) -> f64 {
    (gb as f64 * 1024.0).round()
}

fn bytes_to_gb(bytes: u64) -> f64 {
    (bytes as f64 / 1_073_741_824.0 * 10.0).round() / 10.0
}

fn get_color_code(color_name: &str) -> String {
    if let Some(hex) = color_name.strip_prefix('#') {
        if let Ok((r, g, b)) = parse_hex_color(hex) {
            return format!("\x1b[38;2;{};{};{}m", r, g, b);
        }
    }

    // Match for named colors
    match color_name {
        "bright_black" => "\x1b[90m".to_string(),
        "red" => "\x1b[31m".to_string(),
        "yellow" => "\x1b[33m".to_string(),
        "green" => "\x1b[32m".to_string(),
        "cyan" => "\x1b[36m".to_string(),
        "blue" => "\x1b[34m".to_string(),
        "magenta" => "\x1b[35m".to_string(),
        "black" => "\x1b[30m".to_string(),
        "bright_red" => "\x1b[91m".to_string(),
        "bright_green" => "\x1b[92m".to_string(),
        "bright_yellow" => "\x1b[93m".to_string(),
        "bright_blue" => "\x1b[94m".to_string(),
        "bright_magenta" => "\x1b[95m".to_string(),
        "bright_cyan" => "\x1b[96m".to_string(),
        "bright_white" => "\x1b[97m".to_string(),
        "white" => "\x1b[37m".to_string(),
        _ => "\x1b[0m".to_string(), 
    }
}


