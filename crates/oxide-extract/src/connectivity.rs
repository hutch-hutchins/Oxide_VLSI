use oxide_db::cell::LayoutView;
use oxide_db::shape::ShapeId;

use crate::net::Net;

/// Layers that carry electrical signals.  nwell and label are structural/annotation only.
const SIGNAL_LAYERS: &[&str] = &["active", "poly", "contact", "metal1", "via1", "metal2"];

/// Via/contact connection rules: (connector_layer, layers_below, layers_above).
/// A connector shape bridges any touching shape on a below-layer to any touching shape
/// on an above-layer (and also unions them with the connector itself).
const CONTACT_RULES: &[(&str, &[&str], &[&str])] = &[
    ("contact", &["active", "poly"], &["metal1"]),
    ("via1",    &["metal1"],         &["metal2"]),
];

// ── Union-Find ────────────────────────────────────────────────────────────────

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<u8>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self { parent: (0..n).collect(), rank: vec![0; n] }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let rx = self.find(x);
        let ry = self.find(y);
        if rx == ry { return; }
        match self.rank[rx].cmp(&self.rank[ry]) {
            std::cmp::Ordering::Less    => self.parent[rx] = ry,
            std::cmp::Ordering::Greater => self.parent[ry] = rx,
            std::cmp::Ordering::Equal   => { self.parent[ry] = rx; self.rank[rx] += 1; }
        }
    }
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Flood-fill net extraction.  Returns one `Net` per connected component.
pub fn extract_nets(layout: &LayoutView) -> Vec<Net> {
    // Only work with signal-carrying shapes.
    let shapes: Vec<_> = layout.shapes.iter()
        .filter(|s| SIGNAL_LAYERS.iter().any(|&l| l == s.layer.as_str()))
        .collect();
    let n = shapes.len();
    if n == 0 { return vec![]; }

    let mut uf = UnionFind::new(n);

    // Step 1 — union same-layer shapes that touch or overlap.
    for i in 0..n {
        for j in (i + 1)..n {
            if shapes[i].layer != shapes[j].layer { continue; }
            let ri = shapes[i].bounding_rect();
            let rj = shapes[j].bounding_rect();
            if ri.touches(&rj) || ri.intersects(&rj) {
                uf.union(i, j);
            }
        }
    }

    // Step 2 — contacts and vias bridge layers.
    for &(connector_layer, below_layers, above_layers) in CONTACT_RULES {
        for ci in 0..n {
            if shapes[ci].layer.as_str() != connector_layer { continue; }
            let cr = shapes[ci].bounding_rect();

            let mut group = vec![ci];
            for si in 0..n {
                if si == ci { continue; }
                let layer = shapes[si].layer.as_str();
                let is_below = below_layers.iter().any(|&l| l == layer);
                let is_above = above_layers.iter().any(|&l| l == layer);
                if !is_below && !is_above { continue; }

                let sr = shapes[si].bounding_rect();
                if cr.touches(&sr) || cr.intersects(&sr) {
                    group.push(si);
                }
            }

            for &k in &group[1..] {
                uf.union(group[0], k);
            }
        }
    }

    // Step 3 — group shapes by union-find root → one Net per group.
    let mut groups: std::collections::HashMap<usize, Vec<ShapeId>> =
        std::collections::HashMap::new();
    for (i, shape) in shapes.iter().enumerate() {
        let root = uf.find(i);
        groups.entry(root).or_default().push(shape.id);
    }

    // Assign net names from any label whose position falls inside a shape in the net.
    let mut nets: Vec<Net> = groups
        .into_values()
        .map(|shape_ids| {
            let name = net_name_from_labels(layout, &shape_ids);
            Net { name, shape_ids }
        })
        .collect();

    // Stable ordering: sort by lowest ShapeId in each net.
    nets.sort_by_key(|n| n.shape_ids.iter().map(|s| s.0).min().unwrap_or(0));
    nets
}

fn net_name_from_labels(layout: &LayoutView, shape_ids: &[ShapeId]) -> Option<String> {
    for label in &layout.labels {
        let pt = &label.position;
        for &sid in shape_ids {
            if let Some(shape) = layout.shapes.iter().find(|s| s.id == sid) {
                let r = shape.bounding_rect();
                if pt.x >= r.x && pt.x <= r.x1() && pt.y >= r.y && pt.y <= r.y1() {
                    return Some(label.text.clone());
                }
            }
        }
    }
    None
}
