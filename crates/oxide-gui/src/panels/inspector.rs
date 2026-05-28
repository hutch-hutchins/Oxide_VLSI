use eframe::egui;
use oxide_db::project::Project;
use oxide_db::shape::ShapeId;
use std::collections::HashSet;

pub struct Inspector;

impl Inspector {
    pub fn show(
        ui: &mut egui::Ui,
        project: &Option<Project>,
        active_cell: &Option<String>,
        selected: &HashSet<ShapeId>,
    ) {
        ui.heading("Inspector");
        ui.separator();

        if selected.is_empty() {
            ui.label("Nothing selected.");
            return;
        }

        if let (Some(proj), Some(cell_name)) = (project, active_cell) {
            if let Some(cell) = proj.library.cell(cell_name) {
                for id in selected {
                    if let Some(shape) = cell.layout.shapes.iter().find(|s| s.id == *id) {
                        let r = shape.bounding_rect();
                        ui.label(format!("Layer: {}", shape.layer));
                        ui.label(format!(
                            "Rect: ({:.0}λ, {:.0}λ) → ({:.0}λ, {:.0}λ)",
                            r.x, r.y, r.x1(), r.y1()
                        ));
                        ui.label(format!("W: {:.0}λ  H: {:.0}λ", r.width, r.height));
                        if let Some(net) = &shape.net {
                            ui.label(format!("Net: {}", net));
                        }
                    }
                }
            }
        }
    }
}
