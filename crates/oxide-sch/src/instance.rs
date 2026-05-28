use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchInstance {
    pub symbol: String,
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub port_connections: std::collections::HashMap<String, String>,
}
