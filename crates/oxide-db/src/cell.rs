use crate::geometry::Rect;
use crate::label::Label;
use crate::shape::{Shape, ShapeId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Port {
    pub name: String,
    pub layer: String,
    pub rect: Rect,
    pub direction: PortDir,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortDir {
    Input,
    Output,
    Inout,
    Power,
    Ground,
}

/// A placed instance of another cell inside this cell's layout.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instance {
    pub cell_name: String,
    pub origin: crate::geometry::Point,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LayoutView {
    pub shapes: Vec<Shape>,
    pub instances: Vec<Instance>,
    pub labels: Vec<Label>,
    pub ports: Vec<Port>,
    /// Next available shape ID
    #[serde(skip)]
    next_id: u64,
}

impl LayoutView {
    pub fn next_shape_id(&mut self) -> ShapeId {
        let id = ShapeId(self.next_id);
        self.next_id += 1;
        id
    }

    pub fn add_shape(&mut self, shape: Shape) {
        self.shapes.push(shape);
    }

    pub fn remove_shape(&mut self, id: ShapeId) {
        self.shapes.retain(|s| s.id != id);
    }

    pub fn bounding_rect(&self) -> Option<Rect> {
        if self.shapes.is_empty() {
            return None;
        }
        let first = self.shapes[0].bounding_rect();
        let mut x0 = first.x;
        let mut y0 = first.y;
        let mut x1 = first.x1();
        let mut y1 = first.y1();
        for s in &self.shapes[1..] {
            let r = s.bounding_rect();
            x0 = x0.min(r.x);
            y0 = y0.min(r.y);
            x1 = x1.max(r.x1());
            y1 = y1.max(r.y1());
        }
        Some(Rect::from_corners(x0, y0, x1, y1))
    }

    /// Shapes on a specific layer.
    pub fn shapes_on_layer<'a>(&'a self, layer: &'a str) -> impl Iterator<Item = &'a Shape> {
        self.shapes.iter().filter(move |s| s.layer == layer)
    }

    pub fn translate_shapes(&mut self, ids: &[ShapeId], dx: f64, dy: f64) {
        for shape in &mut self.shapes {
            if ids.contains(&shape.id) {
                shape.geometry = shape.geometry.translated(dx, dy);
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CellMetadata {
    pub description: String,
    pub technology: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cell {
    pub name: String,
    pub layout: LayoutView,
    pub metadata: CellMetadata,
}

impl Cell {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            layout: LayoutView::default(),
            metadata: CellMetadata::default(),
        }
    }
}
