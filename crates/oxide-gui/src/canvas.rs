use crate::app::OxideApp;
use crate::commands::Command;
use crate::tools::ToolMode;
use eframe::egui::{self, Color32, Painter, Pos2, Rect, Sense, Stroke, Vec2};
use oxide_db::shape::ShapeId;

const GRID_COLOR: Color32 = Color32::from_rgba_premultiplied(60, 60, 60, 120);
const GRID_MAJOR_COLOR: Color32 = Color32::from_rgba_premultiplied(80, 80, 80, 180);
const DRC_HIGHLIGHT: Color32 = Color32::from_rgba_premultiplied(255, 60, 60, 120);
const BOX_SELECT_COLOR: Color32 = Color32::from_rgba_premultiplied(100, 180, 255, 200);

// ── Select drag state ─────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum SelectDragKind {
    Move,
    BoxSelect,
}

#[derive(Debug)]
pub struct SelectDrag {
    pub start: (f64, f64),
    pub current: (f64, f64),
    pub kind: SelectDragKind,
}

// ── Canvas state ──────────────────────────────────────────────────────────────

#[derive(Debug, Default)]
pub struct CanvasState {
    pub pan: Vec2,
    pub scale: f32,
    /// Anchor point (lambda) for the rect-draw tool.
    pub draw_start: Option<(f64, f64)>,
    /// DRC panel click target (lambda).
    pub highlight_at: Option<(f64, f64)>,
    pub select_drag: Option<SelectDrag>,
}

impl CanvasState {
    pub fn lambda_to_screen(&self, x: f64, y: f64, origin: Pos2) -> Pos2 {
        Pos2::new(
            origin.x + self.pan.x + (x as f32) * self.scale,
            origin.y + self.pan.y - (y as f32) * self.scale,
        )
    }

    pub fn screen_to_lambda(&self, pos: Pos2, origin: Pos2) -> (f64, f64) {
        let lx = ((pos.x - origin.x - self.pan.x) / self.scale) as f64;
        let ly = ((origin.y + self.pan.y - pos.y) / self.scale) as f64;
        (lx, ly)
    }

    pub fn snap(&self, v: f64, grid: f64) -> f64 {
        (v / grid).round() * grid
    }

    pub fn pan_to_lambda(&mut self, lx: f64, ly: f64, viewport: Rect) {
        self.pan.x = viewport.width() / 2.0 - (lx as f32) * self.scale;
        self.pan.y = viewport.height() / 2.0 + (ly as f32) * self.scale;
    }
}

// ── Entry point ───────────────────────────────────────────────────────────────

pub fn show_canvas(ui: &mut egui::Ui, app: &mut OxideApp) {
    let (response, painter) =
        ui.allocate_painter(ui.available_size(), Sense::click_and_drag());
    let origin = response.rect.min;

    // Init
    if app.canvas.scale == 0.0 {
        app.canvas.scale = 8.0;
        app.canvas.pan =
            Vec2::new(response.rect.width() / 2.0, response.rect.height() / 2.0);
    }

    // Pan (middle-mouse drag)
    if response.dragged_by(egui::PointerButton::Middle) {
        app.canvas.pan += response.drag_delta();
    }

    // Zoom (scroll wheel, zoom toward cursor)
    let scroll = ui.ctx().input(|i| i.smooth_scroll_delta.y);
    if scroll != 0.0 {
        let factor = if scroll > 0.0 { 1.1f32 } else { 1.0 / 1.1 };
        app.canvas.scale = (app.canvas.scale * factor).clamp(1.0, 200.0);
    }

    // Cursor in lambda coords
    let cursor = response.hover_pos().map(|pos| {
        let (lx, ly) = app.canvas.screen_to_lambda(pos, origin);
        let sx = app.canvas.snap(lx, app.tech.grid);
        let sy = app.canvas.snap(ly, app.tech.grid);
        (lx, ly, sx, sy)
    });

    // ── Process interactions first so draw sees updated state ─────────────────
    if let Some((lx, ly, sx, sy)) = cursor {
        match app.tool.clone() {
            ToolMode::DrawRect { layer } => {
                update_draw_rect(&response, app, sx, sy, layer);
            }
            ToolMode::Select => {
                update_select(&response, ui, app, lx, ly, sx, sy);
            }
        }
    }

    // ── Draw ──────────────────────────────────────────────────────────────────
    draw_grid(&painter, &app.canvas, response.rect, origin, app.tech.grid);
    draw_shapes(&painter, &app.canvas, app, origin);

    // Tool overlays (ghost rect, rubber band)
    if let Some((lx, ly, sx, sy)) = cursor {
        draw_overlays(&painter, &app.canvas, app, origin, lx, ly, sx, sy);
    }

    // Coordinate label
    if let Some((_, _, sx, sy)) = cursor {
        painter.text(
            response.rect.left_bottom() + Vec2::new(8.0, -8.0),
            egui::Align2::LEFT_BOTTOM,
            format!("({:.0}λ, {:.0}λ)", sx, sy),
            egui::FontId::monospace(12.0),
            Color32::from_gray(200),
        );
    }
}

