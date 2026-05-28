use crate::library::Library;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use thiserror::Error;

pub const PROJECT_FORMAT_VERSION: u32 = 1;

#[derive(Debug, Error)]
pub enum ProjectError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("TOML parse error: {0}")]
    Toml(#[from] toml::de::Error),
    #[error("RON serialize error: {0}")]
    RonSer(#[from] ron::Error),
    #[error("RON deserialize error: {0}")]
    RonDe(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMeta {
    pub version: u32,
    pub name: String,
    pub technology: String,
    pub cells: Vec<String>,
}

/// An Oxide VLSI project. Lives on disk as a directory.
///
/// ```text
/// my_project/
///   project.toml
///   cells/
///     inv/
///       layout.ron
///       metadata.toml
///   exports/
///   reports/
/// ```
#[derive(Debug, Clone)]
pub struct Project {
    pub meta: ProjectMeta,
    pub library: Library,
    /// Root directory on disk; None for unsaved projects
    pub path: Option<PathBuf>,
}

impl Project {
    pub fn new(name: impl Into<String>, technology: impl Into<String>) -> Self {
        let n = name.into();
        Self {
            meta: ProjectMeta {
                version: PROJECT_FORMAT_VERSION,
                name: n.clone(),
                technology: technology.into(),
                cells: Vec::new(),
            },
            library: Library::new(n),
            path: None,
        }
    }

    pub fn save(&self, dir: &Path) -> Result<(), ProjectError> {
        std::fs::create_dir_all(dir)?;
        let cells_dir = dir.join("cells");
        std::fs::create_dir_all(&cells_dir)?;
        std::fs::create_dir_all(dir.join("exports"))?;
        std::fs::create_dir_all(dir.join("reports"))?;

        // Write project.toml
        let meta_str = toml::to_string_pretty(&self.meta)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
        std::fs::write(dir.join("project.toml"), meta_str)?;

        // Write each cell
        for (cell_name, cell) in &self.library.cells {
            let cell_dir = cells_dir.join(cell_name);
            std::fs::create_dir_all(&cell_dir)?;

            let layout_str = ron::ser::to_string_pretty(&cell.layout, Default::default())
                .map_err(ProjectError::RonSer)?;
            std::fs::write(cell_dir.join("layout.ron"), layout_str)?;

            let meta_str = toml::to_string_pretty(&cell.metadata)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
            std::fs::write(cell_dir.join("metadata.toml"), meta_str)?;
        }

        Ok(())
    }

    pub fn load(dir: &Path) -> Result<Self, ProjectError> {
        let meta_text = std::fs::read_to_string(dir.join("project.toml"))?;
        let meta: ProjectMeta = toml::from_str(&meta_text)?;

        let mut library = Library::new(&meta.name);
        let cells_dir = dir.join("cells");

        for cell_name in &meta.cells {
            let cell_dir = cells_dir.join(cell_name);
            let layout_text = std::fs::read_to_string(cell_dir.join("layout.ron"))?;
            let layout = ron::from_str(&layout_text)
                .map_err(|e| ProjectError::RonDe(e.to_string()))?;
            let meta_text = std::fs::read_to_string(cell_dir.join("metadata.toml"))?;
            let cell_meta = toml::from_str(&meta_text)?;

            library.add_cell(crate::cell::Cell {
                name: cell_name.clone(),
                layout,
                metadata: cell_meta,
            });
        }

        Ok(Self { meta, library, path: Some(dir.to_owned()) })
    }
}
