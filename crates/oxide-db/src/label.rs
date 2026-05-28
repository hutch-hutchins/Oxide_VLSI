use crate::geometry::Point;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Label {
    pub text: String,
    pub layer: String,
    pub position: Point,
    /// If Some, this label assigns a net name; used during extraction.
    pub net: Option<String>,
}

impl Label {
    pub fn new(text: impl Into<String>, layer: impl Into<String>, position: Point) -> Self {
        Self {
            text: text.into(),
            layer: layer.into(),
            position,
            net: None,
        }
    }
}
