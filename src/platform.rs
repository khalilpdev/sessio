use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use directories::ProjectDirs;

pub struct AppPaths {
    data_dir: PathBuf,
    database_path: PathBuf,
}

impl AppPaths {
    pub fn detect() -> Result<Self> {
        let project_dirs = ProjectDirs::from("dev", "khalilpdev", "sessio")
            .context("failed to determine the local Sessio data directory")?;

        let data_dir = project_dirs.data_local_dir().to_path_buf();
        let database_path = data_dir.join("sessio.db");

        Ok(Self {
            data_dir,
            database_path,
        })
    }

    pub fn ensure(&self) -> Result<()> {
        std::fs::create_dir_all(&self.data_dir)
            .with_context(|| format!("failed to create {}", self.data_dir.display()))
    }

    pub fn data_dir(&self) -> &Path {
        &self.data_dir
    }

    pub fn database_path(&self) -> &Path {
        &self.database_path
    }
}
