use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub general: General,
    pub appearance: AppearanceSettings,
    pub order: Order,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct General {
    #[serde(default)]
    pub hostname: bool,
    #[serde(default)]
    pub show_cpu: bool,
    #[serde(default)]
    pub show_memory_usage: bool,
    #[serde(default)]
    pub show_memory_percent_used: bool,
    #[serde(default)]
    pub show_memory_percent_free: bool,
    #[serde(default)]
    pub show_memory_used: bool,
    #[serde(default)]
    pub show_memory_free: bool,
    #[serde(default)]
    pub show_memory_total: bool,
    #[serde(default)]
    pub show_packages: bool,
    #[serde(default)]
    pub show_os: bool,

}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppearanceSettings {
    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    pub fields: Vec<String>,
}

