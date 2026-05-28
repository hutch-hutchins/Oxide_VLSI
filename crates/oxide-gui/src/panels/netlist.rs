use eframe::egui::{self, Color32, RichText};
use oxide_extract::ExtractionResult;

pub struct NetlistPanel;

impl NetlistPanel {
    pub fn show(
        ui: &mut egui::Ui,
        extraction: &Option<ExtractionResult>,
        highlighted_net: &mut Option<usize>,
    ) {
        let Some(result) = extraction else {
            ui.label(RichText::new("No extraction — run Verify → Extract [F6]").color(Color32::DARK_GRAY));
            return;
        };

        ui.horizontal(|ui| {
            ui.strong(format!("{} net(s)", result.nets.len()));
            ui.separator();
            ui.strong(format!("{} device(s)", result.devices.len()));
            if highlighted_net.is_some() {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.small_button("Clear highlight").clicked() {
                        *highlighted_net = None;
                    }
                });
            }
        });
        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            // ── Nets ──────────────────────────────────────────────────────────
            ui.collapsing("Nets", |ui| {
                for (i, net) in result.nets.iter().enumerate() {
                    let name = result.net_name(i);
                    let selected = *highlighted_net == Some(i);
                    let label = format!("{} ({} shapes)", name, net.shape_ids.len());
                    if ui.selectable_label(selected, &label).clicked() {
                        *highlighted_net = if selected { None } else { Some(i) };
                    }
                }
            });

            ui.add_space(4.0);

            // ── Devices ───────────────────────────────────────────────────────
            ui.collapsing("Devices", |ui| {
                if result.devices.is_empty() {
                    ui.label(RichText::new("No transistors found.").color(Color32::DARK_GRAY));
                    ui.label(
                        RichText::new("Draw poly crossing active to create a transistor.")
                            .small()
                            .color(Color32::DARK_GRAY),
                    );
                    return;
                }
                for dev in &result.devices {
                    ui.group(|ui| {
                        ui.label(
                            RichText::new(format!("{} — {}", dev.name, dev.device_type)).strong(),
                        );
                        let gate = dev.gate_net.map(|i| result.net_name(i)).unwrap_or_else(|| "?".into());
                        let sd   = dev.sd_net  .map(|i| result.net_name(i)).unwrap_or_else(|| "?".into());
                        ui.label(format!("  Gate: {}", gate));
                        ui.label(format!("  S/D:  {}", sd));
                        ui.label(format!(
                            "  @ ({:.0}λ, {:.0}λ)",
                            dev.location.x, dev.location.y
                        ));
                        // Clicking a device highlights its gate net
                        if let Some(gi) = dev.gate_net {
                            if ui.small_button("Highlight gate net").clicked() {
                                *highlighted_net = Some(gi);
                            }
                        }
                    });
                    ui.add_space(2.0);
                }
            });
        });
    }
}
