use crate::tools::ToolMode;
use eframe::egui::{self, Color32};
use oxide_tech::tech::Technology;
use std::collections::HashMap;

pub struct LayerPalette;

impl LayerPalette {
    pub fn show(
        ui: &mut egui::Ui,
        tech: &Technology,
        visible: &mut HashMap<String, bool>,
        tool: &mut ToolMode,
    ) {
        ui.heading("Layers");
        ui.separator();

        let mut sorted_layers = tech.layers.clone();
        sorted_layers.sort_by_key(|l| std::cmp::Reverse(l.z_order));

        for layer in &sorted_layers {
            ui.horizontal(|ui| {
                let v = visible.entry(layer.name.clone()).or_insert(true);
                ui.checkbox(v, "");

                let [r, g, b, _] = layer.color;
                let swatch = egui::ColorImage::from_rgba_unmultiplied(
                    [1, 1],
                    &[r, g, b, 200],
                );
                let _ = swatch; // color swatch shown via colored button
                let active = matches!(tool, ToolMode::DrawRect { layer: l } if l == &layer.name);
                let btn = egui::Button::new(&layer.name)
                    .fill(Color32::from_rgb(r, g, b));
                if ui.add(btn).clicked() {
                    *tool = ToolMode::DrawRect { layer: layer.name.clone() };
                }
                if active {
                    ui.label("◀");
                }
            });
        }
    }
}
