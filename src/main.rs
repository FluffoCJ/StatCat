use crate::config::*;
use home::home_dir;
use itertools::Itertools;
use nixinfo;
use std::{collections::HashMap, error::Error, fs, io};

mod config;
mod fetch;
mod fetch_hw;
mod misc;
mod packages;

fn main() -> Result<(), Box<dyn Error>> {
    let config = load_config()?;
    if config.general.figlet {
        let mut figlet_color = config.general.figlet_color.clone().unwrap_or_default();
        let figlet_text = misc::get_figlet(&config).unwrap_or_default();
        let figlet = figlet_text
            .lines()
            .take(figlet_text.lines().count() - 1)
            .collect::<Vec<_>>()
            .join("\n");
        if figlet_color.starts_with("#") {
            figlet_color = misc::hex_to_ansi(&figlet_color);
        }
        println!("{figlet_color}{figlet} \x1b[0m");
    }

    print_config(&config)?;

    Ok(())
}

fn print_config(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    for pair in config
        .config
        .output
        .iter()
        .zip_longest(config.general.ascii.iter())
    {
        let (line, ascii) = match pair {
            itertools::EitherOrBoth::Both(line, ascii) => (line.clone(), ascii.clone()),
            itertools::EitherOrBoth::Left(line) => (line.clone(), String::new()),
            itertools::EitherOrBoth::Right(ascii) => (String::new(), ascii.clone()),
        };

        let mut line = line;
        let mut ascii = ascii;
        for (key, value) in &config.variables {
            let placeholder = format!("{{{}}}", key);
            if value.starts_with("#") {
                let ansi_color = misc::hex_to_ansi(value);
                line = line.replace(&placeholder, &ansi_color);
                ascii = ascii.replace(&placeholder, &ansi_color);
            } else {
                line = line.replace(&placeholder, value);
                ascii = ascii.replace(&placeholder, value);
            }
        }

        let mut replacements: HashMap<&str, fn() -> String> = HashMap::new();
        replacements.insert("{os}", || fetch::get_distro().unwrap_or_default());
        replacements.insert("{hostname}", || fetch::get_hostname().unwrap_or_default());
        replacements.insert("{cpu}", || fetch_hw::get_cpu().unwrap_or_default());
        replacements.insert("{packages}", || packages::get_package_count().to_string());
        replacements.insert("{kernel}", || fetch::get_kernel());
        replacements.insert("{terminal}", || nixinfo::terminal().unwrap_or_default());
        replacements.insert("{uptime}", || fetch::get_uptime().unwrap_or_default());
        replacements.insert("{username}", || fetch::get_user());
        replacements.insert("{shell}", || fetch::get_shell());
        replacements.insert("{desktop}", || fetch::get_desktop());
        replacements.insert("{ip}", || fetch::get_local_ip());
        replacements.insert("{package_manager}", || {
            packages::detect_package_manager().to_string()
        });
        replacements.insert("{session_type}", || fetch::get_session_type());

        // Free Memory
        replacements.insert("{free_mem_gb}", || {
            fetch_hw::get_memory().free_gb.to_string()
        });
        replacements.insert("{free_mem_mb}", || {
            fetch_hw::get_memory().free_mb.to_string()
        });
        replacements.insert("{free_mem_kb}", || {
            fetch_hw::get_memory().free_kb.to_string()
        });
        // Used Memory
        replacements.insert("{used_mem_gb}", || {
            fetch_hw::get_memory().used_gb.to_string()
        });
        replacements.insert("{used_mem_mb}", || {
            fetch_hw::get_memory().used_mb.to_string()
        });
        replacements.insert("{used_mem_kb}", || {
            fetch_hw::get_memory().used_kb.to_string()
        });
        // Total Memory
        replacements.insert("{total_mem_gb}", || {
            fetch_hw::get_memory().total_gb.to_string()
        });
        replacements.insert("{total_mem_mb}", || {
            fetch_hw::get_memory().total_mb.to_string()
        });
        replacements.insert("{total_mem_kb}", || {
            fetch_hw::get_memory().total_kb.to_string()
        });

        // Storage
        replacements.insert("{total_storage}", || {
            fetch_hw::get_storage().total_storage.to_string()
        });
        replacements.insert("{used_storage}", || {
            fetch_hw::get_storage().used_storage.to_string()
        });
        replacements.insert("{free_storage}", || {
            fetch_hw::get_storage().free_storage.to_string()
        });

        for (placeholder, fetch_func) in &replacements {
            if line.contains(placeholder) {
                line = line.replace(placeholder, &fetch_func());
            }
        }

        println!("{} {} \x1b[0m", ascii, line);
    }

    Ok(())
}

pub fn load_config() -> Result<Config, io::Error> {
    if let Some(home_path) = home_dir() {
        let config_path = home_path.join(".config/statcat/config.toml");

        let config_str = fs::read_to_string(config_path)?;

        match toml::de::from_str::<Config>(&config_str) {
            Ok(config) => Ok(config),
            Err(e) => {
                eprintln!("Error parsing TOML: {}", e);
                Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Failed to parse TOML",
                ))
            }
        }
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Home directory not found",
        ))
    }
}
