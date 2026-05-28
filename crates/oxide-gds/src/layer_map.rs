use std::collections::HashMap;

/// Maps Oxide layer names to GDS layer/datatype numbers.
pub fn lambda_cmos_layer_map() -> HashMap<String, (i16, i16)> {
    let mut m = HashMap::new();
    m.insert("nwell".into(), (1, 0));
    m.insert("active".into(), (2, 0));
    m.insert("poly".into(), (3, 0));
    m.insert("contact".into(), (4, 0));
    m.insert("metal1".into(), (5, 0));
    m.insert("via1".into(), (6, 0));
    m.insert("metal2".into(), (7, 0));
    m.insert("label".into(), (8, 0));
    m
}
