use crate::layer_map::lambda_cmos_layer_map;
use gds21::{GdsElement, GdsLibrary};
use oxide_db::cell::LayoutView;
use oxide_db::geometry::{Point, Rect};
use oxide_db::label::Label;
use oxide_db::shape::Shape;
use std::collections::HashMap;
use std::path::Path;

/// Inverse of export: 1 db-unit = 1 nm, 1λ = 1000 db-units → divide by 1000.
const GDS_TO_LAMBDA: f64 = 1.0 / 1000.0;

/// Import a GDSII file, returning the first struct as a LayoutView.
/// Only GdsBoundary (rectangles approximated by bbox) and GdsTextElem are imported.
pub fn import_gds(path: &Path) -> Result<LayoutView, Box<dyn std::error::Error>> {
    let lib = GdsLibrary::load(path)?;

    let layer_map = lambda_cmos_layer_map();
    // Reverse: (gds_layer, datatype) → oxide layer name
    let reverse: HashMap<(i16, i16), &str> = layer_map
        .iter()
        .map(|(name, &nums)| (nums, name.as_str()))
        .collect();

    let mut layout = LayoutView::default();

    // Use the first struct found (top-level cell)
    let Some(gds_struct) = lib.structs.first() else {
        return Ok(layout);
    };

    for elem in &gds_struct.elems {
        match elem {
            GdsElement::GdsBoundary(b) => {
                let Some(&layer_name) = reverse.get(&(b.layer, b.datatype)) else {
                    continue;
                };
                if b.xy.is_empty() {
                    continue;
                }
                let xs: Vec<f64> = b.xy.iter().map(|p| p.x as f64 * GDS_TO_LAMBDA).collect();
                let ys: Vec<f64> = b.xy.iter().map(|p| p.y as f64 * GDS_TO_LAMBDA).collect();
                let x0 = xs.iter().cloned().fold(f64::INFINITY, f64::min);
                let y0 = ys.iter().cloned().fold(f64::INFINITY, f64::min);
                let x1 = xs.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                let y1 = ys.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                if x1 <= x0 || y1 <= y0 {
                    continue;
                }
                let id = layout.next_shape_id();
                layout.add_shape(Shape::new_rect(
                    id,
                    layer_name.to_string(),
                    Rect::new(x0, y0, x1 - x0, y1 - y0),
                ));
            }
            GdsElement::GdsTextElem(t) => {
                let Some(&layer_name) = reverse.get(&(t.layer, 0)) else {
                    continue;
                };
                let x = t.xy.x as f64 * GDS_TO_LAMBDA;
                let y = t.xy.y as f64 * GDS_TO_LAMBDA;
                layout.labels.push(Label {
                    text: t.string.clone(),
                    layer: layer_name.to_string(),
                    position: Point { x, y },
                    net: None,
                });
            }
            _ => {}
        }
    }

    Ok(layout)
}
