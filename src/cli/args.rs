use clap::{Parser, Subcommand, ValueEnum};

// Re-export Shell from clap_complete for use in command definitions
pub use clap_complete::Shell;

/// Claude Dev CLI - Unified development environment management tool
#[derive(Parser, Debug)]
#[command(
    name = "cldev",
    version,
    author,
    about = "Claude Dev CLI - Unified development environment management tool",
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

    /// Set language (en/ja)
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

impl Language {
    /// Convert CLI language to i18n language
    pub fn to_i18n(self) -> crate::core::i18n::Language {
        match self {
            Language::En => crate::core::i18n::Language::English,
            Language::Ja => crate::core::i18n::Language::Japanese,
        }
    }
}

/// Top-level commands organized by category
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Configuration management commands
    #[command(subcommand, about = super::help::config_about())]
    Config(ConfigCommands),

    /// Development workflow commands
    #[command(subcommand, about = super::help::dev_about())]
    Dev(DevCommands),

    /// Git operation commands
    #[command(subcommand, about = super::help::git_about())]
    Git(GitCommands),

    /// Code quality commands
    #[command(subcommand, about = super::help::quality_about())]
    Quality(QualityCommands),

    /// Tech stack specific commands
    #[command(subcommand, about = super::help::tech_about())]
    Tech(TechCommands),

    /// Operations commands
    #[command(subcommand, about = super::help::ops_about())]
    Ops(OpsCommands),

    /// Analysis and review commands
    #[command(subcommand, about = super::help::analysis_about())]
    Analysis(AnalysisCommands),

    /// Learning record commands
    #[command(subcommand, about = super::help::lr_about())]
    Lr(LrCommands),

    /// Todo management commands
    #[command(subcommand, about = super::help::todo_about())]
    Todo(TodoCommands),

    /// Development session management
    #[command(subcommand, about = super::help::session_about())]
    Session(super::SessionCommand),

    /// Generate shell completions
    #[command(about = super::help::completions_about())]
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum, help = super::help::completions_shell_help())]
        shell: Shell,

        /// Print installation instructions
        #[arg(short, long, help = super::help::completions_install_help())]
        install: bool,
    },
}

// ============================================================================
// Configuration Commands
// ============================================================================

