use eframe::egui;
use oxide_db::project::Project;

pub struct ProjectPanel;

impl ProjectPanel {
    pub fn show(
        ui: &mut egui::Ui,
        project: &Option<Project>,
        active_cell: &mut Option<String>,
    ) {
        ui.heading("Project");
        ui.separator();

        if let Some(proj) = project {
            ui.label(&proj.meta.name);
            ui.small(format!("Tech: {}", proj.meta.technology));
            ui.separator();
            ui.label("Cells");
            for cell_name in &proj.meta.cells {
                let selected = active_cell.as_deref() == Some(cell_name.as_str());
                if ui.selectable_label(selected, cell_name).clicked() {
                    *active_cell = Some(cell_name.clone());
                }
            }
        } else {
            ui.label("No project open.");
        }
    }
}
