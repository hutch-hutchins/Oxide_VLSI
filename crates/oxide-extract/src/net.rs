use oxide_db::shape::ShapeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Net {
    pub name: Option<String>,
    pub shape_ids: Vec<ShapeId>,
}
