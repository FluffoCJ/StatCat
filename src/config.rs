use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub general: General,
    pub appearance: AppearanceSettings,
    pub order: Order,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct General {
    pub hostname: bool,
    pub show_cpu: bool,
    pub show_memory: bool,
    pub show_os: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppearanceSettings {
    // Add fields here as necessary
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    pub fields: Vec<String>,
}

