use serde::{Deserialize, Serialize};

/// Minimum width rule: a shape on `layer` must be at least `min` lambda wide.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidthRule {
    pub layer: String,
    pub min: f64,
    pub description: String,
}

/// Minimum spacing rule between two shapes on the same (or different) layer(s).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpacingRule {
    pub layer_a: String,
    /// If None, applies to same-layer spacing (layer_a to layer_a)
    pub layer_b: Option<String>,
    pub min: f64,
    pub description: String,
}

/// The shape on `outer` must enclose the shape on `inner` by at least `min` lambda on all sides.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnclosureRule {
    pub outer: String,
    pub inner: String,
    pub min: f64,
    pub description: String,
}

/// A shape on `contained` must be geometrically inside a shape on `container`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainmentRule {
    /// Layer that must be contained (e.g. "active" of pMOS type)
    pub contained: String,
    /// Layer that must contain it (e.g. "nwell")
    pub container: String,
    /// Human-readable condition for when this rule applies
    pub condition: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DesignRules {
    pub width: Vec<WidthRule>,
    pub spacing: Vec<SpacingRule>,
    pub enclosure: Vec<EnclosureRule>,
    pub containment: Vec<ContainmentRule>,
}
