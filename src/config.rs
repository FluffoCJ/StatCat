use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub hostname: HostName,
    pub general: General,
    pub appearance: AppearanceSettings,
    pub order: Order,
}

#[derive(Serialize, Deserialize, Default)]
pub struct HostName {
    #[serde(default)]
    pub icon: String,
    #[serde(default)]
    pub text: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Cpu {
    #[serde(default)]
    pub icon: String,
    #[serde(default)]
    pub text: String,

}



#[derive(Serialize, Deserialize, Default)]
pub struct General {
}

#[derive(Serialize, Deserialize, Default)]
pub struct AppearanceSettings {
    
}

#[derive(Serialize, Deserialize, Default)]
pub struct Order {
    pub fields: Vec<String>,
}

