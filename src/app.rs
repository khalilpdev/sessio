use anyhow::Result;
use clap::{CommandFactory, Parser};

use crate::{cli::Cli, commands, platform::AppPaths, storage::Storage};

pub struct AppContext {
    paths: AppPaths,
    storage: Storage,
}

impl AppContext {
    pub fn bootstrap() -> Result<Self> {
        let paths = AppPaths::detect()?;
        let storage = Storage::bootstrap(&paths)?;

        Ok(Self { paths, storage })
    }

    pub fn data_dir(&self) -> &std::path::Path {
        self.paths.data_dir()
    }

    pub fn database_path(&self) -> &std::path::Path {
        self.storage.database_path()
    }
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(command) => {
            let context = AppContext::bootstrap()?;
            commands::execute(command, &context)
        }
        None => {
            let mut command = Cli::command();
            command.print_help()?;
            println!();
            Ok(())
        }
    }
}