// ── Draw helpers ──────────────────────────────────────────────────────────────

fn layer_color(color: [u8; 4]) -> Color32 {
    Color32::from_rgba_premultiplied(color[0], color[1], color[2], color[3])
}

fn oxide_rect_to_egui(state: &CanvasState, r: &oxide_db::geometry::Rect, origin: Pos2) -> Rect {
    let tl = state.lambda_to_screen(r.x, r.y + r.height, origin);
    let br = state.lambda_to_screen(r.x + r.width, r.y, origin);
    Rect::from_min_max(tl, br)
}

fn draw_grid(painter: &Painter, state: &CanvasState, viewport: Rect, origin: Pos2, grid: f64) {
    let px_per_lambda = state.scale as f64;
    if px_per_lambda < 4.0 {
        return;
    }
    let step = px_per_lambda * grid;
    let major_every = 10i64;

    // Invert sx = origin.x + pan.x + xi*step  →  xi = (sx - origin.x - pan.x) / step
    let x_offset = (origin.x + state.pan.x) as f64;
    let x_start = ((viewport.left() as f64 - x_offset) / step).floor() as i64;
    let x_end   = ((viewport.right() as f64 - x_offset) / step).ceil() as i64;
    for xi in x_start..=x_end {
        let sx = (x_offset + xi as f64 * step) as f32;
        let color = if xi % major_every == 0 { GRID_MAJOR_COLOR } else { GRID_COLOR };
        painter.line_segment(
            [Pos2::new(sx, viewport.top()), Pos2::new(sx, viewport.bottom())],
            Stroke::new(1.0, color),
        );
    }

    // Invert sy = origin.y + pan.y - yi*step  →  yi = (origin.y + pan.y - sy) / step
    let y_offset = (origin.y + state.pan.y) as f64;
    let y_start = ((y_offset - viewport.bottom() as f64) / step).floor() as i64;
    let y_end   = ((y_offset - viewport.top() as f64) / step).ceil() as i64;
    for yi in y_start..=y_end {
        let sy = (y_offset - yi as f64 * step) as f32;
        let color = if yi % major_every == 0 { GRID_MAJOR_COLOR } else { GRID_COLOR };
        painter.line_segment(
            [Pos2::new(viewport.left(), sy), Pos2::new(viewport.right(), sy)],
            Stroke::new(1.0, color),
        );
    }
}

