use std::process::Command;
use std::io::{self, BufRead};

fn main() {
    let username = get_user(); 
    let (hostname, os, architecture) = get_system_info();

    println!("Username: {}", username);
    println!("Hostname: {}", hostname);
    println!("Distro: {} {}", os, architecture);
    
    match detect_package_manager() {
        Some(manager) =>
        if let Some(count) = get_installed_packages(manager) {
            println!("Packages: {} ({})", count, manager);
        } 
        else {
            println!("Failed to get package count from {}", manager);
        }
        None => println!("No package manager found"),
    }
    
}


fn get_user() -> String {
    let output = {
        Command::new("sh")
        .arg("-c")
        .arg("echo $USER")
        .output()
        .expect("Failed to execute process")
    };
    
    String::from_utf8(output.stdout)
        .expect("Invalid UTF-8")
        .trim()
        .to_string()

}

fn get_system_info() -> (String, String, String) {
    let output = {
        Command::new("sh")
        .arg("-c")
        .arg("hostnamectl")
        .output()
        .expect("Failed to execute process")
    };

    let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");

    let extract_field =  |field: &str| {
        stdout
            .lines()
            .find(|line| line.trim_start().starts_with(field))
            .and_then(|line| line.splitn(2, ':').nth(1))
            .map(|value| value.trim().to_string())
            .unwrap_or_else(|| "Not found".to_string())
    };

    let hostname = extract_field("Static hostname");
    let os = extract_field("Operating System");
    let architecture = extract_field("Architecture");

    (hostname, os, architecture)
}

fn detect_package_manager() -> Option<&'static str> {
    let managers = [
    ("pacman", "pacman"),
    ("dpkg", "dpkg"),
    ("rpm", "rpm"),
    ("zypper", "zypper")
    ];

    for (name, cmd) in &managers {
        if Command::new(cmd).output().is_ok() {
            return Some(name);
        }
    }
    None
}

fn get_installed_packages(manager: &str) -> Option<String> {
    let command = match manager {
        "pacman" => Command::new("pacman").args(&["-Qq"]).output(),
        "dpkg" => Command::new("dpkg").args(&["--get-selections"]).output(),
        "rpm" => Command::new("rpm").args(&["-qa"]).output(),
        "zypper" => Command::new("zypper").args(&["se", "--installed-only"]).output(),
        _ => return None,
    };

    match command {
        Ok(output) if output.status.success() => {
            Some(output.stdout.lines().count().to_string())
        }
        _ => None,
    }
}

