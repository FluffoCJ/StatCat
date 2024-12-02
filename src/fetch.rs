use std::process::Command;
use std::io::BufRead;
use sysinfo::{System, RefreshKind, CpuRefreshKind};




pub fn print_cpu_brand() {
    let s = System::new_with_specifics(
        RefreshKind::new().with_cpu(CpuRefreshKind::everything()),
    );
    if let Some(cpu) = s.cpus().iter().next() {
        println!("Processor: {}", cpu.brand());
    } 
    else {
        println!("No CPUs found");
    }
}


pub fn print_package() {
    match detect_package_manager() {
        Some(manager) =>
        if let Some(count) = get_installed_packages(manager) {
            println!("Packages: {} ({})", count, manager);
        } 
        else {
            println!("Failed to get package count from {}", manager);
        }
        none => println!("No package manager found"),
    }
}

pub fn get_shell() -> String {
    let output = Command::new("ps")
        .arg("-p")
        .arg(std::process::id().to_string())
        .arg("-o")
        .arg("ppid=")
        .output()
        .expect("Failed to execute ps");

    let parent_pid = String::from_utf8_lossy(&output.stdout).trim().to_string();

    let output = Command::new("ps")
        .arg("-p")
        .arg(parent_pid)
        .arg("-o")
        .arg("comm=")
        .output()
        .expect("Failed to execute ps");

    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

pub fn detect_package_manager() -> Option<&'static str> {
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

pub fn get_installed_packages(manager: &str) -> Option<String> {
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


pub fn get_desktop() -> String {
    std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_else(|_| "Unknown".to_string())
}

pub fn get_user() -> String {
    std::env::var("USER").unwrap_or_else(|_| "Unknown".to_string())
}
