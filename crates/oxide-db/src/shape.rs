use crate::geometry::{Geometry, Rect};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ShapeId(pub u64);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shape {
    pub id: ShapeId,
    pub layer: String,
    pub geometry: Geometry,
    /// Net name assigned by a label or connectivity extraction
    pub net: Option<String>,
    /// Whether this shape represents a pMOS active region (used for pMOS-in-nwell DRC)
    pub is_pmos: bool,
}

impl Shape {
    pub fn new_rect(id: ShapeId, layer: impl Into<String>, rect: Rect) -> Self {
        Self {
            id,
            layer: layer.into(),
            geometry: Geometry::Rect(rect),
            net: None,
            is_pmos: false,
        }
    }

    pub fn bounding_rect(&self) -> Rect {
        self.geometry.bounding_rect()
    }
}
