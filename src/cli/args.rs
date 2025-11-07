use clap::{Parser, Subcommand, ValueEnum};

// Re-export Shell from clap_complete for use in command definitions
pub use clap_complete::Shell;

/// Claude Dev CLI - Unified development environment management tool
#[derive(Parser, Debug)]
#[command(
    name = "cldev",
    version,
    author,
    about = "Unified development environment management tool with AI-powered workflow",
    long_about = None,
    propagate_version = true
)]
pub struct Cli {
    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Suppress non-error output
    #[arg(short, long, global = true, conflicts_with = "verbose")]
    pub quiet: bool,

    /// Disable colored output
    #[arg(long, global = true)]
    pub no_color: bool,

    /// Set language (ja/en)
    #[arg(long, global = true, value_enum, default_value = "en")]
    pub lang: Language,

    #[command(subcommand)]
    pub command: Commands,
}

/// Language selection
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Language {
    /// English
    En,
    /// Japanese
    Ja,
}

/// Top-level commands organized by category
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Configuration management commands
    #[command(subcommand)]
    Config(ConfigCommands),

    /// Development workflow commands
    #[command(subcommand)]
    Dev(DevCommands),

    /// Git operation commands
    #[command(subcommand)]
    Git(GitCommands),

    /// Code quality commands
    #[command(subcommand)]
    Quality(QualityCommands),

    /// Tech stack specific commands
    #[command(subcommand)]
    Tech(TechCommands),

    /// Operations commands
    #[command(subcommand)]
    Ops(OpsCommands),

    /// Analysis and review commands
    #[command(subcommand)]
    Analysis(AnalysisCommands),

    /// Learning record commands
    #[command(subcommand)]
    Lr(LrCommands),

    /// Todo management commands
    #[command(subcommand)]
    Todo(TodoCommands),

    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,

        /// Print installation instructions
        #[arg(short, long)]
        install: bool,
    },
}

// ============================================================================
// Configuration Commands
// ============================================================================

