use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeviceType {
    Nmos,
    Pmos,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Terminal {
    pub name: String,
    pub net: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub name: String,
    pub device_type: DeviceType,
    pub gate: Terminal,
    pub source: Terminal,
    pub drain: Terminal,
    pub body: Terminal,
}
