#![allow(unused)]
use std::io::Read;
use sysinfo::System;
use toml;
use std::path::PathBuf;
use std::fs::File;
use nixinfo;
use crate::config::*;


mod fetch;
mod config;

fn main() {
    let system = System::new_all();
    let config = load_config();

    for field in &config.order.fields {
        if let Some((text, icon)) = get_icon_text(&config, field) {
            match field.as_str() {
                // TODO: Add memory_free, memory_total, and memory_used modules
                "memory" => {
                    // TODO: Add display_percent bool
                    let used_memory = bytes_to_gb(system.used_memory());
                    let total_memory = bytes_to_gb(system.total_memory());
                    push_icon(icon.clone());

                    if config.memory.display_mb {
                        let used_memory = gb_to_mb(used_memory);
                        let total_memory = gb_to_mb(total_memory);
                        println!("{icon}{text}: {}MB/{}MB", used_memory, total_memory);
                    }
                    else {
                        println!("{icon}{text}: {}GB/{}GB", used_memory, total_memory);
                    }
                }
                "os" => {
                    let distro = nixinfo::distro().unwrap_or_default().trim_matches('"').to_string();
                    push_icon(icon.clone());
                    println!("{icon}{text}: {}", distro);
                }
                _ => {
                    let value = match field.as_str() {
                        "hostname" => System::host_name().unwrap_or_default(),
                        "cpu" => nixinfo::cpu().unwrap_or_default(),
                        "packages" => nixinfo::packages(fetch::detect_package_manager()).unwrap_or_default(),
                        "shell" => fetch::get_shell(),
                        "gpu" => nixinfo::gpu().unwrap_or_default(),
                        "terminal" => nixinfo::terminal().unwrap_or_default(),
                        "uptime" => nixinfo::uptime().unwrap_or_default(),
                        "desktop" => nixinfo::environment().unwrap_or_default(),
                        "username" => fetch::get_user().to_string(),
                        _ => continue,
                    };
                    push_icon(icon.clone());
                    println!("{icon}{text}: {}", value);
                }
            }
        } else {
            println!("Unknown field: {}", field);
        }
    }
}

fn gb_to_mb(gb: f64) -> f64 {
    (gb as f64 * 1024.0).round()
}

fn bytes_to_gb(bytes: u64) -> f64 {
    (bytes as f64 / 1_073_741_824.0 * 10.0).round() / 10.0
}


fn get_icon_text<'a>(config: &'a Config, field: &'a str) -> Option<(&'a str, String)> {
    match field {
        "os" => Some((&config.os.text, config.os.icon.clone())),
        "cpu" => Some((&config.cpu.text, config.cpu.icon.clone())),
        "memory" => Some((&config.memory.text, config.memory.icon.clone())),
        "hostname" => Some((&config.hostname.text, config.hostname.icon.clone())),
        "packages" => Some((&config.packages.text, config.packages.icon.clone())),
        "shell" => Some((&config.shell.text, config.shell.icon.clone())),
        "gpu" => Some((&config.gpu.text, config.gpu.icon.clone())),
        "terminal" => Some((&config.terminal.text, config.terminal.icon.clone())),
        "uptime" => Some((&config.uptime.text, config.uptime.icon.clone())),
        "desktop" => Some((&config.desktop.text, config.desktop.icon.clone())),
        "username" => Some((&config.username.text, config.username.icon.clone())), 
        _ => None,
    }
}



fn push_icon(mut icon: String) -> String {
    if !icon.is_empty() {
        icon.push(' ');
    }
    icon
}


fn get_config_path() -> PathBuf {
    let config_dir = dirs::config_dir().expect("Unable to determine the config directory");
    config_dir.join("fetchd").join("config.toml")
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




