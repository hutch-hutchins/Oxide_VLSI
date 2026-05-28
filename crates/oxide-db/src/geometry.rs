use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// Axis-aligned rectangle in lambda coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rect {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self { x, y, width, height }
    }

    pub fn from_corners(x0: f64, y0: f64, x1: f64, y1: f64) -> Self {
        let (x, w) = if x0 <= x1 { (x0, x1 - x0) } else { (x1, x0 - x1) };
        let (y, h) = if y0 <= y1 { (y0, y1 - y0) } else { (y1, y0 - y1) };
        Self { x, y, width: w, height: h }
    }

    pub fn x1(&self) -> f64 { self.x + self.width }
    pub fn y1(&self) -> f64 { self.y + self.height }
    pub fn min_dim(&self) -> f64 { self.width.min(self.height) }

    pub fn contains_rect(&self, other: &Rect) -> bool {
        self.x <= other.x
            && self.y <= other.y
            && self.x1() >= other.x1()
            && self.y1() >= other.y1()
    }

    pub fn intersects(&self, other: &Rect) -> bool {
        self.x < other.x1()
            && self.x1() > other.x
            && self.y < other.y1()
            && self.y1() > other.y
    }

    pub fn touches(&self, other: &Rect) -> bool {
        self.x <= other.x1()
            && self.x1() >= other.x
            && self.y <= other.y1()
            && self.y1() >= other.y
    }

    /// Gap between two non-overlapping rects (negative means overlap).
    pub fn gap_to(&self, other: &Rect) -> f64 {
        let dx = (other.x - self.x1()).max(self.x - other.x1()).max(0.0);
        let dy = (other.y - self.y1()).max(self.y - other.y1()).max(0.0);
        if dx > 0.0 || dy > 0.0 {
            dx.max(dy)
        } else {
            // overlapping: return negative of overlap area diagonal
            -1.0
        }
    }

    /// Minimum edge-to-edge separation (0 if touching/overlapping).
    pub fn min_separation(&self, other: &Rect) -> f64 {
        let gap_x = (other.x - self.x1()).max(self.x - other.x1()).max(0.0);
        let gap_y = (other.y - self.y1()).max(self.y - other.y1()).max(0.0);
        if gap_x > 0.0 && gap_y > 0.0 {
            // diagonal gap — use the smaller axis distance for DRC purposes
            gap_x.min(gap_y)
        } else {
            gap_x + gap_y
        }
    }

    pub fn center(&self) -> Point {
        Point::new(self.x + self.width / 2.0, self.y + self.height / 2.0)
    }

    /// Grow the rect outward by `amount` on all sides.
    pub fn expanded(&self, amount: f64) -> Rect {
        Rect {
            x: self.x - amount,
            y: self.y - amount,
            width: self.width + 2.0 * amount,
            height: self.height + 2.0 * amount,
        }
    }

    /// Shrink inward by `amount`; returns None if the rect would collapse.
    pub fn shrunk(&self, amount: f64) -> Option<Rect> {
        if self.width < 2.0 * amount || self.height < 2.0 * amount {
            return None;
        }
        Some(Rect {
            x: self.x + amount,
            y: self.y + amount,
            width: self.width - 2.0 * amount,
            height: self.height - 2.0 * amount,
        })
    }

    pub fn translated(&self, dx: f64, dy: f64) -> Rect {
        Rect { x: self.x + dx, y: self.y + dy, ..*self }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Geometry {
    Rect(Rect),
    // Future: Path, Polygon
}

impl Geometry {
    pub fn bounding_rect(&self) -> Rect {
        match self {
            Geometry::Rect(r) => *r,
        }
    }

    pub fn translated(&self, dx: f64, dy: f64) -> Geometry {
        match self {
            Geometry::Rect(r) => Geometry::Rect(r.translated(dx, dy)),
        }
    }
}
