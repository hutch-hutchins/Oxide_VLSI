use oxide_db::geometry::{Point, Rect};
use oxide_db::shape::ShapeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrcViolation {
    pub rule: String,
    /// Representative location (e.g. center of the offending shape)
    pub location: Point,
    /// Bounding rect of the violation for canvas highlight
    pub bbox: Option<Rect>,
    /// Involved shape IDs (for canvas selection)
    pub shapes: Vec<ShapeId>,
    /// What the rule requires (e.g. "poly width >= 2λ")
    pub required: String,
    /// What was found (e.g. "poly width = 1λ")
    pub found: String,
    /// Teaching explanation of why this rule exists
    pub explanation: String,
    /// Concrete fix suggestion
    pub fix_hint: String,
}

impl DrcViolation {
    pub fn new(
        rule: impl Into<String>,
        location: Point,
        bbox: Option<Rect>,
        shapes: Vec<ShapeId>,
        required: impl Into<String>,
        found: impl Into<String>,
        explanation: impl Into<String>,
        fix_hint: impl Into<String>,
    ) -> Self {
        Self {
            rule: rule.into(),
            location,
            bbox,
            shapes,
            required: required.into(),
            found: found.into(),
            explanation: explanation.into(),
            fix_hint: fix_hint.into(),
        }
    }
}
