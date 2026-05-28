use crate::app::OxideApp;
use eframe::egui::{self, RichText, Vec2};

pub struct WelcomeScreen;

impl WelcomeScreen {
    pub fn show(ui: &mut egui::Ui, app: &mut OxideApp) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(40.0);

                // Logo — shrink to fit if window is narrow
                let logo_size = ui.available_width().min(260.0);
                ui.add(
                    egui::Image::new((app.logo.id(), Vec2::splat(logo_size)))
                        .maintain_aspect_ratio(true),
                );

                ui.add_space(12.0);
                ui.label(
                    RichText::new("CMOS layout education without the commercial EDA wall.")
                        .size(13.0)
                        .color(egui::Color32::GRAY),
                );
                ui.add_space(24.0);
                ui.separator();
                ui.add_space(16.0);

                ui.label(RichText::new("New Project").size(15.0).strong());
                ui.add_space(8.0);

                let templates: &[(&str, &str)] = &[
                    ("CMOS Inverter Lab", "inverter"),
                    ("2-Input NAND Lab", "nand2"),
                    ("2-Input NOR Lab", "nor2"),
                    ("Blank Layout Cell", "blank"),
                ];

                for (label, key) in templates {
                    if ui
                        .add(
                            egui::Button::new(RichText::new(*label).size(13.0))
                                .min_size(Vec2::new(220.0, 30.0)),
                        )
                        .clicked()
                    {
                        app.new_from_template(key);
                    }
                    ui.add_space(4.0);
                }

                ui.add_space(20.0);
                ui.separator();
                ui.add_space(10.0);
                ui.label(
                    RichText::new("v0.1  ·  lambda CMOS  ·  DRC  ·  SVG/PNG export")
                        .size(11.0)
                        .color(egui::Color32::DARK_GRAY),
                );
                ui.add_space(20.0);
            });
        });
    }
}
