use oxide_db::geometry::Point;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeviceType {
    Nmos,
    Pmos,
}

impl std::fmt::Display for DeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceType::Nmos => write!(f, "nMOS"),
            DeviceType::Pmos => write!(f, "pMOS"),
        }
    }
}

/// A recognised transistor.  Net fields index into `ExtractionResult::nets`.
/// Source and drain share the same net in v0.2 (single active region, not yet split).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub name: String,
    pub device_type: DeviceType,
    /// Index into ExtractionResult::nets for the gate (poly net).
    pub gate_net: Option<usize>,
    /// Index into ExtractionResult::nets for source/drain (active net).
    pub sd_net: Option<usize>,
    /// Centre of the poly × active overlap in lambda coordinates.
    pub location: Point,
}
