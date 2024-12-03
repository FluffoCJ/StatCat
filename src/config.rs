use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub memory: Memory,
    #[serde(default)]
    pub hostname: HostName,
    #[serde(default)]
    pub cpu: Cpu,
    #[serde(default)]
    pub packages: Packages,
    #[serde(default)]
    pub order: Order,
    #[serde(default)]
    pub os: OS,
    #[serde{default}]
    pub gpu: Gpu,
    #[serde{default}]
    pub shell: Shell,
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
pub struct Shell {
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