fn draw_shapes(painter: &Painter, state: &CanvasState, app: &OxideApp, origin: Pos2) {
    // Visual offset applied to selected shapes during a move drag.
    let move_delta: (f64, f64) = match &state.select_drag {
        Some(SelectDrag { start, current, kind: SelectDragKind::Move }) => {
            (current.0 - start.0, current.1 - start.1)
        }
        _ => (0.0, 0.0),
    };
    let is_moving = move_delta.0 != 0.0 || move_delta.1 != 0.0;

    if let (Some(proj), Some(cell_name)) = (&app.project, &app.active_cell) {
        if let Some(cell) = proj.library.cell(cell_name) {
            let mut shapes: Vec<&oxide_db::shape::Shape> = cell.layout.shapes.iter().collect();
            shapes.sort_by_key(|s| app.tech.layer(&s.layer).map(|l| l.z_order).unwrap_or(0));

            for shape in &shapes {
                if !app.layers_visible.get(&shape.layer).copied().unwrap_or(true) {
                    continue;
                }
                let r = shape.bounding_rect();
                let r = if is_moving && app.selected.contains(&shape.id) {
                    r.translated(move_delta.0, move_delta.1)
                } else {
                    r
                };
                let screen_rect = oxide_rect_to_egui(state, &r, origin);
                let color = app
                    .tech
                    .layer(&shape.layer)
                    .map(|l| layer_color(l.color))
                    .unwrap_or(Color32::GRAY);
                painter.rect_filled(screen_rect, 0.0, color);
                if app.selected.contains(&shape.id) {
                    painter.rect_stroke(
                        screen_rect,
                        0.0,
                        Stroke::new(2.0, Color32::WHITE),
                        egui::StrokeKind::Middle,
                    );
                }
            }

            // DRC error overlays (at DB positions, not displaced)
            let drc_ids: std::collections::HashSet<ShapeId> =
                app.drc_results.iter().flat_map(|v| v.shapes.iter().copied()).collect();
            for shape in &cell.layout.shapes {
                if drc_ids.contains(&shape.id) {
                    let screen_rect = oxide_rect_to_egui(state, &shape.bounding_rect(), origin);
                    painter.rect_filled(screen_rect, 0.0, DRC_HIGHLIGHT);
                }
            }
        }
    }
}

fn draw_overlays(
    painter: &Painter,
    state: &CanvasState,
    app: &OxideApp,
    origin: Pos2,
    lx: f64,
    ly: f64,
    sx: f64,
    sy: f64,
) {
    // DrawRect ghost
    if let ToolMode::DrawRect { layer } = &app.tool {
        if let Some((ax, ay)) = state.draw_start {
            let tl = state.lambda_to_screen(ax.min(sx), ay.max(sy), origin);
            let br = state.lambda_to_screen(ax.max(sx), ay.min(sy), origin);
            let color = app
                .tech
                .layer(layer)
                .map(|l| {
                    let c = l.color;
                    Color32::from_rgba_premultiplied(c[0], c[1], c[2], 100)
                })
                .unwrap_or(Color32::from_gray(100));
            let ghost = Rect::from_min_max(tl, br);
            painter.rect_filled(ghost, 0.0, color);
            painter.rect_stroke(ghost, 0.0, Stroke::new(1.0, Color32::WHITE), egui::StrokeKind::Middle);
        }
    }

    // Box-select rubber band
    if let Some(SelectDrag { start, current, kind: SelectDragKind::BoxSelect }) = &state.select_drag {
        let (x0, y0) = *start;
        let (x1, y1) = *current;
        let tl = state.lambda_to_screen(x0.min(x1), y0.max(y1), origin);
        let br = state.lambda_to_screen(x0.max(x1), y0.min(y1), origin);
        painter.rect_stroke(
            Rect::from_min_max(tl, br),
            0.0,
            Stroke::new(1.0, BOX_SELECT_COLOR),
            egui::StrokeKind::Middle,
        );
    }

    // Suppress unused-variable warnings for unneeded coords in this function
    let _ = (lx, ly);
}

// ── Interaction handlers ──────────────────────────────────────────────────────

fn update_draw_rect(
    response: &egui::Response,
    app: &mut OxideApp,
    sx: f64,
    sy: f64,
    layer: String,
) {
    if response.drag_started_by(egui::PointerButton::Primary) {
        app.canvas.draw_start = Some((sx, sy));
    }

    if response.drag_stopped() {
        if let Some((ax, ay)) = app.canvas.draw_start.take() {
            let (x0, x1) = (ax.min(sx), ax.max(sx));
            let (y0, y1) = (ay.min(sy), ay.max(sy));
            if x1 - x0 > 0.1 && y1 - y0 > 0.1 {
                // Assign ID, then commit via command (so it's undoable)
                let (cell_name, id) = {
                    let cn = app.active_cell.clone().unwrap_or_default();
                    let id = app
                        .project
                        .as_mut()
                        .and_then(|p| p.library.cell_mut(&cn))
                        .map(|c| c.layout.next_shape_id())
                        .unwrap_or(oxide_db::shape::ShapeId(0));
                    (cn, id)
                };
                let shape = oxide_db::shape::Shape::new_rect(
                    id,
                    layer,
                    oxide_db::geometry::Rect::from_corners(x0, y0, x1, y1),
                );
                app.apply_cmd(Command::AddShape { cell: cell_name, shape });
            }
        }
    }
}

