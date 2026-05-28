use oxide_db::{
    cell::{Cell, CellMetadata},
    geometry::Rect,
    project::Project,
    shape::{Shape, ShapeId},
};

#[test]
fn shape_ron_roundtrip() {
    let original = Shape::new_rect(ShapeId(42), "metal1", Rect::new(10.0, 20.0, 5.0, 3.0));
    let serialized = ron::to_string(&original).expect("serialize shape");
    let restored: Shape = ron::from_str(&serialized).expect("deserialize shape");
    assert_eq!(restored.id, original.id);
    assert_eq!(restored.layer, original.layer);
    let r = restored.bounding_rect();
    assert!((r.x - 10.0).abs() < 1e-9);
    assert!((r.y - 20.0).abs() < 1e-9);
    assert!((r.width - 5.0).abs() < 1e-9);
    assert!((r.height - 3.0).abs() < 1e-9);
}

#[test]
fn layout_view_ron_roundtrip() {
    let mut cell = Cell::new("inv");
    let id = cell.layout.next_shape_id();
    let s = Shape::new_rect(id, "poly", Rect::new(4.0, 4.0, 2.0, 8.0));
    cell.layout.add_shape(s);

    let serialized = ron::to_string(&cell.layout).expect("serialize layout");
    let restored: oxide_db::cell::LayoutView = ron::from_str(&serialized).expect("deserialize");
    assert_eq!(restored.shapes.len(), 1);
    assert_eq!(restored.shapes[0].layer, "poly");
}

#[test]
fn project_save_load_roundtrip() {
    let tmp = std::env::temp_dir().join("oxide_test_project");
    let _ = std::fs::remove_dir_all(&tmp);

    let mut project = Project::new("test_proj", "lambda_cmos");
    let mut cell = Cell::new("inv");
    cell.metadata = CellMetadata {
        description: "CMOS inverter".into(),
        technology: "lambda_cmos".into(),
    };
    let id = cell.layout.next_shape_id();
    cell.layout.add_shape(Shape::new_rect(
        id,
        "metal1",
        Rect::new(0.0, 0.0, 6.0, 3.0),
    ));
    project.library.add_cell(cell);
    project.meta.cells.push("inv".into());

    project.save(&tmp).expect("save project");

    let loaded = Project::load(&tmp).expect("load project");
    assert_eq!(loaded.meta.name, "test_proj");
    assert_eq!(loaded.meta.technology, "lambda_cmos");
    assert!(loaded.library.cell("inv").is_some());

    let inv = loaded.library.cell("inv").unwrap();
    assert_eq!(inv.layout.shapes.len(), 1);
    assert_eq!(inv.layout.shapes[0].layer, "metal1");
    assert_eq!(inv.metadata.description, "CMOS inverter");

    // Cleanup
    let _ = std::fs::remove_dir_all(&tmp);
}
