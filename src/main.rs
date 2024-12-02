#![allow(unused)]
use std::process::Command;
use std::io::{BufRead, Read};
use sysinfo::{System, RefreshKind, CpuRefreshKind};
use toml;
use std::path::PathBuf;
use std::fs::File;
use crate::config::{Config, General, AppearanceSettings, Order};

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
                if config.general.hostname {
                    println!("Hostname: {}", System::host_name().unwrap_or_default());
                }

            }
            "cpu" => {
                if config.general.show_cpu {
                    fetch::print_cpu_brand();
                }
            }
            "memory_usage" => {
                if config.general.show_memory_usage {
                    let used_memory_gb = bytes_to_gb(system.used_memory());
                    let total_memory_gb = bytes_to_gb(system.total_memory());
                    println!("Memory: {}GB/{}GB", used_memory_gb, total_memory_gb); 
                }
            }
            "memory_percent_used" => {
                if config.general.show_memory_percent_used {
                    let used_memory_gb = bytes_to_gb(system.used_memory());
                    let total_memory_gb = bytes_to_gb(system.total_memory()); 
                    let used_memory_percent_used = used_memory_gb / total_memory_gb * 100.0;
                    println!("Memory used: {}%", used_memory_percent_used.round());
                }
            }
            "os" => {
                if config.general.show_os {
                    println!("OS: {}", System::host_name().unwrap_or_default());
                }
            }
            "packages" => {
                if config.general.show_packages {
                    fetch::print_package();
                }
            }
            "shell" => {
                if config.general.show_shell {
                    println!("Shell: {}", fetch::get_shell());
                }
            }
            _ => {
                    println!("Unknown field: {}", field);
                }
        }
    }



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
            general: General {
                show_shell: true,
                hostname: true,
                show_cpu: true,
                show_memory_usage: true,
                show_memory_percent_used: false,
                show_memory_percent_free: false,
                show_memory_total: false,
                show_memory_used: false,
                show_memory_free: false,
                show_os: true,
                show_packages: true,
            },
            appearance: AppearanceSettings {

            },
            order: Order {
                fields: vec!["hostname".to_string(), "cpu".to_string(), "memory".to_string(), "os".to_string()],
            },
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


