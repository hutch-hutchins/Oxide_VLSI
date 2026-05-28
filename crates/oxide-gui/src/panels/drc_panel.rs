use crate::canvas::CanvasState;
use eframe::egui::{self, Color32, RichText};
use oxide_drc::violation::DrcViolation;

#[derive(Default)]
pub struct DrcPanel {
    pub selected_violation: Option<usize>,
}

impl DrcPanel {
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        violations: &[DrcViolation],
        canvas: &mut CanvasState,
    ) {
        ui.horizontal(|ui| {
            ui.heading("DRC");
            ui.separator();
            if violations.is_empty() {
                ui.label(RichText::new("PASS").color(Color32::GREEN).strong());
            } else {
                ui.label(
                    RichText::new(format!("FAIL — {} error(s)", violations.len()))
                        .color(Color32::RED)
                        .strong(),
                );
            }
        });
        ui.separator();

        egui::ScrollArea::vertical().max_height(120.0).show(ui, |ui| {
            for (i, v) in violations.iter().enumerate() {
                let selected = self.selected_violation == Some(i);
                let text = format!(
                    "{}: {} at ({:.0}λ, {:.0}λ)",
                    v.rule, v.found, v.location.x, v.location.y
                );
                if ui.selectable_label(selected, &text).clicked() {
                    self.selected_violation = Some(i);
                    canvas.highlight_at = Some((v.location.x, v.location.y));
                }
                if selected {
                    ui.indent("drc_detail", |ui| {
                        ui.label(format!("Required: {}", v.required));
                        ui.label(format!("Found: {}", v.found));
                        ui.label(format!("Why: {}", v.explanation));
                        ui.label(RichText::new(format!("Fix: {}", v.fix_hint)).italics());
                    });
                }
            }
        });
    }
}
