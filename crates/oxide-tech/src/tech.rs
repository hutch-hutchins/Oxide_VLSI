use crate::layer::LayerDef;
use crate::rules::DesignRules;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Technology {
    pub name: String,
    /// Unit name displayed to students (e.g. "λ")
    pub unit: String,
    /// Grid resolution in lambda (e.g. 0.5 means half-lambda grid)
    pub grid: f64,
    pub layers: Vec<LayerDef>,
    pub rules: DesignRules,
}

impl Technology {
    pub fn load_toml(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let text = std::fs::read_to_string(path)?;
        let tech: Technology = toml::from_str(&text)?;
        Ok(tech)
    }

    pub fn load_toml_str(text: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let tech: Technology = toml::from_str(text)?;
        Ok(tech)
    }

    /// Return a map from layer name to LayerDef for quick lookup.
    pub fn layer_map(&self) -> HashMap<&str, &LayerDef> {
        self.layers.iter().map(|l| (l.name.as_str(), l)).collect()
    }

    pub fn layer(&self, name: &str) -> Option<&LayerDef> {
        self.layers.iter().find(|l| l.name == name)
    }
}
