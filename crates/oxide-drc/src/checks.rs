use crate::violation::DrcViolation;
use oxide_db::cell::LayoutView;
use oxide_db::geometry::Rect;
use oxide_db::shape::Shape;
use oxide_tech::rules::{DesignRules, EnclosureRule, SpacingRule, WidthRule};

/// Run all width checks for a layout.
pub fn check_widths(layout: &LayoutView, rules: &[WidthRule]) -> Vec<DrcViolation> {
    let mut violations = Vec::new();

    for rule in rules {
        for shape in layout.shapes_on_layer(&rule.layer) {
            let r = shape.bounding_rect();
            let min_dim = r.min_dim();
            if min_dim < rule.min - 1e-9 {
                violations.push(DrcViolation::new(
                    format!("{} width violation", rule.layer),
                    r.center(),
                    Some(r),
                    vec![shape.id],
                    format!("{} width >= {}λ", rule.layer, rule.min),
                    format!("{} width = {:.1}λ", rule.layer, min_dim),
                    format!(
                        "{}. If shapes on this layer are too narrow, they may fail \
                         during fabrication.",
                        rule.description
                    ),
                    format!(
                        "Increase the {} shape width to at least {}λ.",
                        rule.layer, rule.min
                    ),
                ));
            }
        }
    }

    violations
}

/// Run all spacing checks. O(n²) per layer pair; fine for small educational layouts.
pub fn check_spacing(layout: &LayoutView, rules: &[SpacingRule]) -> Vec<DrcViolation> {
    let mut violations = Vec::new();

    for rule in rules {
        let layer_b = rule.layer_b.as_deref().unwrap_or(&rule.layer_a);
        let shapes_a: Vec<&Shape> = layout.shapes_on_layer(&rule.layer_a).collect();
        let shapes_b: Vec<&Shape> = layout.shapes_on_layer(layer_b).collect();

        for (i, sa) in shapes_a.iter().enumerate() {
            let ra = sa.bounding_rect();

            let start = if rule.layer_b.is_none() { i + 1 } else { 0 };
            let iter: Box<dyn Iterator<Item = &&Shape>> = if rule.layer_b.is_none() {
                Box::new(shapes_a[start..].iter())
            } else {
                Box::new(shapes_b.iter())
            };

            for sb in iter {
                if sa.id == sb.id {
                    continue;
                }
                let rb = sb.bounding_rect();
                let sep = rect_min_separation(ra, rb);
                if sep < rule.min - 1e-9 && sep >= 0.0 {
                    let mid_x = (ra.center().x + rb.center().x) / 2.0;
                    let mid_y = (ra.center().y + rb.center().y) / 2.0;
                    violations.push(DrcViolation::new(
                        format!("{} spacing violation", rule.layer_a),
                        oxide_db::geometry::Point::new(mid_x, mid_y),
                        None,
                        vec![sa.id, sb.id],
                        format!("{} spacing >= {}λ", rule.layer_a, rule.min),
                        format!("{} spacing = {:.1}λ", rule.layer_a, sep),
                        format!(
                            "{}. Shapes that are too close may short during fabrication.",
                            rule.description
                        ),
                        format!("Increase the spacing between these {} shapes to at least {}λ.", rule.layer_a, rule.min),
                    ));
                }
            }
        }
    }

    violations
}

/// Run enclosure checks: `outer` must enclose `inner` by at least `min` on all sides.
pub fn check_enclosures(layout: &LayoutView, rules: &[EnclosureRule]) -> Vec<DrcViolation> {
    let mut violations = Vec::new();

    for rule in rules {
        let inners: Vec<&Shape> = layout.shapes_on_layer(&rule.inner).collect();
        let outers: Vec<&Shape> = layout.shapes_on_layer(&rule.outer).collect();

        'inner_loop: for inner in &inners {
            let ri = inner.bounding_rect();
            // Find an outer shape that encloses with sufficient margin
            for outer in &outers {
                let ro = outer.bounding_rect();
                if enclosure_margin(ro, ri) >= rule.min - 1e-9 {
                    continue 'inner_loop;
                }
            }
            // No adequate enclosure found
            violations.push(DrcViolation::new(
                format!("{} must enclose {}", rule.outer, rule.inner),
                ri.center(),
                Some(ri),
                vec![inner.id],
                format!("{} encloses {} by >= {}λ", rule.outer, rule.inner, rule.min),
                format!("no enclosing {} found with {}λ margin", rule.outer, rule.min),
                rule.description.clone(),
                format!(
                    "Draw a {} shape around this {} with at least {}λ overlap on all sides.",
                    rule.outer, rule.inner, rule.min
                ),
            ));
        }
    }

    violations
}

/// Check that pMOS active regions are inside nwell.
pub fn check_pmos_in_nwell(layout: &LayoutView) -> Vec<DrcViolation> {
    let mut violations = Vec::new();
    let nwells: Vec<Rect> = layout.shapes_on_layer("nwell").map(|s| s.bounding_rect()).collect();

    for shape in layout.shapes_on_layer("active") {
        if !shape.is_pmos {
            continue;
        }
        let ra = shape.bounding_rect();
        let enclosed = nwells.iter().any(|nw| nw.contains_rect(&ra));
        if !enclosed {
            violations.push(DrcViolation::new(
                "pMOS outside n-well",
                ra.center(),
                Some(ra),
                vec![shape.id],
                "pMOS active inside nwell",
                "pMOS active not enclosed by any nwell",
                "pMOS transistors must be fabricated inside an n-well. \
                 The body of the pMOS is connected to the n-well, which is \
                 biased to VDD to prevent the source-body junction from forward biasing.",
                "Draw an nwell region that completely encloses the pMOS active area, \
                 with at least 3λ of margin on all sides.",
            ));
        }
    }

    violations
}

// ─── Geometry helpers ────────────────────────────────────────────────────────

/// Minimum edge-to-edge separation between two rects (0 if touching, negative if overlapping).
fn rect_min_separation(a: Rect, b: Rect) -> f64 {
    let gap_x = (b.x - a.x1()).max(a.x - b.x1()).max(0.0);
    let gap_y = (b.y - a.y1()).max(a.y - b.y1()).max(0.0);
    if gap_x == 0.0 && gap_y == 0.0 {
        // Overlapping or touching: separation is 0 (or negative for overlap)
        if a.intersects(&b) { -1.0 } else { 0.0 }
    } else {
        // Diagonal gap: use the Manhattan-axis minimum
        if gap_x > 0.0 && gap_y > 0.0 { gap_x.min(gap_y) } else { gap_x + gap_y }
    }
}

/// Minimum enclosure margin of `outer` around `inner` (min of 4 sides).
fn enclosure_margin(outer: Rect, inner: Rect) -> f64 {
    let left = inner.x - outer.x;
    let right = outer.x1() - inner.x1();
    let bottom = inner.y - outer.y;
    let top = outer.y1() - inner.y1();
    left.min(right).min(bottom).min(top)
}
