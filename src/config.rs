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
    pub show_memory_usage: bool,
    pub show_memory_percent_used: bool,
    pub show_memory_percent_free: bool,
    pub show_memory_used: bool,
    pub show_memory_free: bool,
    pub show_memory_total: bool,
    pub show_os: bool,

}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppearanceSettings {
    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    pub fields: Vec<String>,
}

