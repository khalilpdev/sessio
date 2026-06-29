use anyhow::Result;

use crate::{app::AppContext, cli::Command};

pub fn execute(command: Command, context: &AppContext) -> Result<()> {
    let message = match command {
        Command::Index { full } => index_message(context, full),
        Command::Search { query } => search_message(context, &query),
        Command::Recent => recent_message(context),
        Command::Continue => continue_message(context),
        Command::Dashboard => dashboard_message(context),
    };

    println!("{message}");
    Ok(())
}

fn index_message(context: &AppContext, full: bool) -> String {
    let mode = if full {
        "full reindex"
    } else {
        "incremental index"
    };

    format!(
        "Sessio is bootstrapped and ready for {mode} flows.\nLocal data directory: {}\nSQLite database: {}\nProject and session indexing will be added next, but this command now provides the stable CLI entrypoint for future indexers.",
        context.data_dir().display(),
        context.database_path().display()
    )
}

fn search_message(context: &AppContext, query: &str) -> String {
    let trimmed = query.trim();

    if trimmed.is_empty() {
        return format!(
            "Please provide a non-empty query. Search storage is prepared at {}.",
            context.database_path().display()
        );
    }

    format!(
        "Search for \"{trimmed}\" is not implemented yet.\nSQLite storage is ready at {}\nThis placeholder keeps the CLI stable while full-text indexing and ranking are built.",
        context.database_path().display()
    )
}

fn recent_message(context: &AppContext) -> String {
    format!(
        "No recent development timeline is available yet.\nSessio has prepared local storage at {}\nRecent activity views will become available once indexing lands.",
        context.database_path().display()
    )
}

fn continue_message(context: &AppContext) -> String {
    format!(
        "There is no resumable context yet.\nRun `sessio index` after more indexers land to populate {}\nThe `continue` command is now reserved for future session handoff workflows.",
        context.database_path().display()
    )
}

fn dashboard_message(context: &AppContext) -> String {
    format!(
        "The local dashboard is not available yet.\nSessio has already reserved cross-platform state under {}\nA future release will launch a local UI backed by the same SQLite store.",
        context.data_dir().display()
    )
}
