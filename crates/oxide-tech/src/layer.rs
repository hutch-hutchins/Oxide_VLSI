use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LayerPurpose {
    Well,
    Diffusion,
    Gate,
    Contact,
    Routing,
    Via,
    Label,
    Substrate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerDef {
    pub name: String,
    /// RGBA color for canvas rendering
    pub color: [u8; 4],
    pub purpose: LayerPurpose,
    /// Draw order: higher z_order renders on top
    pub z_order: u32,
    /// Whether this layer is selectable by default
    pub selectable: bool,
}
