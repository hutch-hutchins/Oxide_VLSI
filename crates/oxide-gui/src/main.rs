mod app;
mod canvas;
mod commands;
mod export;
mod panels;
mod templates;
mod tools;

use std::sync::Arc;

const ICON_BYTES: &[u8] =
    include_bytes!("../../../assets/Oxide_VLSI_Logo_1254x1254_1.png");

fn main() -> eframe::Result<()> {
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Oxide VLSI")
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_icon(load_icon()),
        ..Default::default()
    };

    eframe::run_native(
        "Oxide VLSI",
        options,
        Box::new(|cc| Ok(Box::new(app::OxideApp::new(cc)))),
    )
}

fn load_icon() -> Arc<egui::IconData> {
    let img = image::load_from_memory(ICON_BYTES)
        .expect("valid icon PNG")
        .into_rgba8();
    let (w, h) = img.dimensions();
    Arc::new(egui::IconData {
        rgba: img.into_raw(),
        width: w,
        height: h,
    })
}
