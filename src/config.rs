use serde::{Deserialize, Serialize};

macro_rules! define_struct {
    ($name:ident) => {
        #[derive(Serialize, Deserialize, Default)]
        pub struct $name {
            #[serde(default)]
            pub icon: String,
            #[serde(default)]
            pub text: String,
        }
    };
}

define_struct!(OS);
define_struct!(Gpu);
define_struct!(Terminal);
define_struct!(Shell);
define_struct!(Desktop);
define_struct!(HostName);
define_struct!(Cpu);


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
pub struct Uptime {
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


