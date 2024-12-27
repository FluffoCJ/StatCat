use crate::Config;
use std::process::Command;
pub fn hex_to_ansi(hex: &str) -> String {
    let hex = hex.trim_start_matches('#');

    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);

    format!("\u{001b}[38;2;{};{};{}m", r, g, b)
}

pub fn get_figlet(config: &Config) -> Result<String, String> {
    let output = Command::new("figlet")
        .arg(&config.general.figlet_text)
        .arg(&config.general.figlet_arg)
        .output();

    match output {
        Ok(output) => {
            if !output.stdout.is_empty() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            } else {
                Err("No output from the command".to_string())
            }
        }
        Err(e) => Err(format!("Error running command: {}", e)),
    }
}
