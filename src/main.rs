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
        match field.as_str() {
            "hostname" => {
                let icon = config.hostname.icon.clone();
                let text = &config.hostname.text;
                push_icon(icon.clone());
                println!("{icon}{text}: {}", System::host_name().unwrap_or_default());

            }
            "cpu" => {
                let icon = config.cpu.icon.clone();
                let text = &config.cpu.text;
                push_icon(icon.clone());
                println!("{icon}{text}: {}", nixinfo::cpu().unwrap_or_default());
            }
            "memory" => {
                // TODO: Add percentage and mb bool
                // TODO: Add config to display separate used, free, and total memory
                let used_memory_gb = bytes_to_gb(system.used_memory());
                let total_memory_gb = bytes_to_gb(system.total_memory());
                let text = &config.memory.text;
                let icon = config.memory.icon.clone();
                push_icon(icon.clone());
                println!("{icon}{text}: {}GB/{}GB", used_memory_gb, total_memory_gb); 
            }
            "os" => {
                let text = &config.os.text;
                let icon = config.os.icon.clone();
                let distro = nixinfo::distro().unwrap_or_default();
                let distro = distro.trim_matches('"');
                push_icon(icon.clone());
                println!("{icon}{text}: {}", distro);
            }
            "packages" => {
                let manager = fetch::detect_package_manager();
                let text = &config.packages.text;
                let icon = config.packages.icon.clone();
                push_icon(icon.clone());
                println!("{icon}{text}: {}", nixinfo::packages(manager).unwrap_or_default());
            }
            "shell" => {
                let text = &config.shell.text;
                let icon = config.shell.icon.clone();
                push_icon(icon.clone());
                println!("{icon}{text}: {}", fetch::get_shell());
            }
            "gpu" => {
                let text = &config.gpu.text;
                let icon = config.gpu.icon.clone();
                push_icon(icon.clone());
                println!("{icon}{text}: {}", nixinfo::gpu().unwrap_or_default());
            }
            "terminal" => {
                let text = &config.terminal.text;
                let icon = config.terminal.icon.clone();
                push_icon(icon.clone());
                println!("{icon}{text}: {}", nixinfo::terminal().unwrap_or_default());
            }
            "uptime" => {
                let text = &config.uptime.text;
                let icon = config.uptime.icon.clone();
                push_icon(icon.clone());
                println!("{icon}{text}: {}", nixinfo::uptime().unwrap_or_default());
            }
            "desktop" => {
                let text = &config.desktop.text;
                let icon = config.desktop.icon.clone();
                push_icon(icon.clone());
                println!("{icon}{text}: {}", nixinfo::environment().unwrap_or_default());
            }
            _ => {
                    println!("Unknown field: {}", field);
                }
        }
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



fn bytes_to_gb(bytes: u64) -> f64 {
    (bytes as f64 / 1_073_741_824.0 * 10.0).round() / 10.0
}


