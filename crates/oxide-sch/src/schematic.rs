use crate::instance::SchInstance;
use crate::wire::Wire;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SchematicView {
    pub instances: Vec<SchInstance>,
    pub wires: Vec<Wire>,
}
