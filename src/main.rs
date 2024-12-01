use std::process::Command;
use std::io::{self, BufRead};
use sysinfo::{System, RefreshKind, CpuRefreshKind};

fn main() {
    let username = get_user(); 
    let desktop = get_desktop();
    //let (hostname, os, architecture, kernel) = get_system_info();

    println!("Username: {}", username);
    println!("Kernel: {}", System::kernel_version().unwrap_or_default());
    println!("System host name: {}", System::host_name().unwrap_or_default());
    print_package(); 
    print_cpu_brand();
    println!("Desktop: {}", desktop);

    
}

fn print_cpu_brand() {
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

fn print_package() {
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

fn get_desktop() -> String {
    let output = {
        Command::new("sh")
            .arg("-c")
            .arg("echo $XDG_CURRENT_DESKTOP")
            .output()
            .expect("Failed to execute process")
    };

    String::from_utf8(output.stdout)
        .expect("Invalid UTF-8")
        .trim()
        .to_string()

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
