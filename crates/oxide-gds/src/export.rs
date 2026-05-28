use crate::layer_map::lambda_cmos_layer_map;
use gds21::{GdsBoundary, GdsElement, GdsLibrary, GdsPoint, GdsStruct, GdsTextElem, GdsUnits};
use oxide_db::cell::LayoutView;
use std::path::Path;

/// GDS integer coordinate units: 1 db-unit = 1 nm, 1 user-unit = 1 μm.
/// We treat 1λ = 1 μm → multiply lambda coords by 1000 to get db integers.
const LAMBDA_TO_GDS: f64 = 1000.0;

pub fn export_gds(
    layout: &LayoutView,
    cell_name: &str,
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let layer_map = lambda_cmos_layer_map();

    let mut gds_struct = GdsStruct {
        name: cell_name.into(),
        ..Default::default()
    };

    // Shapes → GdsBoundary (closed 5-point polygon for each rectangle)
    for shape in &layout.shapes {
        let Some(&(gds_layer, datatype)) = layer_map.get(&shape.layer) else {
            continue;
        };
        let r = shape.bounding_rect();
        let x0 = (r.x * LAMBDA_TO_GDS).round() as i32;
        let y0 = (r.y * LAMBDA_TO_GDS).round() as i32;
        let x1 = ((r.x + r.width) * LAMBDA_TO_GDS).round() as i32;
        let y1 = ((r.y + r.height) * LAMBDA_TO_GDS).round() as i32;
        gds_struct.elems.push(GdsElement::GdsBoundary(GdsBoundary {
            layer: gds_layer,
            datatype,
            xy: vec![
                GdsPoint::new(x0, y0),
                GdsPoint::new(x1, y0),
                GdsPoint::new(x1, y1),
                GdsPoint::new(x0, y1),
                GdsPoint::new(x0, y0), // close polygon
            ],
            ..Default::default()
        }));
    }

    // Labels → GdsTextElem
    for label in &layout.labels {
        let Some(&(gds_layer, _)) = layer_map.get(&label.layer) else {
            continue;
        };
        let x = (label.position.x * LAMBDA_TO_GDS).round() as i32;
        let y = (label.position.y * LAMBDA_TO_GDS).round() as i32;
        gds_struct.elems.push(GdsElement::GdsTextElem(GdsTextElem {
            layer: gds_layer,
            string: label.text.clone(),
            xy: GdsPoint::new(x, y),
            ..Default::default()
        }));
    }

    let lib = GdsLibrary {
        name: "oxide_lambda_cmos".into(),
        units: GdsUnits::new(1e-9, 1e-6), // db = 1 nm, user = 1 μm (= 1λ)
        structs: vec![gds_struct],
        ..Default::default()
    };
    lib.save(path)?;

    Ok(())
}
