use crate::canvas::CanvasState;
use crate::commands::{Command, CommandStack};
use crate::panels::drc_panel::DrcPanel;
use crate::panels::inspector::Inspector;
use crate::panels::layers::LayerPalette;
use crate::panels::netlist::NetlistPanel;
use crate::panels::project::ProjectPanel;
use crate::tools::ToolMode;
use eframe::egui;
use oxide_db::project::Project;
use oxide_db::shape::{Shape, ShapeId};
use oxide_drc::violation::DrcViolation;
use oxide_tech::tech::Technology;
use std::collections::{HashMap, HashSet};

const LAMBDA_CMOS_TOML: &str = include_str!("../../oxide-tech/data/lambda_cmos.toml");
const LOGO_BYTES: &[u8] = include_bytes!("../../../assets/Oxide_VLSI_Logo_1254x1254_2.png");

pub struct OxideApp {
    pub tech: Technology,
    pub project: Option<Project>,
    pub active_cell: Option<String>,
    pub tool: ToolMode,
    pub selected: HashSet<ShapeId>,
    pub drc_results: Vec<DrcViolation>,
    pub layers_visible: HashMap<String, bool>,
    pub canvas: CanvasState,
    pub drc_panel: DrcPanel,
    pub logo: egui::TextureHandle,
    pub commands: CommandStack,
    pub export_status: Option<String>,
    pub extraction: Option<oxide_extract::ExtractionResult>,
    pub highlighted_net: Option<usize>,
    pub bottom_tab: BottomTab,
}

#[derive(Default, PartialEq)]
pub enum BottomTab {
    #[default]
    Drc,
    Netlist,
}

