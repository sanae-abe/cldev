mod cli;
mod commands;
mod core;

use clap::Parser;
use cli::args::{Cli, Commands, ConfigCommands};
use cli::output::OutputHandler;
use core::error::Result;

fn main() {
    if let Err(e) = run() {
        let output = OutputHandler::default();
        output.error(&format!("Error: {}", e));
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    // Initialize i18n help system before clap parsing
    cli::help::init_help_i18n(cli.lang.to_i18n());

    // Initialize output handler with global flags and language
    let output =
        OutputHandler::with_language(cli.verbose, cli.quiet, cli.no_color, cli.lang.to_i18n());

    // Route to appropriate command handler
    match &cli.command {
        Commands::Config(cmd) => handle_config_command(cmd, &output),
        Commands::Dev(cmd) => handle_dev_command(cmd, &output),
        Commands::Git(cmd) => handle_git_command(cmd, &output),
        Commands::Quality(cmd) => handle_quality_command(cmd, &output),
        Commands::Tech(cmd) => handle_tech_command(cmd, &output),
        Commands::Ops(cmd) => handle_ops_command(cmd, &output),
        Commands::Analysis(cmd) => handle_analysis_command(cmd, &output),
        Commands::Lr(cmd) => handle_lr_command(cmd, &output),
        Commands::Todo(cmd) => handle_todo_command(cmd, &output),
        Commands::Completions { shell, install } => {
            handle_completions_command(*shell, *install, &output)
        }
    }
}

// Config command handler - Phase 1-A implementation
fn handle_config_command(cmd: &ConfigCommands, output: &OutputHandler) -> Result<()> {
    match cmd {
        ConfigCommands::Init { defaults, force } => handle_config_init(*defaults, *force, output),
        ConfigCommands::Check { detailed, fix } => {
            commands::config::check_config(None, *detailed, *fix, output)
        }
        ConfigCommands::Edit { target } => {
            output.debug(&format!("Editing config target: {:?}", target));
            commands::config::edit_config(None, None, output)
        }
        ConfigCommands::List { detailed, filter } => {
            let filter_str = filter.as_ref().map(|f| format!("{:?}", f));
            commands::config::list_commands(filter_str, *detailed, output)
        }
        ConfigCommands::Maintain { backup, cleanup } => {
            handle_config_maintain(*backup, *cleanup, output)
        }
        ConfigCommands::UpdateDocs { doc_type, validate } => {
            handle_config_update_docs(doc_type.as_ref(), *validate, output)
        }
    }
}

/// Initialize configuration
fn handle_config_init(defaults: bool, force: bool, output: &OutputHandler) -> Result<()> {
    use core::config::Config;

    // If --defaults flag is set, skip interactive mode and create default config
    if defaults {
        let config_path = Config::default_path()?;

        if config_path.exists() && !force {
            let msg = output.t("config-already-exists");
            output.warning(&format!("{}: {}", msg, config_path.display()));
            output.info(&output.t("use-force-to-overwrite"));
            return Ok(());
        }

        output.info(&output.t("creating-default-config"));

        let config = Config::default();
        config.save(Some(config_path.clone()))?;

        let success_msg = output.t_format(
            "config-created-at",
            "path",
            &config_path.display().to_string(),
        );
        output.success(&format!("✅ {}", success_msg));

        output.info("\n💡 Next steps:");
        output.list_item(&output.t_format("next-step", "command", "cldev config edit"));
        output.list_item(&output.t_format("next-step", "command", "cldev config check"));
        output.list_item(&output.t_format("next-step", "command", "cldev config list"));

        return Ok(());
    }

    // Run interactive setup wizard
    commands::config::run_interactive_init(force, output)
}

/// Maintain configuration files
fn handle_config_maintain(backup: bool, cleanup: bool, output: &OutputHandler) -> Result<()> {
    use core::config::Config;
    use std::fs;
    use std::path::PathBuf;

    let config_path = Config::default_path()?;

    // Validate configuration
    output.info("🔍 Validating configuration...");
    match Config::load(None) {
        Ok(_) => output.success("✅ Configuration is valid"),
        Err(e) => {
            output.error(&format!("❌ Configuration validation failed: {}", e));
            return Err(e);
        }
    }

    // Perform backup if requested
    if backup {
        output.info("\n📦 Creating configuration backup...");

        if !config_path.exists() {
            output.warning("No configuration file found to backup");
        } else {
            let backup_dir = config_path
                .parent()
                .ok_or_else(|| core::error::CldevError::config("Invalid config path"))?
                .join("backups");

            fs::create_dir_all(&backup_dir).map_err(|e| {
                core::error::CldevError::io(format!("Failed to create backup directory: {}", e))
            })?;

            let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
            let backup_path = backup_dir.join(format!("config.toml.{}", timestamp));

            fs::copy(&config_path, &backup_path).map_err(|e| {
                core::error::CldevError::io(format!("Failed to create backup: {}", e))
            })?;

            output.success(&format!("✅ Backup created: {}", backup_path.display()));
        }
    }

    // Cleanup old backups if requested
    if cleanup {
        output.info("\n🧹 Cleaning up old backups...");

        let backup_dir = config_path
            .parent()
            .ok_or_else(|| core::error::CldevError::config("Invalid config path"))?
            .join("backups");

        if !backup_dir.exists() {
            output.info("No backup directory found");
        } else {
            let mut backups: Vec<PathBuf> = fs::read_dir(&backup_dir)
                .map_err(|e| {
                    core::error::CldevError::io(format!("Failed to read backup directory: {}", e))
                })?
                .filter_map(|entry| entry.ok())
                .map(|entry| entry.path())
                .filter(|path| {
                    path.is_file()
                        && path
                            .file_name()
                            .and_then(|n| n.to_str())
                            .map(|n| n.starts_with("config.toml."))
                            .unwrap_or(false)
                })
                .collect();

            if backups.is_empty() {
                output.info("No backups found");
            } else {
                // Sort by modification time (newest first)
                backups.sort_by_key(|path| {
                    fs::metadata(path)
                        .and_then(|m| m.modified())
                        .ok()
                        .map(|t| std::time::SystemTime::now().duration_since(t).ok())
                        .flatten()
                });

                let keep_count = 10;
                let remove_count = backups.len().saturating_sub(keep_count);

                if remove_count > 0 {
                    output.info(&format!(
                        "Keeping {} most recent backups, removing {} old backups",
                        keep_count, remove_count
                    ));

                    for backup in backups.iter().skip(keep_count) {
                        match fs::remove_file(backup) {
                            Ok(_) => output.success(&format!(
                                "  Removed: {}",
                                backup.file_name().unwrap().to_string_lossy()
                            )),
                            Err(e) => output.warning(&format!(
                                "  Failed to remove {}: {}",
                                backup.display(),
                                e
                            )),
                        }
                    }
                } else {
                    output.info(&format!("Found {} backups (keeping all)", backups.len()));
                }
            }
        }
    }

    // Report configuration health
    output.info("\n📊 Configuration Health Report:");
    output.list_item(&format!("Config location: {}", config_path.display()));
    output.list_item(&format!("Config exists: {}", config_path.exists()));

    if config_path.exists() {
        if let Ok(metadata) = fs::metadata(&config_path) {
            output.list_item(&format!("Config size: {} bytes", metadata.len()));

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mode = metadata.permissions().mode();
                let mode_str = format!("{:o}", mode & 0o777);
                output.list_item(&format!("Permissions: {}", mode_str));
            }
        }
    }

    if !backup && !cleanup {
        output.info("\n💡 Tip: Use --backup to create a backup or --cleanup to remove old backups");
    }

    Ok(())
}

