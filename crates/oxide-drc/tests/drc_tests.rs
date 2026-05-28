use oxide_db::{
    cell::LayoutView,
    geometry::Rect,
    shape::{Shape, ShapeId},
};
use oxide_drc::{engine::DrcEngine, violation::DrcViolation};
use oxide_tech::{
    rules::{DesignRules, SpacingRule, WidthRule},
    tech::Technology,
};

/// Build a minimal technology with just the rules under test.
fn tech_width_only(layer: &str, min: f64) -> Technology {
    Technology {
        name: "test".into(),
        unit: "λ".into(),
        grid: 1.0,
        layers: vec![oxide_tech::layer::LayerDef {
            name: layer.to_string(),
            color: [0, 0, 0, 255],
            purpose: oxide_tech::layer::LayerPurpose::Routing,
            z_order: 1,
            selectable: true,
        }],
        rules: DesignRules {
            width: vec![WidthRule {
                layer: layer.to_string(),
                min,
                description: "test width rule".into(),
            }],
            ..Default::default()
        },
    }
}

fn tech_spacing_only(layer: &str, min: f64) -> Technology {
    Technology {
        name: "test".into(),
        unit: "λ".into(),
        grid: 1.0,
        layers: vec![oxide_tech::layer::LayerDef {
            name: layer.to_string(),
            color: [0, 0, 0, 255],
            purpose: oxide_tech::layer::LayerPurpose::Routing,
            z_order: 1,
            selectable: true,
        }],
        rules: DesignRules {
            spacing: vec![SpacingRule {
                layer_a: layer.to_string(),
                layer_b: None,
                min,
                description: "test spacing rule".into(),
            }],
            ..Default::default()
        },
    }
}

fn shape(id: u64, layer: &str, x: f64, y: f64, w: f64, h: f64) -> Shape {
    Shape::new_rect(ShapeId(id), layer, Rect::new(x, y, w, h))
}

fn layout_with(shapes: Vec<Shape>) -> LayoutView {
    let mut lv = LayoutView::default();
    for s in shapes {
        lv.add_shape(s);
    }
    lv
}

// ─── Width checks ────────────────────────────────────────────────────────────

#[test]
fn width_pass_exact() {
    let tech = tech_width_only("metal1", 3.0);
    let layout = layout_with(vec![shape(0, "metal1", 0.0, 0.0, 3.0, 5.0)]);
    let violations = DrcEngine::new(&tech).run(&layout);
    assert!(violations.is_empty(), "exact min width should pass: {:?}", violations);
}

#[test]
fn width_fail_too_narrow() {
    let tech = tech_width_only("metal1", 3.0);
    // Width = 2λ, which violates min = 3λ
    let layout = layout_with(vec![shape(0, "metal1", 0.0, 0.0, 2.0, 5.0)]);
    let violations = DrcEngine::new(&tech).run(&layout);
    assert_eq!(violations.len(), 1);
    assert!(violations[0].rule.contains("metal1"));
    assert!(violations[0].required.contains("3"));
}

#[test]
fn width_pass_wider() {
    let tech = tech_width_only("poly", 2.0);
    let layout = layout_with(vec![shape(0, "poly", 0.0, 0.0, 4.0, 4.0)]);
    let violations = DrcEngine::new(&tech).run(&layout);
    assert!(violations.is_empty());
}

#[test]
fn width_checked_by_min_dimension() {
    // A rect 10λ wide but only 1λ tall violates min_width = 2λ
    let tech = tech_width_only("poly", 2.0);
    let layout = layout_with(vec![shape(0, "poly", 0.0, 0.0, 10.0, 1.0)]);
    let violations = DrcEngine::new(&tech).run(&layout);
    assert_eq!(violations.len(), 1);
}

// ─── Spacing checks ──────────────────────────────────────────────────────────

#[test]
fn spacing_pass_exact() {
    let tech = tech_spacing_only("metal1", 3.0);
    // Two shapes exactly 3λ apart
    let layout = layout_with(vec![
        shape(0, "metal1", 0.0, 0.0, 4.0, 4.0),
        shape(1, "metal1", 7.0, 0.0, 4.0, 4.0),
    ]);
    let violations = DrcEngine::new(&tech).run(&layout);
    assert!(violations.is_empty(), "exact min spacing should pass: {:?}", violations);
}

#[test]
fn spacing_fail_too_close() {
    let tech = tech_spacing_only("metal1", 3.0);
    // Two shapes only 2λ apart
    let layout = layout_with(vec![
        shape(0, "metal1", 0.0, 0.0, 4.0, 4.0),
        shape(1, "metal1", 6.0, 0.0, 4.0, 4.0),
    ]);
    let violations = DrcEngine::new(&tech).run(&layout);
    assert_eq!(violations.len(), 1);
    assert!(violations[0].rule.contains("metal1"));
}

