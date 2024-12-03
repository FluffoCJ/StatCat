use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub hostname: HostName,
    #[serde(default)]
    pub cpu: Cpu,
    #[serde(default)]
    pub gpu: Gpu,
    #[serde(default)]
    pub terminal: Terminal,
    #[serde(default)]
    pub uptime: Uptime,
    #[serde(default)]
    pub os: OS,
    #[serde(default)]
    pub shell: Shell,
    #[serde(default)]
    pub packages: Packages,
    #[serde(default)]
    pub memory: Memory,
    #[serde(default)]
    pub desktop: Desktop,
    #[serde(default)]
    pub order: Order,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Memory {
    #[serde(default)]
    pub icon: String,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub percent: bool,
    #[serde(default)]
    pub display_mb: bool,
}

#[derive(Serialize, Deserialize, Default)]
pub struct OS {
    #[serde(default)]
    pub icon: String,
    #[serde(default)]
    pub text: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Gpu {
    #[serde(default)]
    pub icon: String,
    #[serde(default)]
    pub text: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Terminal {
    #[serde(default)]
    pub icon: String,
    #[serde(default)]
    pub text: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Uptime {
    #[serde(default)]
    pub icon: String,
    #[serde(default)]
    pub text: String,
}


#[derive(Serialize, Deserialize, Default)]
pub struct Shell {
    #[serde(default)]
    pub icon: String,
    #[serde(default)]
    pub text: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Desktop {
    #[serde(default)]
    pub icon: String,
    #[serde(default)]
    pub text: String,
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
pub struct Packages {
    #[serde(default)]
    pub icon: String,
    #[serde(default)]
    pub text: String,

}

#[derive(Serialize, Deserialize, Default)]
pub struct Order {
    pub fields: Vec<String>,
}