#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    /// Initialize cldev configuration
    Init {
        /// Skip interactive prompts and use defaults
        #[arg(short, long)]
        defaults: bool,

        /// Force initialization even if config exists
        #[arg(short, long)]
        force: bool,
    },

    /// Check configuration health
    Check {
        /// Perform detailed validation
        #[arg(short, long)]
        detailed: bool,

        /// Fix issues automatically if possible
        #[arg(short, long)]
        fix: bool,
    },

    /// Edit configuration file
    Edit {
        /// Configuration file to edit (global/project/stack)
        #[arg(value_enum, default_value = "global")]
        target: ConfigTarget,
    },

    /// List all configurations
    List {
        /// Show detailed information
        #[arg(short, long)]
        detailed: bool,

        /// Filter by configuration type
        #[arg(short, long, value_enum)]
        filter: Option<ConfigTarget>,
    },

    /// Maintain configuration files
    Maintain {
        /// Backup configurations before maintenance
        #[arg(short, long)]
        backup: bool,

        /// Clean up old backups
        #[arg(short, long)]
        cleanup: bool,
    },

    /// Update documentation
    UpdateDocs {
        /// Documentation type to update
        #[arg(value_enum)]
        doc_type: Option<DocType>,

        /// Validate documentation after update
        #[arg(long)]
        validate: bool,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ConfigTarget {
    Global,
    Project,
    Stack,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum DocType {
    Implementation,
    Api,
    Architecture,
}

// ============================================================================
// Development Commands
// ============================================================================

#[derive(Subcommand, Debug)]
pub enum DevCommands {
    /// Emergency response for production issues (5min initial response)
    Urgent {
        /// Problem description
        problem: String,

        /// Skip confirmation prompts
        #[arg(short = 'y', long)]
        yes: bool,
    },

    /// Fix critical bugs (same-day resolution target)
    Fix {
        /// Bug or issue to fix
        target: String,

        /// Create fix branch automatically
        #[arg(short, long)]
        branch: bool,
    },

    /// Systematic debugging workflow
    Debug {
        /// Symptom or error description
        symptom: String,

        /// Enable verbose debugging output
        #[arg(short, long)]
        verbose: bool,
    },

    /// Implement new feature (requirements to test)
    Feature {
        /// Feature name or description
        name: String,

        /// Skip requirements confirmation
        #[arg(short, long)]
        skip_confirm: bool,
    },

    /// Safe refactoring (incremental execution)
    Refactor {
        /// Target to refactor
        target: String,

        /// Refactoring scope
        #[arg(short, long, value_enum, default_value = "module")]
        scope: RefactorScope,
    },

    /// Performance optimization (measure -> analyze -> improve)
    Optimize {
        /// Target to optimize
        target: String,

        /// Focus area for optimization
        #[arg(short, long, value_enum)]
        focus: Option<OptimizationFocus>,
    },

    /// Technical research and learning records
    Research {
        /// Research topic
        topic: String,

        /// Output format
        #[arg(short, long, value_enum, default_value = "markdown")]
        format: ResearchFormat,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum RefactorScope {
    Function,
    Module,
    Package,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum OptimizationFocus {
    Performance,
    Memory,
    Bundle,
    Database,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ResearchFormat {
    Markdown,
    Json,
    Html,
}

// ============================================================================
// Git Commands
// ============================================================================

#[derive(Subcommand, Debug)]
pub enum GitCommands {
    /// Create conventional commit
    Commit {
        /// Commit message (optional, will be generated if not provided)
        message: Option<String>,

        /// Skip pre-commit hooks
        #[arg(long)]
        no_verify: bool,

        /// Amend previous commit
        #[arg(long)]
        amend: bool,
    },

    /// Create conventional branch
    Branch {
        /// Branch name (optional, will be generated if not provided)
        name: Option<String>,

        /// Branch type
        #[arg(short, long, value_enum)]
        branch_type: Option<BranchType>,
    },

    /// Create merge request (GitLab) or pull request (GitHub)
    MergeRequest {
        /// Target branch
        #[arg(short, long, default_value = "main")]
        target: String,

        /// MR/PR title (will be generated if not provided)
        title: Option<String>,

        /// Enable detailed mode
        #[arg(short, long)]
        detailed: bool,
    },

    /// Show enhanced git status
    Status {
        /// Show detailed branch information
        #[arg(short, long)]
        detailed: bool,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum BranchType {
    Feature,
    Fix,
    Hotfix,
    Refactor,
    Docs,
    Test,
}

// ============================================================================
// Quality Commands
// ============================================================================

#[derive(Subcommand, Debug)]
pub enum QualityCommands {
    /// Run linter
    Lint {
        /// Auto-fix issues
        #[arg(short, long)]
        fix: bool,

        /// Specific files or patterns
        paths: Vec<String>,
    },

    /// Format code
    Format {
        /// Check formatting without modifying files
        #[arg(short, long)]
        check: bool,

        /// Specific files or patterns
        paths: Vec<String>,
    },

    /// Run tests
    Test {
        /// Run specific test pattern
        pattern: Option<String>,

        /// Generate coverage report
        #[arg(short, long)]
        coverage: bool,

        /// Watch mode
        #[arg(short, long)]
        watch: bool,
    },
}

// ============================================================================
// Tech Stack Commands
// ============================================================================

#[derive(Subcommand, Debug)]
pub enum TechCommands {
    /// Start development environment
    Start {
        /// Tech stack to use
        #[arg(value_enum)]
        stack: TechStack,

        /// Port number
        #[arg(short, long)]
        port: Option<u16>,

        /// Environment (development/production)
        #[arg(short, long, value_enum, default_value = "development")]
        env: Environment,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum TechStack {
    Web,
    Api,
    Mobile,
    DataScience,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Environment {
    Development,
    Staging,
    Production,
}

// ============================================================================
// Operations Commands
// ============================================================================

#[derive(Subcommand, Debug)]
pub enum OpsCommands {
    /// Build project
    Build {
        /// Build environment
        #[arg(short, long, value_enum, default_value = "production")]
        env: Environment,

        /// Analyze bundle after build
        #[arg(short, long)]
        analyze: bool,

        /// Clean before build
        #[arg(short, long)]
        clean: bool,
    },

    /// Deploy project
    Deploy {
        /// Deploy target environment
        #[arg(value_enum)]
        env: Environment,

        /// Skip confirmation prompts
        #[arg(short = 'y', long)]
        yes: bool,

        /// Dry run (show what would be deployed)
        #[arg(short, long)]
        dry_run: bool,
    },
}

// ============================================================================
// Analysis Commands
// ============================================================================

#[derive(Subcommand, Debug)]
pub enum AnalysisCommands {
    /// Analyze project
    Analyze {
        /// Analysis target
        #[arg(value_enum, default_value = "overview")]
        target: AnalysisTarget,

        /// Output format
        #[arg(short, long, value_enum, default_value = "text")]
        format: AnalysisFormat,

        /// Enable detailed analysis
        #[arg(short, long)]
        detailed: bool,
    },

    /// Explain code or concept
    Explain {
        /// Target to explain (function/component/concept name)
        target: String,

        /// Show usage examples
        #[arg(short, long)]
        examples: bool,

        /// Detailed explanation
        #[arg(short, long)]
        detailed: bool,
    },

    /// Review merge request
    ReviewMr {
        /// MR/PR number
        number: u32,

        /// Enable detailed review
        #[arg(short, long)]
        detailed: bool,

        /// Focus on security
        #[arg(long)]
        security_focus: bool,

        /// Focus on performance
        #[arg(long)]
        performance_focus: bool,
    },

    /// Semantic code analysis (Serena MCP)
    Serena {
        /// Analysis mode
        #[arg(value_enum, default_value = "interactive")]
        mode: SerenaMode,

        /// Target files or directories
        targets: Vec<String>,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum AnalysisTarget {
    Structure,
    Performance,
    Quality,
    Debt,
    Overview,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum AnalysisFormat {
    Text,
    Json,
    Html,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum SerenaMode {
    Interactive,
    Batch,
    Watch,
}

// ============================================================================
// Learning Record Commands
// ============================================================================

#[derive(Subcommand, Debug)]
pub enum LrCommands {
    /// Find learning records
    Find {
        /// Search query
        query: String,

        /// Search in specific field
        #[arg(short, long, value_enum)]
        field: Option<SearchField>,

        /// Limit results
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },

    /// Show learning statistics
    Stats {
        /// Time period for statistics
        #[arg(short, long, value_enum, default_value = "month")]
        period: TimePeriod,

        /// Show detailed breakdown
        #[arg(short, long)]
        detailed: bool,
    },

    /// List unsolved problems
    Problems {
        /// Priority filter
        #[arg(short, long, value_enum)]
        priority: Option<Priority>,

        /// Show only recent problems
        #[arg(short, long)]
        recent: bool,
    },

    /// Create new learning record
    New {
        /// Topic name
        topic: String,

        /// Open editor immediately
        #[arg(short, long)]
        edit: bool,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum SearchField {
    Topic,
    Tag,
    Content,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum TimePeriod {
    Day,
    Week,
    Month,
    Year,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

// ============================================================================
// Todo Commands
// ============================================================================

#[derive(Subcommand, Debug)]
pub enum TodoCommands {
    /// Manage todos
    Manage {
        /// Action to perform
        #[arg(value_enum)]
        action: TodoAction,

        /// Todo description (for add action)
        description: Option<String>,
    },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum TodoAction {
    Add,
    List,
    Complete,
    Sync,
    Interactive,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parsing() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }

    #[test]
    fn test_language_enum() {
        assert!(matches!(Language::En, Language::En));
        assert!(matches!(Language::Ja, Language::Ja));
    }
}
