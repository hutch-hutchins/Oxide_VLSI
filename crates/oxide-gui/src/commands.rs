use oxide_db::project::Project;
use oxide_db::shape::{Shape, ShapeId};

pub enum Command {
    AddShape { cell: String, shape: Shape },
    DeleteShapes { cell: String, shapes: Vec<Shape> },
    MoveShapes { cell: String, ids: Vec<ShapeId>, dx: f64, dy: f64 },
}

impl Command {
    fn apply(&self, project: &mut Project) {
        match self {
            Command::AddShape { cell, shape } => {
                if let Some(c) = project.library.cell_mut(cell) {
                    c.layout.add_shape(shape.clone());
                }
            }
            Command::DeleteShapes { cell, shapes } => {
                if let Some(c) = project.library.cell_mut(cell) {
                    for s in shapes {
                        c.layout.remove_shape(s.id);
                    }
                }
            }
            Command::MoveShapes { cell, ids, dx, dy } => {
                if let Some(c) = project.library.cell_mut(cell) {
                    c.layout.translate_shapes(ids, *dx, *dy);
                }
            }
        }
    }

    fn unapply(&self, project: &mut Project) {
        match self {
            Command::AddShape { cell, shape } => {
                if let Some(c) = project.library.cell_mut(cell) {
                    c.layout.remove_shape(shape.id);
                }
            }
            Command::DeleteShapes { cell, shapes } => {
                if let Some(c) = project.library.cell_mut(cell) {
                    for s in shapes {
                        c.layout.add_shape(s.clone());
                    }
                }
            }
            Command::MoveShapes { cell, ids, dx, dy } => {
                if let Some(c) = project.library.cell_mut(cell) {
                    c.layout.translate_shapes(ids, -*dx, -*dy);
                }
            }
        }
    }
}

#[derive(Default)]
pub struct CommandStack {
    undo: Vec<Command>,
    redo: Vec<Command>,
}

impl CommandStack {
    pub fn push(&mut self, cmd: Command, project: &mut Project) {
        cmd.apply(project);
        self.undo.push(cmd);
        self.redo.clear();
    }

    pub fn undo(&mut self, project: &mut Project) -> bool {
        if let Some(cmd) = self.undo.pop() {
            cmd.unapply(project);
            self.redo.push(cmd);
            true
        } else {
            false
        }
    }

    pub fn redo(&mut self, project: &mut Project) -> bool {
        if let Some(cmd) = self.redo.pop() {
            cmd.apply(project);
            self.undo.push(cmd);
            true
        } else {
            false
        }
    }

    pub fn can_undo(&self) -> bool { !self.undo.is_empty() }
    pub fn can_redo(&self) -> bool { !self.redo.is_empty() }

    pub fn clear(&mut self) {
        self.undo.clear();
        self.redo.clear();
    }
}