impl OxideApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let tech = Technology::load_toml_str(LAMBDA_CMOS_TOML)
            .expect("embedded lambda_cmos.toml is valid");
        let layers_visible = tech.layers.iter().map(|l| (l.name.clone(), true)).collect();

        let logo = load_texture(&cc.egui_ctx, "oxide_vlsi_logo", LOGO_BYTES);

        Self {
            tech,
            project: None,
            active_cell: None,
            tool: ToolMode::Select,
            selected: HashSet::new(),
            drc_results: Vec::new(),
            layers_visible,
            canvas: CanvasState::default(),
            drc_panel: DrcPanel::default(),
            logo,
            commands: CommandStack::default(),
            export_status: None,
            extraction: None,
            highlighted_net: None,
            bottom_tab: BottomTab::Drc,
        }
    }

    pub fn new_blank_project(&mut self) {
        let mut project = Project::new("untitled", "lambda_cmos");
        let cell = oxide_db::cell::Cell::new("cell1");
        project.library.add_cell(cell);
        project.meta.cells.push("cell1".to_string());
        self.active_cell = Some("cell1".to_string());
        self.project = Some(project);
        self.drc_results.clear();
        self.selected.clear();
        self.canvas = CanvasState::default();
        self.commands.clear();
        self.extraction = None;
        self.highlighted_net = None;
    }

    pub fn new_from_template(&mut self, key: &str) {
        if key == "blank" {
            self.new_blank_project();
            return;
        }
        let (project_name, cell_name) = crate::templates::meta(key);
        let mut project = Project::new(project_name, "lambda_cmos");
        let mut cell = oxide_db::cell::Cell::new(cell_name);
        for (layer, rect) in crate::templates::shapes(key) {
            let id = cell.layout.next_shape_id();
            cell.layout.add_shape(oxide_db::shape::Shape::new_rect(id, layer, rect));
        }
        project.library.add_cell(cell);
        project.meta.cells.push(cell_name.to_string());
        self.active_cell = Some(cell_name.to_string());
        self.project = Some(project);
        self.drc_results.clear();
        self.selected.clear();
        self.canvas = CanvasState::default();
        self.commands.clear();
        self.extraction = None;
        self.highlighted_net = None;
    }

    pub fn apply_cmd(&mut self, cmd: Command) {
        let commands = &mut self.commands;
        if let Some(proj) = self.project.as_mut() {
            commands.push(cmd, proj);
        }
    }

    pub fn undo(&mut self) {
        let commands = &mut self.commands;
        if let Some(proj) = self.project.as_mut() {
            commands.undo(proj);
        }
        self.selected.clear();
    }

    pub fn redo(&mut self) {
        let commands = &mut self.commands;
        if let Some(proj) = self.project.as_mut() {
            commands.redo(proj);
        }
        self.selected.clear();
    }

    pub fn delete_selected(&mut self) {
        if self.selected.is_empty() { return; }
        let cell_name = match self.active_cell.clone() { Some(c) => c, None => return };
        let shapes: Vec<Shape> = match &self.project {
            Some(proj) => match proj.library.cell(&cell_name) {
                Some(cell) => cell.layout.shapes.iter()
                    .filter(|s| self.selected.contains(&s.id))
                    .cloned()
                    .collect(),
                None => return,
            },
            None => return,
        };
        self.selected.clear();
        self.apply_cmd(Command::DeleteShapes { cell: cell_name, shapes });
    }

    pub fn save_project(&mut self) {
        let path = self
            .project
            .as_ref()
            .and_then(|p| p.path.clone())
            .or_else(|| {
                rfd::FileDialog::new()
                    .set_title("Save Project — choose or create a folder")
                    .pick_folder()
            });

        if let (Some(path), Some(proj)) = (path, self.project.as_mut()) {
            match proj.save(&path) {
                Ok(()) => {
                    proj.path = Some(path.clone());
                    self.export_status =
                        Some(format!("Saved → {}", path.display()));
                }
                Err(e) => {
                    self.export_status = Some(format!("Save failed: {e}"));
                }
            }
        }
    }

    pub fn save_project_as(&mut self) {
        let path = rfd::FileDialog::new()
            .set_title("Save Project As — choose or create a folder")
            .pick_folder();

        if let (Some(path), Some(proj)) = (path, self.project.as_mut()) {
            match proj.save(&path) {
                Ok(()) => {
                    proj.path = Some(path.clone());
                    self.export_status =
                        Some(format!("Saved → {}", path.display()));
                }
                Err(e) => {
                    self.export_status = Some(format!("Save failed: {e}"));
                }
            }
        }
    }

    pub fn open_project(&mut self) {
        let path = rfd::FileDialog::new()
            .set_title("Open Project — select project folder")
            .pick_folder();

        if let Some(path) = path {
            match oxide_db::project::Project::load(&path) {
                Ok(proj) => {
                    self.active_cell = proj.meta.cells.first().cloned();
                    self.project = Some(proj);
                    self.drc_results.clear();
                    self.selected.clear();
                    self.canvas = crate::canvas::CanvasState::default();
                    self.commands.clear();
                    self.extraction = None;
                    self.highlighted_net = None;
                }
                Err(e) => {
                    self.export_status = Some(format!("Open failed: {e}"));
                }
            }
        }
    }

    pub fn do_export(&mut self, format: &str) {
        let cell_name = match self.active_cell.clone() { Some(c) => c, None => return };
        let (layout, tech) = match &self.project {
            Some(proj) => match proj.library.cell(&cell_name) {
                Some(cell) => (cell.layout.clone(), self.tech.clone()),
                None => return,
            },
            None => return,
        };
        let drc_results = self.drc_results.clone();

        let export_dir = std::env::current_dir()
            .unwrap_or_default()
            .join("exports");
        if std::fs::create_dir_all(&export_dir).is_err() {
            self.export_status = Some("Could not create exports/ directory.".into());
            return;
        }

        let ext = if format == "md" { "md" } else { format };
        let path = export_dir.join(format!("{}.{}", cell_name, ext));

        let result: anyhow::Result<()> = match format {
            "svg" => crate::export::export_svg(&layout, &tech, &path),
            "png" => crate::export::export_png(&layout, &tech, &path),
            "gds" => oxide_gds::export_gds(&layout, &cell_name, &path)
                .map_err(|e| anyhow::anyhow!("{e}")),
            "md" => {
                // List SVG/PNG files already present in the exports dir.
                let exported: Vec<String> = ["svg", "png"]
                    .iter()
                    .filter_map(|e| {
                        let p = export_dir.join(format!("{}.{}", cell_name, e));
                        p.exists().then(|| p.display().to_string())
                    })
                    .collect();
                let report = oxide_report::MarkdownReport {
                    cell_name: cell_name.clone(),
                    drc_violations: drc_results,
                    exported_files: exported,
                };
                std::fs::write(&path, report.render())
                    .map_err(|e| anyhow::anyhow!(e))
            }
            _ => return,
        };

        self.export_status = Some(match result {
            Ok(()) => format!("Exported {} → {}", format.to_uppercase(), path.display()),
            Err(e) => format!("Export failed: {}", e),
        });
    }

    pub fn run_extract(&mut self) {
        if let (Some(proj), Some(cell_name)) = (&self.project, &self.active_cell) {
            if let Some(cell) = proj.library.cell(cell_name) {
                self.extraction = Some(oxide_extract::Extractor::run(&cell.layout));
                self.highlighted_net = None;
                self.bottom_tab = BottomTab::Netlist;
            }
        }
    }

    pub fn run_drc(&mut self) {
        let results = if let (Some(proj), Some(cell_name)) =
            (&self.project, &self.active_cell)
        {
            if let Some(cell) = proj.library.cell(cell_name) {
                oxide_drc::engine::DrcEngine::new(&self.tech).run(&cell.layout)
            } else {
                vec![]
            }
        } else {
            vec![]
        };
        self.drc_results = results;
    }
}

