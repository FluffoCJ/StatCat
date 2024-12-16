use crate::config::*;
use battery::Manager;
use chrono::Local;
use nixinfo;
use std::collections::HashMap;
use sysinfo::System;

mod config;
mod fetch;
mod packages;

fn main() {
    let mut system = System::new();
    system.refresh_memory();

    let distro = nixinfo::distro()
        .unwrap_or_default()
        .trim_matches('"')
        .to_string();
    let packages = packages::get_package_count().to_string();
    let hostname = fetch::get_hostname().unwrap_or_default();
    let cpu = fetch::get_cpu().unwrap_or_default();

    let variables: HashMap<&str, String> = HashMap::from([
        ("hostname", hostname),
        ("cpu", cpu),
        ("distro", distro),
        ("packages", packages),
    ]);

    if let Err(e) = print_config(&variables) {
        eprintln!("Error: {}", e);
    }
}

fn print_config(variables: &HashMap<&str, String>) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = "config.toml";
    let config_content = load_config(config_path)?;
    let config: Config = toml::from_str(&config_content)?;

    for line in &config.config.output {
        let mut rendered_line = line.clone();

        // Replace variables
        for (key, value) in variables {
            let placeholder = format!("{{{}}}", key);
            rendered_line = rendered_line.replace(&placeholder, value);
        }

        // Replace colors
        for (key, value) in &config.colors {
            let placeholder = format!("{{{}}}", key);
            rendered_line = rendered_line.replace(&placeholder, value);
        }

        println!("{}", rendered_line);
    }
    Ok(())
}
