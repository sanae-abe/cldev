use clap::{Parser, Subcommand};

/// Claude Dev CLI - Unified development environment management tool
#[derive(Parser, Debug)]
#[command(name = "cldev")]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Disable colored output
    #[arg(long, global = true)]
    pub no_color: bool,

    #[command(subcommand)]
    pub command: Commands,
}

/// Available commands
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Configuration management commands
    #[command(subcommand)]
    Config(ConfigCommands),

    /// Check system health and configuration
    Health {
        /// Show detailed health information
        #[arg(short, long)]
        detailed: bool,
    },

    /// Manage development sessions
    Session {
        /// Session name
        name: Option<String>,

        /// List all sessions
        #[arg(short, long)]
        list: bool,
    },

    /// Quick start development environment
    Start {
        /// Project type (web, api, mobile, data-science)
        #[arg(short, long)]
        project_type: Option<String>,
    },
}

/// Configuration subcommands
#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    /// Validate and check configuration
    Check {
        /// Show all validation details
        #[arg(short, long)]
        verbose: bool,

        /// Path to configuration file (defaults to ~/.config/cldev/cldev.toml)
        #[arg(short, long)]
        path: Option<String>,
    },

    /// Edit configuration file
    Edit {
        /// Editor to use (overrides EDITOR env var and config)
        #[arg(short, long)]
        editor: Option<String>,

        /// Path to configuration file
        #[arg(short, long)]
        path: Option<String>,
    },

    /// List all available commands
    List {
        /// Filter by category
        #[arg(short, long)]
        category: Option<String>,

        /// Show detailed information
        #[arg(short, long)]
        detailed: bool,
    },

    /// Initialize configuration file
    Init {
        /// Force overwrite existing configuration
        #[arg(short, long)]
        force: bool,

        /// Path to create configuration file
        #[arg(short, long)]
        path: Option<String>,
    },

    /// Show configuration file path
    Path,
}
