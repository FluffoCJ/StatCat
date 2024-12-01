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
    //let (hostname, os, architecture, kernel) = get_system_info();
    

    let config = load_config();
        println!("Loaded Configuration: {:?}", config);

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
            "memory" => {
                if config.general.show_memory {
                    let used_memory_gb = bytes_to_gb(system.used_memory());
                    let total_memory_gb = bytes_to_gb(system.total_memory());
                    println!("Memory: {}GB/{}GB", used_memory_gb, total_memory_gb); 
                }
            }
            "os" => {
                if config.general.show_os {
                    println!("OS: {}", System::host_name().unwrap_or_default());
                }
            }
            _ => {
                    println!("Unknown field: {}", field);
                }
        }
    }


    //println!("Username: {}", username);
    //println!("Kernel: {}", System::kernel_version().unwrap_or_default());
    //println!("System host name: {}", System::host_name().unwrap_or_default());
    //print_package(); 
    //print_cpu_brand();
    //println!("Desktop: {}", desktop);
    //
    //let used_memory_gb = bytes_to_gb(system.used_memory());
    //let total_memory_gb = bytes_to_gb(system.total_memory());
    //println!("Memory: {}GB/{}GB", used_memory_gb, total_memory_gb);
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
                hostname: true,
                show_cpu: true,
                show_memory: true,
                show_os: true,
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


//fn get_system_info() -> (String, String, String, String) {
//    let output = {
//        Command::new("sh")
//            .arg("-c")
//            .arg("hostnamectl")
//            .output()
//            .expect("Failed to execute process")
//    };
//
//    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
//
//    let extract_field =  |field: &str| {
//        stdout
//            .lines()
//            .find(|line| line.trim_start().starts_with(field))
//            .and_then(|line| line.splitn(2, ':').nth(1))
//            .map(|value| value.trim().to_string())
//            .unwrap_or_else(|| "Not found".to_string())
//    };
//
//    let hostname = extract_field("Static hostname");
//    let os = extract_field("Operating System");
//    let architecture = extract_field("Architecture");
//    let kernel = extract_field("Kernel");
//
//    (hostname, os, architecture, kernel)
//}
