use crate::cell::Cell;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Library {
    pub name: String,
    pub cells: HashMap<String, Cell>,
}

impl Library {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), cells: HashMap::new() }
    }

    pub fn add_cell(&mut self, cell: Cell) {
        self.cells.insert(cell.name.clone(), cell);
    }

    pub fn cell(&self, name: &str) -> Option<&Cell> {
        self.cells.get(name)
    }

    pub fn cell_mut(&mut self, name: &str) -> Option<&mut Cell> {
        self.cells.get_mut(name)
    }
}
