use oxide_db::geometry::Rect;

/// Returns `(project_name, cell_name)` metadata for a template key.
pub fn meta(key: &str) -> (&'static str, &'static str) {
    match key {
        "inverter" => ("cmos_inverter", "inv"),
        "nand2"    => ("nand2_gate",    "nand2"),
        "nor2"     => ("nor2_gate",     "nor2"),
        _          => ("untitled",      "cell1"),
    }
}

/// Pre-drawn skeleton shapes for each template.
/// Returns `(layer_name, rect)` pairs.
/// Coordinates in lambda units; (0,0) = bottom-left of cell.
///
/// Each template provides:
///   - nwell region covering the pMOS transistor area
///   - metal1 VDD rail at top
///   - metal1 GND rail at bottom
///
/// Students draw: active, poly, contacts, and metal wiring.
pub fn shapes(key: &str) -> Vec<(&'static str, Rect)> {
    match key {
        "inverter" => inverter(),
        "nand2"    => nand2(),
        "nor2"     => nor2(),
        _          => vec![],
    }
}

// ── CMOS Inverter ─────────────────────────────────────────────────────────────
// Layout: 14λ wide × 20λ tall
//   y=17..20  VDD rail (metal1)
//   y= 8..20  nwell  (pMOS transistor lives here)
//   y= 0.. 3  GND rail (metal1)
//
//   Students draw: pMOS active (inside nwell), nMOS active (below nwell),
//   poly gate crossing both active regions, contacts, and output metal1.
fn inverter() -> Vec<(&'static str, Rect)> {
    vec![
        ("nwell",  Rect::new(0.0, 8.0, 14.0, 12.0)), // pMOS pocket
        ("metal1", Rect::new(0.0, 17.0, 14.0, 3.0)), // VDD rail
        ("metal1", Rect::new(0.0, 0.0,  14.0, 3.0)), // GND rail
    ]
}

// ── 2-Input NAND ──────────────────────────────────────────────────────────────
// Layout: 28λ wide × 20λ tall
//   pMOS: A and B in parallel (two pMOS side-by-side in nwell)
//   nMOS: A and B in series   (stacked, sharing a node)
fn nand2() -> Vec<(&'static str, Rect)> {
    vec![
        ("nwell",  Rect::new(0.0, 8.0, 28.0, 12.0)), // two pMOS side-by-side
        ("metal1", Rect::new(0.0, 17.0, 28.0, 3.0)), // VDD rail
        ("metal1", Rect::new(0.0, 0.0,  28.0, 3.0)), // GND rail
    ]
}

// ── 2-Input NOR ───────────────────────────────────────────────────────────────
// Layout: 28λ wide × 26λ tall
//   pMOS: A and B in series   (stacked in nwell — nwell must be taller)
//   nMOS: A and B in parallel (side-by-side)
//
//   The extra nwell height gives room for the stacked pMOS pair.
fn nor2() -> Vec<(&'static str, Rect)> {
    vec![
        ("nwell",  Rect::new(0.0, 10.0, 28.0, 16.0)), // taller nwell for series pMOS
        ("metal1", Rect::new(0.0, 23.0, 28.0, 3.0)),  // VDD rail
        ("metal1", Rect::new(0.0, 0.0,  28.0, 3.0)),  // GND rail
    ]
}
