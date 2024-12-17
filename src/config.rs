use serde::{Deserialize, Serialize};
use std::collections::HashMap;


#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub config: OutputConfig,
    #[serde(default)]
    pub general: General,
    #[serde(default)]
    pub colors: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct General {
    #[serde(default)]
    pub figlet: bool,
    #[serde(default)]
    pub figlet_text: String,
    #[serde(default)]
    pub figlet_arg: String,
    #[serde(default)]
    pub figlet_color: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct OutputConfig {
    pub output: Vec<String>,
}



