use std::path::Path;

use anyhow::{Context, Result};
use rusqlite::Connection;

use crate::platform::AppPaths;

pub struct Storage {
    database_path: std::path::PathBuf,
}

impl Storage {
    pub fn bootstrap(paths: &AppPaths) -> Result<Self> {
        paths.ensure()?;

        let connection = Connection::open(paths.database_path()).with_context(|| {
            format!(
                "failed to open local SQLite database at {}",
                paths.database_path().display()
            )
        })?;

        connection.execute_batch(
            "CREATE TABLE IF NOT EXISTS app_meta (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );
            INSERT OR IGNORE INTO app_meta (key, value) VALUES ('schema_version', '0');",
        )?;

        Ok(Self {
            database_path: paths.database_path().to_path_buf(),
        })
    }

    pub fn database_path(&self) -> &Path {
        &self.database_path
    }
}
