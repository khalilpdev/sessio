use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "sessio",
    version,
    about = "Cross-platform developer memory platform",
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Clone, Subcommand, PartialEq, Eq)]
pub enum Command {
    /// Index project and session data into the local store
    Index {
        /// Force a full reindex instead of incremental discovery
        #[arg(long)]
        full: bool,
    },
    /// Search indexed development context
    Search {
        /// Query text to search for
        query: String,
    },
    /// Show the most recent development activity
    Recent,
    /// Resume the best available development context
    #[command(name = "continue")]
    Continue,
    /// Launch the future local dashboard
    Dashboard,
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use super::{Cli, Command};

    #[test]
    fn parses_index_command() {
        let cli = Cli::parse_from(["sessio", "index", "--full"]);
        assert_eq!(cli.command, Some(Command::Index { full: true }));
    }

    #[test]
    fn parses_search_command() {
        let cli = Cli::parse_from(["sessio", "search", "jwt"]);
        assert_eq!(
            cli.command,
            Some(Command::Search {
                query: "jwt".to_string(),
            })
        );
    }

    #[test]
    fn parses_recent_command() {
        let cli = Cli::parse_from(["sessio", "recent"]);
        assert_eq!(cli.command, Some(Command::Recent));
    }

    #[test]
    fn parses_continue_command() {
        let cli = Cli::parse_from(["sessio", "continue"]);
        assert_eq!(cli.command, Some(Command::Continue));
    }

    #[test]
    fn parses_dashboard_command() {
        let cli = Cli::parse_from(["sessio", "dashboard"]);
        assert_eq!(cli.command, Some(Command::Dashboard));
    }
}