#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    /// Initialize cldev configuration
    #[command(about = super::help::config_init_about())]
    Init {
        /// Skip interactive prompts and use defaults
        #[arg(short, long, help = super::help::config_init_defaults_help())]
        defaults: bool,

        /// Force initialization even if config exists
        #[arg(short, long, help = super::help::config_init_force_help())]
        force: bool,
    },

    /// Check configuration health
    #[command(about = super::help::config_check_about())]
    Check {
        /// Perform detailed validation
        #[arg(short, long, help = super::help::config_check_detailed_help())]
        detailed: bool,

        /// Fix issues automatically if possible
        #[arg(short, long, help = super::help::config_check_fix_help())]
        fix: bool,
    },

    /// Edit configuration file
    #[command(about = super::help::config_edit_about())]
    Edit {
        /// Configuration file to edit (global/project/stack)
        #[arg(value_enum, default_value = "global", help = super::help::config_edit_target_help())]
        target: ConfigTarget,
    },

    #[command(about = super::help::config_list_about())]
    List {
        #[arg(short, long, help = super::help::config_list_detailed_help())]
        detailed: bool,

        #[arg(short, long, value_enum, help = super::help::config_list_filter_help())]
        filter: Option<ConfigTarget>,
    },

    #[command(about = super::help::config_maintain_about())]
    Maintain {
        #[arg(short, long, help = super::help::config_maintain_backup_help())]
        backup: bool,

        #[arg(short, long, help = super::help::config_maintain_cleanup_help())]
        cleanup: bool,

        #[arg(long, help = super::help::config_maintain_archive_help())]
        archive: bool,

        #[arg(long, help = super::help::config_maintain_retention_days_help())]
        retention_days: Option<i64>,
    },

    #[command(about = super::help::config_update_docs_about())]
    UpdateDocs {
        #[arg(value_enum, help = super::help::config_update_docs_type_help())]
        doc_type: Option<DocType>,

        #[arg(long, help = super::help::config_update_docs_validate_help())]
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
    #[command(about = super::help::dev_urgent_about())]
    Urgent {
        #[arg(help = super::help::dev_urgent_problem_help())]
        problem: String,

        #[arg(short = 'y', long, help = super::help::dev_urgent_yes_help())]
        yes: bool,
    },

    #[command(about = super::help::dev_fix_about())]
    Fix {
        #[arg(help = super::help::dev_fix_target_help())]
        target: String,

        #[arg(short, long, help = super::help::dev_fix_branch_help())]
        branch: bool,
    },

    #[command(about = super::help::dev_debug_about())]
    Debug {
        #[arg(help = super::help::dev_debug_symptom_help())]
        symptom: String,

        #[arg(short, long, help = super::help::dev_debug_verbose_help())]
        verbose: bool,
    },

    #[command(about = super::help::dev_feature_about())]
    Feature {
        #[arg(help = super::help::dev_feature_name_help())]
        name: String,

        #[arg(short, long, help = super::help::dev_feature_skip_confirm_help())]
        skip_confirm: bool,
    },

    #[command(about = super::help::dev_refactor_about())]
    Refactor {
        #[arg(help = super::help::dev_refactor_target_help())]
        target: String,

        #[arg(short, long, value_enum, default_value = "module", help = super::help::dev_refactor_scope_help())]
        scope: RefactorScope,
    },

    #[command(about = super::help::dev_optimize_about())]
    Optimize {
        #[arg(help = super::help::dev_optimize_target_help())]
        target: String,

        #[arg(short, long, value_enum, help = super::help::dev_optimize_focus_help())]
        focus: Option<OptimizationFocus>,
    },

    #[command(about = super::help::dev_research_about())]
    Research {
        #[arg(help = super::help::dev_research_topic_help())]
        topic: String,

        #[arg(short, long, value_enum, default_value = "markdown", help = super::help::dev_research_format_help())]
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
    #[command(about = super::help::git_commit_about())]
    Commit {
        #[arg(help = super::help::git_commit_message_help())]
        message: Option<String>,

        #[arg(long, help = super::help::git_commit_no_verify_help())]
        no_verify: bool,

        #[arg(long, help = super::help::git_commit_amend_help())]
        amend: bool,
    },

    #[command(about = super::help::git_branch_about())]
    Branch {
        #[arg(help = super::help::git_branch_name_help())]
        name: Option<String>,

        #[arg(short, long, value_enum, help = super::help::git_branch_type_help())]
        branch_type: Option<BranchType>,
    },

    #[command(about = super::help::git_merge_request_about())]
    MergeRequest {
        #[arg(short, long, default_value = "main", help = super::help::git_merge_request_target_help())]
        target: String,

        #[arg(help = super::help::git_merge_request_title_help())]
        title: Option<String>,

        #[arg(short, long, help = super::help::git_merge_request_detailed_help())]
        detailed: bool,
    },

    #[command(about = super::help::git_status_about())]
    Status {
        #[arg(short, long, help = super::help::git_status_detailed_help())]
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
    #[command(about = super::help::quality_lint_about())]
    Lint {
        #[arg(short, long, help = super::help::quality_lint_fix_help())]
        fix: bool,

        #[arg(help = super::help::quality_lint_paths_help())]
        paths: Vec<String>,
    },

    #[command(about = super::help::quality_format_about())]
    Format {
        #[arg(short, long, help = super::help::quality_format_check_help())]
        check: bool,

        #[arg(help = super::help::quality_format_paths_help())]
        paths: Vec<String>,
    },

    #[command(about = super::help::quality_test_about())]
    Test {
        #[arg(help = super::help::quality_test_pattern_help())]
        pattern: Option<String>,

        #[arg(short, long, help = super::help::quality_test_coverage_help())]
        coverage: bool,

        #[arg(short, long, help = super::help::quality_test_watch_help())]
        watch: bool,
    },
}

// ============================================================================
// Tech Stack Commands
// ============================================================================

#[derive(Subcommand, Debug)]
pub enum TechCommands {
    #[command(about = super::help::tech_start_about())]
    Start {
        #[arg(value_enum, help = super::help::tech_start_stack_help())]
        stack: TechStack,

        #[arg(short, long, help = super::help::tech_start_port_help())]
        port: Option<u16>,

        #[arg(short, long, value_enum, default_value = "development", help = super::help::tech_start_env_help())]
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
    #[command(about = super::help::ops_build_about())]
    Build {
        #[arg(short, long, value_enum, default_value = "production", help = super::help::ops_build_env_help())]
        env: Environment,

        #[arg(short, long, help = super::help::ops_build_analyze_help())]
        analyze: bool,

        #[arg(short, long, help = super::help::ops_build_clean_help())]
        clean: bool,
    },

    #[command(about = super::help::ops_deploy_about())]
    Deploy {
        #[arg(value_enum, help = super::help::ops_deploy_env_help())]
        env: Environment,

        #[arg(short = 'y', long, help = super::help::ops_deploy_yes_help())]
        yes: bool,

        #[arg(short, long, help = super::help::ops_deploy_dry_run_help())]
        dry_run: bool,
    },
}

// ============================================================================
// Analysis Commands
// ============================================================================

#[derive(Subcommand, Debug)]
pub enum AnalysisCommands {
    #[command(about = super::help::analysis_analyze_about())]
    Analyze {
        #[arg(value_enum, default_value = "overview", help = super::help::analysis_analyze_target_help())]
        target: AnalysisTarget,

        #[arg(short, long, value_enum, default_value = "text", help = super::help::analysis_analyze_format_help())]
        format: AnalysisFormat,

        #[arg(short, long, help = super::help::analysis_analyze_detailed_help())]
        detailed: bool,
    },

    #[command(about = super::help::analysis_explain_about())]
    Explain {
        #[arg(help = super::help::analysis_explain_target_help())]
        target: String,

        #[arg(short, long, help = super::help::analysis_explain_examples_help())]
        examples: bool,

        #[arg(short, long, help = super::help::analysis_explain_detailed_help())]
        detailed: bool,
    },

    #[command(about = super::help::analysis_review_mr_about())]
    ReviewMr {
        #[arg(help = super::help::analysis_review_mr_number_help())]
        number: u32,

        #[arg(short, long, help = super::help::analysis_review_mr_detailed_help())]
        detailed: bool,

        #[arg(long, help = super::help::analysis_review_mr_security_focus_help())]
        security_focus: bool,

        #[arg(long, help = super::help::analysis_review_mr_performance_focus_help())]
        performance_focus: bool,
    },

    #[command(about = super::help::analysis_serena_about())]
    Serena {
        #[arg(value_enum, default_value = "interactive", help = super::help::analysis_serena_mode_help())]
        mode: SerenaMode,

        #[arg(help = super::help::analysis_serena_targets_help())]
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
    #[command(about = super::help::lr_find_about())]
    Find {
        #[arg(help = super::help::lr_find_query_help())]
        query: String,

        #[arg(short, long, value_enum, help = super::help::lr_find_field_help())]
        field: Option<SearchField>,

        #[arg(short, long, default_value = "10", help = super::help::lr_find_limit_help())]
        limit: usize,
    },

    #[command(about = super::help::lr_stats_about())]
    Stats {
        #[arg(short, long, value_enum, default_value = "month", help = super::help::lr_stats_period_help())]
        period: TimePeriod,

        #[arg(short, long, help = super::help::lr_stats_detailed_help())]
        detailed: bool,
    },

    #[command(about = super::help::lr_problems_about())]
    Problems {
        #[arg(short, long, value_enum, help = super::help::lr_problems_priority_help())]
        priority: Option<Priority>,

        #[arg(short, long, help = super::help::lr_problems_recent_help())]
        recent: bool,
    },

    #[command(about = super::help::lr_new_about())]
    New {
        #[arg(help = super::help::lr_new_topic_help())]
        topic: String,

        #[arg(short, long, help = super::help::lr_new_edit_help())]
        edit: bool,
    },

    #[command(about = super::help::lr_check_file_about())]
    CheckFile {
        #[arg(help = super::help::lr_check_file_path_help())]
        file_path: String,
    },

    #[command(about = super::help::lr_suggest_about())]
    Suggest {
        #[arg(help = super::help::lr_suggest_error_help())]
        error_msg: String,

        #[arg(short, long, default_value = "0.7", help = super::help::lr_suggest_threshold_help())]
        threshold: Option<f64>,

        #[arg(short, long, default_value = "10", help = super::help::lr_suggest_limit_help())]
        limit: Option<usize>,
    },

    #[command(about = super::help::lr_similar_about())]
    Similar {
        #[arg(help = super::help::lr_similar_session_id_help())]
        session_id: String,

        #[arg(short, long, default_value = "10", help = super::help::lr_similar_limit_help())]
        limit: Option<usize>,
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
    /// Add a new todo item
    #[command(about = super::help::todo_add_about())]
    Add {
        #[arg(help = super::help::todo_add_description_help())]
        description: Option<String>,
    },

    /// List all todo items
    #[command(about = super::help::todo_list_about())]
    List,

    /// Mark a todo item as completed
    #[command(about = super::help::todo_complete_about())]
    Complete,

    /// Sync todos with git commits
    #[command(about = super::help::todo_sync_about())]
    Sync,

    /// Interactive todo management mode
    #[command(about = super::help::todo_interactive_about())]
    Interactive,
}

/// Extract language flag from command-line arguments before clap parsing
/// This allows i18n help messages to be displayed in the correct language
pub fn extract_language_from_args() -> crate::core::i18n::Language {
    let args: Vec<String> = std::env::args().collect();

    // Look for --lang flag
    for i in 0..args.len() {
        if args[i] == "--lang" {
            if let Some(lang_str) = args.get(i + 1) {
                return match lang_str.as_str() {
                    "ja" => crate::core::i18n::Language::Japanese,
                    _ => crate::core::i18n::Language::English,
                };
            }
        }
    }

    // Check environment variable CLDEV_LANG as fallback
    if let Ok(env_lang) = std::env::var("CLDEV_LANG") {
        return match env_lang.as_str() {
            "ja" => crate::core::i18n::Language::Japanese,
            _ => crate::core::i18n::Language::English,
        };
    }

    // Default to English
    crate::core::i18n::Language::English
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parsing() {
        // Initialize i18n before creating CLI command
        super::super::help::init_help_i18n(crate::core::i18n::Language::English);

        use clap::CommandFactory;
        Cli::command().debug_assert();
    }

    #[test]
    fn test_language_enum() {
        assert!(matches!(Language::En, Language::En));
        assert!(matches!(Language::Ja, Language::Ja));
    }
}