pub fn load_texture(ctx: &egui::Context, name: &str, bytes: &[u8]) -> egui::TextureHandle {
    let img = image::load_from_memory(bytes)
        .expect("valid PNG")
        .into_rgba8();
    let (w, h) = img.dimensions();
    let color_image = egui::ColorImage::from_rgba_unmultiplied(
        [w as usize, h as usize],
        img.as_raw(),
    );
    ctx.load_texture(name, color_image, egui::TextureOptions::LINEAR)
}

impl eframe::App for OxideApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // ── Menu bar ─────────────────────────────────────────────────────────
        egui::Panel::top("menu_bar").show_inside(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New Blank Project").clicked() {
                        self.new_blank_project();
                        ui.close();
                    }
                    ui.separator();
                    if ui
                        .add_enabled(
                            self.project.is_some(),
                            egui::Button::new("Save Project  [Ctrl+S]"),
                        )
                        .clicked()
                    {
                        self.save_project();
                        ui.close();
                    }
                    if ui
                        .add_enabled(
                            self.project.is_some(),
                            egui::Button::new("Save Project As..."),
                        )
                        .clicked()
                    {
                        self.save_project_as();
                        ui.close();
                    }
                    if ui.button("Open Project...").clicked() {
                        self.open_project();
                        ui.close();
                    }
                    ui.separator();
                    if ui.button("Quit").clicked() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.menu_button("Edit", |ui| {
                    if ui
                        .add_enabled(self.commands.can_undo(), egui::Button::new("Undo  [Ctrl+Z]"))
                        .clicked()
                    {
                        self.undo();
                        ui.close();
                    }
                    if ui
                        .add_enabled(self.commands.can_redo(), egui::Button::new("Redo  [Ctrl+Y]"))
                        .clicked()
                    {
                        self.redo();
                        ui.close();
                    }
                    ui.separator();
                    if ui
                        .add_enabled(!self.selected.is_empty(), egui::Button::new("Delete Selected  [Del]"))
                        .clicked()
                    {
                        self.delete_selected();
                        ui.close();
                    }
                });

                ui.menu_button("Draw", |ui| {
                    if ui.button("Select  [Esc]").clicked() {
                        self.tool = ToolMode::Select;
                        ui.close();
                    }
                    if ui.button("Rectangle  [R]").clicked() {
                        self.tool = ToolMode::DrawRect { layer: "metal1".into() };
                        ui.close();
                    }
                });

                ui.menu_button("Verify", |ui| {
                    if ui.button("Run DRC      [F5]").clicked() {
                        self.run_drc();
                        ui.close();
                    }
                    if ui.button("Run Extract  [F6]").clicked() {
                        self.run_extract();
                        ui.close();
                    }
                });

                let has_project = self.project.is_some();
                ui.menu_button("Export", |ui| {
                    if ui.add_enabled(has_project, egui::Button::new("Export SVG")).clicked() {
                        self.do_export("svg");
                        ui.close();
                    }
                    if ui.add_enabled(has_project, egui::Button::new("Export PNG")).clicked() {
                        self.do_export("png");
                        ui.close();
                    }
                    if ui.add_enabled(has_project, egui::Button::new("Export GDS")).clicked() {
                        self.do_export("gds");
                        ui.close();
                    }
                    ui.separator();
                    if ui.add_enabled(has_project, egui::Button::new("Export Report  [Markdown]")).clicked() {
                        self.do_export("md");
                        ui.close();
                    }
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let label = match &self.tool {
                        ToolMode::Select => "● Select".to_string(),
                        ToolMode::DrawRect { layer } => format!("● Rect: {}", layer),
                    };
                    ui.label(egui::RichText::new(label).small());
                });
            });
        });

        // Keyboard shortcuts
        let ctx = ui.ctx().clone();
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.tool = ToolMode::Select;
        }
        if ctx.input(|i| i.key_pressed(egui::Key::R)) && self.project.is_some() {
            self.tool = ToolMode::DrawRect { layer: "metal1".into() };
        }
        if ctx.input(|i| i.key_pressed(egui::Key::F5)) {
            self.run_drc();
        }
        if ctx.input(|i| i.key_pressed(egui::Key::F6)) {
            self.run_extract();
        }
        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::Z)) {
            self.undo();
        }
        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::Y)) {
            self.redo();
        }
        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::S))
            && self.project.is_some()
        {
            self.save_project();
        }
        if ctx.input(|i| i.key_pressed(egui::Key::Delete)) {
            self.delete_selected();
        }

        // ── Bottom panel: DRC / Netlist tabs ─────────────────────────────────
        egui::Panel::bottom("bottom_panel")
            .min_size(130.0)
            .show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.bottom_tab, BottomTab::Drc, "DRC");
                    ui.selectable_value(&mut self.bottom_tab, BottomTab::Netlist, "Netlist");
                });
                ui.separator();
                match self.bottom_tab {
                    BottomTab::Drc => {
                        self.drc_panel.show(ui, &self.drc_results, &mut self.canvas);
                    }
                    BottomTab::Netlist => {
                        NetlistPanel::show(ui, &self.extraction, &mut self.highlighted_net);
                    }
                }
            });

        // ── Left panel: project tree ─────────────────────────────────────────
        egui::Panel::left("project_panel")
            .min_size(160.0)
            .show_inside(ui, |ui| {
                ProjectPanel::show(ui, &self.project, &mut self.active_cell);
            });

        // ── Right panel: inspector + layer palette ───────────────────────────
        egui::Panel::right("inspector_panel")
            .min_size(180.0)
            .show_inside(ui, |ui| {
                Inspector::show(ui, &self.project, &self.active_cell, &self.selected);
                ui.separator();
                LayerPalette::show(ui, &self.tech, &mut self.layers_visible, &mut self.tool);
            });

        // ── Export status popup ──────────────────────────────────────────────
        if self.export_status.is_some() {
            let msg = self.export_status.clone().unwrap();
            egui::Window::new("Export")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
                .show(ui.ctx(), |ui| {
                    ui.label(&msg);
                    ui.add_space(8.0);
                    if ui.button("OK").clicked() {
                        self.export_status = None;
                    }
                });
        }

        // ── Central canvas (remaining space after panels) ────────────────────
        if self.project.is_none() {
            crate::panels::welcome::WelcomeScreen::show(ui, self);
        } else {
            crate::canvas::show_canvas(ui, self);
        }
    }
}