fn update_select(
    response: &egui::Response,
    ui: &egui::Ui,
    app: &mut OxideApp,
    lx: f64,
    ly: f64,
    sx: f64,
    sy: f64,
) {
    let shift = ui.ctx().input(|i| i.modifiers.shift);
    let hovered_id = hit_test(app, lx, ly);

    // Drag start
    if response.drag_started_by(egui::PointerButton::Primary) {
        if let Some(id) = hovered_id {
            if !app.selected.contains(&id) {
                if !shift { app.selected.clear(); }
                app.selected.insert(id);
            }
            app.canvas.select_drag = Some(SelectDrag {
                start: (sx, sy),
                current: (sx, sy),
                kind: SelectDragKind::Move,
            });
        } else {
            if !shift { app.selected.clear(); }
            app.canvas.select_drag = Some(SelectDrag {
                start: (lx, ly),
                current: (lx, ly),
                kind: SelectDragKind::BoxSelect,
            });
        }
    }

    // Update current position each frame while dragging
    if response.dragged_by(egui::PointerButton::Primary) {
        if let Some(drag) = &mut app.canvas.select_drag {
            drag.current = match drag.kind {
                SelectDragKind::Move => (sx, sy),
                SelectDragKind::BoxSelect => (lx, ly),
            };
        }
    }

    // Drag released — commit
    if response.drag_stopped() {
        if let Some(drag) = app.canvas.select_drag.take() {
            match drag.kind {
                SelectDragKind::Move => {
                    let dx = drag.current.0 - drag.start.0;
                    let dy = drag.current.1 - drag.start.1;
                    if dx.abs() > 0.5 || dy.abs() > 0.5 {
                        let ids: Vec<ShapeId> = app.selected.iter().cloned().collect();
                        let cell = app.active_cell.clone().unwrap_or_default();
                        app.apply_cmd(Command::MoveShapes { cell, ids, dx, dy });
                    }
                }
                SelectDragKind::BoxSelect => {
                    let (x0, y0) = drag.start;
                    let (x1, y1) = drag.current;
                    let box_r = oxide_db::geometry::Rect::from_corners(x0, y0, x1, y1);
                    if let (Some(proj), Some(cell_name)) = (&app.project, &app.active_cell) {
                        if let Some(cell) = proj.library.cell(cell_name) {
                            for shape in &cell.layout.shapes {
                                if box_r.intersects(&shape.bounding_rect()) {
                                    app.selected.insert(shape.id);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Click (no significant drag) — toggle or single select
    if response.clicked() {
        if let Some(id) = hovered_id {
            if shift {
                if app.selected.contains(&id) {
                    app.selected.remove(&id);
                } else {
                    app.selected.insert(id);
                }
            } else {
                app.selected.clear();
                app.selected.insert(id);
            }
        } else if !shift {
            app.selected.clear();
        }
    }
}

fn hit_test(app: &OxideApp, lx: f64, ly: f64) -> Option<ShapeId> {
    let proj = app.project.as_ref()?;
    let cell_name = app.active_cell.as_ref()?;
    let cell = proj.library.cell(cell_name)?;
    // Pick the topmost (highest z-order) shape under the cursor.
    let mut best: Option<(u32, ShapeId)> = None;
    for shape in &cell.layout.shapes {
        let r = shape.bounding_rect();
        if lx >= r.x && lx <= r.x1() && ly >= r.y && ly <= r.y1() {
            let z = app.tech.layer(&shape.layer).map(|l| l.z_order).unwrap_or(0);
            if best.map_or(true, |(bz, _)| z >= bz) {
                best = Some((z, shape.id));
            }
        }
    }
    best.map(|(_, id)| id)
}
