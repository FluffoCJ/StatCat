#![allow(unused)]
use std::process::Command;
use std::io::{BufRead, Read};
use sysinfo::{System, RefreshKind, CpuRefreshKind};
use toml;
use std::path::PathBuf;
use std::fs::File;
use nixinfo::cpu;
use nixinfo::packages;
use crate::config::*;

mod fetch;
mod config;

fn main() {
    let system = System::new_all();


    let username = fetch::get_user(); 
    let desktop = fetch::get_desktop();
    



    let config = load_config();

    for field in &config.order.fields {
        match field.as_str() {
            "hostname" => {
                let mut icon = config.hostname.icon.clone();
                let text = &config.hostname.text;
                push_icon(icon.clone());
                println!("{icon}{text}: {}", System::host_name().unwrap_or_default());

            }
            "cpu" => {
                let cpu = nixinfo::cpu();
                println!("CPU: {}", nixinfo::cpu().unwrap_or_default());
            }
            "memory_usage" => {
                let used_memory_gb = bytes_to_gb(system.used_memory());
                let total_memory_gb = bytes_to_gb(system.total_memory());
                println!("Memory: {}GB/{}GB", used_memory_gb, total_memory_gb); 
            }
            // TODO: move this to config option
            //"memory_percent_used" => {
            //    let used_memory_gb = bytes_to_gb(system.used_memory());
            //    let total_memory_gb = bytes_to_gb(system.total_memory()); 
            //    let used_memory_percent_used = used_memory_gb / total_memory_gb * 100.0;
            //    println!("Memory used: {}%", used_memory_percent_used.round());
            //}
            "total_memory" => {
                let total_memory_gb = bytes_to_gb(system.total_memory());
                println!("Memory: {}GB", total_memory_gb); 
            }
            "used_memory" => {
                let used_memory_gb = bytes_to_gb(system.used_memory());
                println!("Used Memory: {}GB", used_memory_gb);
            }
            "os" => {
                println!("OS: {}", System::host_name().unwrap_or_default());
            }
            "packages" => {
                let manager = fetch::detect_package_manager();
                println!("Packages: {}", nixinfo::packages(manager).unwrap_or_default());
            }
            "shell" => {
                println!("Shell: {}", fetch::get_shell());
            }
            "gpu" => {

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
            ..Default::default() // Use Serde defaults for the entire structure
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