/// Update documentation
fn handle_config_update_docs(
    doc_type: Option<&cli::args::DocType>,
    validate: bool,
    output: &OutputHandler,
) -> Result<()> {
    use std::path::Path;

    // If no doc type specified, show available options
    if doc_type.is_none() {
        output.section("Available Documentation Types");
        output.list_item("implementation - Code implementation documentation");
        output.list_item("api - API reference documentation");
        output.list_item("architecture - Architecture and design documentation");
        output.raw("");
        output.info("Usage: cldev config update-docs --type <TYPE>");
        output.info("Add --validate to check documentation completeness");
        return Ok(());
    }

    let doc_type = doc_type.unwrap();

    // Validation mode
    if validate {
        output.section("Validating Documentation");

        let docs_dir = Path::new("docs");
        if !docs_dir.exists() {
            output.warning("docs/ directory not found");
            output.info("Consider creating documentation structure:");
            output.list_item("docs/implementation/");
            output.list_item("docs/api/");
            output.list_item("docs/architecture/");
            return Ok(());
        }

        // Count markdown files
        let mut total_docs = 0;
        if let Ok(entries) = std::fs::read_dir(docs_dir) {
            for entry in entries.flatten() {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        if let Some(ext) = entry.path().extension() {
                            if ext == "md" {
                                total_docs += 1;
                            }
                        }
                    } else if file_type.is_dir() {
                        // Count files in subdirectories
                        if let Ok(sub_entries) = std::fs::read_dir(entry.path()) {
                            for sub_entry in sub_entries.flatten() {
                                if let Some(ext) = sub_entry.path().extension() {
                                    if ext == "md" {
                                        total_docs += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        output.success(&format!(
            "Found {} markdown documentation file(s)",
            total_docs
        ));
        output.info("Documentation structure:");
        output.list_item(&format!("Location: {}", docs_dir.display()));
        output.list_item(&format!("Total .md files: {}", total_docs));
    }

    // Show doc type specific message
    output.section(&format!(
        "Updating {} Documentation",
        match doc_type {
            cli::args::DocType::Implementation => "Implementation",
            cli::args::DocType::Api => "API",
            cli::args::DocType::Architecture => "Architecture",
        }
    ));

    match doc_type {
        cli::args::DocType::Implementation => {
            output.info("Implementation documentation update will include:");
            output.list_item("Scan source code for modules and functions");
            output.list_item("Extract inline documentation comments");
            output.list_item("Generate usage examples");
            output.list_item("Update implementation guides");
        }
        cli::args::DocType::Api => {
            output.info("API documentation update will include:");
            output.list_item("Extract API endpoint definitions");
            output.list_item("Document request/response schemas");
            output.list_item("Generate API examples and curl commands");
            output.list_item("Update API reference documentation");
        }
        cli::args::DocType::Architecture => {
            output.info("Architecture documentation update will include:");
            output.list_item("Analyze project structure and dependencies");
            output.list_item("Generate component diagrams");
            output.list_item("Document design patterns and decisions");
            output.list_item("Update architecture guides");
        }
    }

    output.raw("");
    output.warning("Full implementation coming soon");
    output.info("Documentation will be generated in: docs/");

    Ok(())
}

fn handle_dev_command(cmd: &cli::args::DevCommands, output: &OutputHandler) -> Result<()> {
    use cli::args::DevCommands;

    match cmd {
        DevCommands::Urgent { problem, yes: _ } => {
            commands::dev::handle_urgent(Some(problem.clone()), output)
        }
        DevCommands::Fix { target, branch: _ } => {
            commands::dev::handle_fix(Some(target.clone()), output)
        }
        DevCommands::Debug {
            symptom,
            verbose: _,
        } => commands::dev::handle_debug(Some(symptom.clone()), output),
        DevCommands::Feature {
            name,
            skip_confirm: _,
        } => commands::dev::handle_feature(Some(name.clone()), output),
        DevCommands::Refactor { target, scope: _ } => {
            commands::dev::handle_refactor(Some(target.clone()), output)
        }
        DevCommands::Optimize { target, focus: _ } => {
            commands::dev::handle_optimize(Some(target.clone()), output)
        }
        DevCommands::Research { topic, format: _ } => {
            commands::dev::handle_research(Some(topic.clone()), output)
        }
    }
}

fn handle_git_command(cmd: &cli::args::GitCommands, output: &OutputHandler) -> Result<()> {
    use cli::args::GitCommands;

    match cmd {
        GitCommands::Commit {
            message,
            no_verify,
            amend,
        } => commands::git::create_commit(message.clone(), *no_verify, *amend, output),
        GitCommands::Branch { name, branch_type } => {
            commands::git::create_branch(name.clone(), *branch_type, output)
        }
        GitCommands::MergeRequest {
            target,
            title,
            detailed,
        } => commands::git::create_merge_request(target, title.clone(), *detailed, output),
        GitCommands::Status { detailed } => commands::git::show_status(*detailed, output),
    }
}

fn handle_quality_command(cmd: &cli::args::QualityCommands, output: &OutputHandler) -> Result<()> {
    use cli::args::QualityCommands;

    match cmd {
        QualityCommands::Lint { fix, paths } => {
            output.debug(&format!("Lint command - fix: {}, paths: {:?}", fix, paths));
            commands::quality::run_lint(paths, *fix, output)
        }
        QualityCommands::Format { check, paths } => {
            output.debug(&format!(
                "Format command - check: {}, paths: {:?}",
                check, paths
            ));
            commands::quality::format_code(paths, *check, output)
        }
        QualityCommands::Test {
            pattern,
            coverage,
            watch,
        } => {
            output.debug(&format!(
                "Test command - pattern: {:?}, coverage: {}, watch: {}",
                pattern, coverage, watch
            ));
            commands::quality::run_tests(pattern.as_deref(), *coverage, *watch, output)
        }
    }
}

fn handle_tech_command(cmd: &cli::args::TechCommands, output: &OutputHandler) -> Result<()> {
    use cli::args::TechCommands;

    match cmd {
        TechCommands::Start { stack, port, env } => {
            output.debug(&format!("Starting tech stack: {:?}", stack));
            commands::tech::handle_start(*stack, *port, *env)
        }
    }
}

fn handle_ops_command(cmd: &cli::args::OpsCommands, output: &OutputHandler) -> Result<()> {
    use cli::args::OpsCommands;

    match cmd {
        OpsCommands::Build {
            env,
            analyze,
            clean,
        } => {
            output.debug(&format!("Building for environment: {:?}", env));
            commands::ops::handle_build(*env, *analyze, *clean)
        }
        OpsCommands::Deploy { env, yes, dry_run } => {
            output.debug(&format!("Deploying to environment: {:?}", env));
            commands::ops::handle_deploy(*env, *yes, *dry_run)
        }
    }
}

fn handle_analysis_command(
    cmd: &cli::args::AnalysisCommands,
    output: &OutputHandler,
) -> Result<()> {
    use cli::args::AnalysisCommands;

    match cmd {
        AnalysisCommands::Analyze {
            target,
            format,
            detailed,
        } => commands::analyze_project(*target, *format, *detailed, output),
        AnalysisCommands::Explain {
            target,
            examples,
            detailed,
        } => commands::explain_target(target, *examples, *detailed, output),
        AnalysisCommands::ReviewMr {
            number,
            detailed,
            security_focus,
            performance_focus,
        } => commands::review_merge_request(
            *number,
            *detailed,
            *security_focus,
            *performance_focus,
            output,
        ),
        AnalysisCommands::Serena { mode, targets } => commands::run_serena(*mode, targets, output),
    }
}

fn handle_lr_command(cmd: &cli::args::LrCommands, output: &OutputHandler) -> Result<()> {
    use cli::args::LrCommands;

    match cmd {
        LrCommands::New { topic, edit } => {
            output.debug(&format!("Creating new learning record: {}", topic));
            commands::lr::handle_new(topic.clone(), *edit)
        }
        LrCommands::Find {
            query,
            field,
            limit,
        } => {
            output.debug(&format!("Searching learning records: {}", query));
            commands::lr::handle_find(query.clone(), *field, *limit)
        }
        LrCommands::Stats { period, detailed } => {
            output.debug(&format!("Generating learning statistics: {:?}", period));
            commands::lr::handle_stats(*period, *detailed)
        }
        LrCommands::Problems { priority, recent } => {
            output.debug("Analyzing problem patterns");
            commands::lr::handle_problems(*priority, *recent)
        }
    }
}

fn handle_todo_command(cmd: &cli::args::TodoCommands, output: &OutputHandler) -> Result<()> {
    use cli::args::TodoCommands;

    match cmd {
        TodoCommands::Manage {
            action,
            description,
        } => {
            output.debug(&format!("Managing todos: {:?}", action));
            commands::todo::handle_manage(*action, description.clone())
        }
    }
}

fn handle_completions_command(
    shell: cli::args::Shell,
    install: bool,
    output: &OutputHandler,
) -> Result<()> {
    use cli::{generate_completions, print_installation_instructions};

    output.debug(&format!("Generating completions for: {:?}", shell));

    // Generate completions to stdout
    generate_completions(shell);

    // Print installation instructions if requested
    if install {
        print_installation_instructions(shell);
    }

    Ok(())
}