#[test]
fn spacing_no_violation_with_single_shape() {
    let tech = tech_spacing_only("metal1", 3.0);
    let layout = layout_with(vec![shape(0, "metal1", 0.0, 0.0, 4.0, 4.0)]);
    let violations = DrcEngine::new(&tech).run(&layout);
    assert!(violations.is_empty());
}

#[test]
fn spacing_different_layers_not_checked() {
    // metal1 spacing rule should not flag poly and metal1 being close
    let tech = tech_spacing_only("metal1", 3.0);
    let layout = layout_with(vec![
        shape(0, "metal1", 0.0, 0.0, 4.0, 4.0),
        shape(1, "poly", 5.0, 0.0, 4.0, 4.0),
    ]);
    let violations = DrcEngine::new(&tech).run(&layout);
    assert!(violations.is_empty());
}

// ─── pMOS-in-nwell check ─────────────────────────────────────────────────────

#[test]
fn pmos_in_nwell_pass() {
    let tech = Technology {
        name: "test".into(),
        unit: "λ".into(),
        grid: 1.0,
        layers: vec![
            oxide_tech::layer::LayerDef {
                name: "nwell".into(),
                color: [0, 0, 0, 255],
                purpose: oxide_tech::layer::LayerPurpose::Well,
                z_order: 1,
                selectable: true,
            },
            oxide_tech::layer::LayerDef {
                name: "active".into(),
                color: [0, 0, 0, 255],
                purpose: oxide_tech::layer::LayerPurpose::Diffusion,
                z_order: 2,
                selectable: true,
            },
        ],
        rules: DesignRules::default(),
    };
    let nwell = Shape::new_rect(ShapeId(0), "nwell", Rect::new(0.0, 0.0, 20.0, 20.0));
    let mut pmos_active = Shape::new_rect(ShapeId(1), "active", Rect::new(3.0, 3.0, 6.0, 6.0));
    pmos_active.is_pmos = true;
    let layout = layout_with(vec![nwell, pmos_active]);
    let violations = DrcEngine::new(&tech).run(&layout);
    assert!(violations.is_empty(), "pMOS inside nwell should pass: {:?}", violations);
}

#[test]
fn pmos_outside_nwell_fails() {
    let tech = Technology {
        name: "test".into(),
        unit: "λ".into(),
        grid: 1.0,
        layers: vec![
            oxide_tech::layer::LayerDef {
                name: "nwell".into(),
                color: [0, 0, 0, 255],
                purpose: oxide_tech::layer::LayerPurpose::Well,
                z_order: 1,
                selectable: true,
            },
            oxide_tech::layer::LayerDef {
                name: "active".into(),
                color: [0, 0, 0, 255],
                purpose: oxide_tech::layer::LayerPurpose::Diffusion,
                z_order: 2,
                selectable: true,
            },
        ],
        rules: DesignRules::default(),
    };
    let nwell = Shape::new_rect(ShapeId(0), "nwell", Rect::new(0.0, 0.0, 10.0, 10.0));
    // pMOS active is outside the nwell
    let mut pmos_active = Shape::new_rect(ShapeId(1), "active", Rect::new(15.0, 15.0, 6.0, 6.0));
    pmos_active.is_pmos = true;
    let layout = layout_with(vec![nwell, pmos_active]);
    let violations = DrcEngine::new(&tech).run(&layout);
    assert_eq!(violations.len(), 1);
    assert!(violations[0].rule.contains("n-well") || violations[0].rule.contains("nwell") || violations[0].rule.contains("pMOS"));
}

#[test]
fn nmos_outside_nwell_does_not_flag() {
    let tech = Technology {
        name: "test".into(),
        unit: "λ".into(),
        grid: 1.0,
        layers: vec![
            oxide_tech::layer::LayerDef {
                name: "active".into(),
                color: [0, 0, 0, 255],
                purpose: oxide_tech::layer::LayerPurpose::Diffusion,
                z_order: 1,
                selectable: true,
            },
        ],
        rules: DesignRules::default(),
    };
    // nMOS active (is_pmos = false) should not trigger pMOS-in-nwell check
    let nmos_active = Shape::new_rect(ShapeId(0), "active", Rect::new(0.0, 0.0, 6.0, 6.0));
    let layout = layout_with(vec![nmos_active]);
    let violations = DrcEngine::new(&tech).run(&layout);
    assert!(violations.is_empty());
}
