use std::collections::HashMap;

use oxide_db::cell::LayoutView;
use oxide_db::geometry::Point;
use oxide_db::shape::ShapeId;

use crate::connectivity::extract_nets;
use crate::device::{Device, DeviceType};
use crate::net::Net;

pub struct Extractor;

pub struct ExtractionResult {
    pub nets: Vec<Net>,
    pub devices: Vec<Device>,
}

impl ExtractionResult {
    /// Human-readable name for a net: label name if one was assigned, otherwise "net_N".
    pub fn net_name(&self, idx: usize) -> String {
        self.nets
            .get(idx)
            .and_then(|n| n.name.clone())
            .unwrap_or_else(|| format!("net_{}", idx))
    }

    /// Find which net (by index) contains `shape_id`.
    pub fn net_for_shape(&self, shape_id: ShapeId) -> Option<usize> {
        self.nets.iter().position(|n| n.shape_ids.contains(&shape_id))
    }
}

impl Extractor {
    pub fn run(layout: &LayoutView) -> ExtractionResult {
        let nets = extract_nets(layout);
        let devices = recognise_transistors(layout, &nets);
        ExtractionResult { nets, devices }
    }
}

// ── Transistor recognition ────────────────────────────────────────────────────

fn recognise_transistors(layout: &LayoutView, nets: &[Net]) -> Vec<Device> {
    // Build ShapeId → net-index map.
    let shape_to_net: HashMap<ShapeId, usize> = nets
        .iter()
        .enumerate()
        .flat_map(|(ni, net)| net.shape_ids.iter().map(move |&sid| (sid, ni)))
        .collect();

    let poly_shapes: Vec<_> = layout.shapes.iter()
        .filter(|s| s.layer == "poly")
        .collect();

    let active_shapes: Vec<_> = layout.shapes.iter()
        .filter(|s| s.layer == "active")
        .collect();

    let nwell_rects: Vec<_> = layout.shapes.iter()
        .filter(|s| s.layer == "nwell")
        .map(|s| s.bounding_rect())
        .collect();

    let mut devices = Vec::new();

    for poly in &poly_shapes {
        let poly_rect = poly.bounding_rect();

        for active in &active_shapes {
            let active_rect = active.bounding_rect();
            if !poly_rect.intersects(&active_rect) {
                continue;
            }

            // pMOS if the active region intersects any nwell shape.
            let is_pmos = nwell_rects.iter().any(|nw| nw.intersects(&active_rect));

            // Location = centre of poly × active overlap.
            let ox  = poly_rect.x.max(active_rect.x);
            let oy  = poly_rect.y.max(active_rect.y);
            let ox1 = poly_rect.x1().min(active_rect.x1());
            let oy1 = poly_rect.y1().min(active_rect.y1());
            let location = Point::new((ox + ox1) / 2.0, (oy + oy1) / 2.0);

            let idx = devices.len() + 1;
            devices.push(Device {
                name: format!("M{}", idx),
                device_type: if is_pmos { DeviceType::Pmos } else { DeviceType::Nmos },
                gate_net: shape_to_net.get(&poly.id).copied(),
                sd_net:   shape_to_net.get(&active.id).copied(),
                location,
            });
        }
    }

    devices
}
