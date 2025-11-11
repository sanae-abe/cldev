mod cli;
mod commands;
mod core;

use crate::core::error::Result;
use clap::Parser;
use cli::args::{Cli, Commands, ConfigCommands};
use cli::output::OutputHandler;

fn main() {
    if let Err(e) = run() {
        let output = OutputHandler::default();
        output.error(&format!("Error: {}", e));
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    // Initialize i18n help system after clap parsing
    cli::help::init_help_i18n(cli.lang.to_i18n());

    // Initialize output handler with global flags and language
    let mut output =
        OutputHandler::with_language(cli.verbose, cli.quiet, cli.no_color, cli.lang.to_i18n());

    // Track command execution in active session (if any)
    let command_name = format!("{:?}", cli.command);
    let start_time = std::time::Instant::now();

    // Route to appropriate command handler
    let result = match &cli.command {
        Commands::Config(cmd) => handle_config_command(cmd, &mut output),
        Commands::Dev(cmd) => handle_dev_command(cmd, &output),
        Commands::Git(cmd) => handle_git_command(cmd, &output),
        Commands::Quality(cmd) => handle_quality_command(cmd, &output),
        Commands::Tech(cmd) => handle_tech_command(cmd, &output),
        Commands::Ops(cmd) => handle_ops_command(cmd, &output),
        Commands::Analysis(cmd) => handle_analysis_command(cmd, &output),
        Commands::Lr(cmd) => handle_lr_command(cmd, &output),
        Commands::Todo(cmd) => handle_todo_command(cmd, &output),
        Commands::Session(cmd) => handle_session_command(cmd, &output),
        Commands::Completions { shell, install } => {
            handle_completions_command(*shell, *install, &output)
        }
    };

    // Update session context if active
    let execution_time = start_time.elapsed();
    update_session_context(&command_name, &result, execution_time);

    result
}

// Config command handler - Phase 1-A implementation
fn handle_config_command(cmd: &ConfigCommands, output: &mut OutputHandler) -> Result<()> {
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
        ConfigCommands::Maintain {
            backup,
            cleanup,
            archive,
            retention_days,
        } => commands::config::handle_config_maintain(
            *backup,
            *cleanup,
            *archive,
            *retention_days,
            output,
        ),
        ConfigCommands::UpdateDocs { doc_type, validate } => {
            commands::config::handle_update_docs(doc_type.as_ref(), *validate, output)
        }
    }
}

/// Initialize configuration
fn handle_config_init(defaults: bool, force: bool, output: &mut OutputHandler) -> Result<()> {
    use crate::core::config::Config;

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
        LrCommands::CheckFile { file_path } => {
            output.debug(&format!("Checking file hotspot: {}", file_path));
            commands::lr::handle_check_file(file_path)
        }
        LrCommands::Suggest {
            error_msg,
            threshold,
            limit,
        } => {
            output.debug(&format!("Suggesting similar errors for: {}", error_msg));
            commands::lr::handle_suggest(error_msg, *threshold, *limit)
        }
        LrCommands::Similar { session_id, limit } => {
            output.debug(&format!("Finding similar sessions to: {}", session_id));
            commands::lr::handle_similar(session_id, *limit)
        }
    }
}

fn handle_todo_command(cmd: &cli::args::TodoCommands, output: &OutputHandler) -> Result<()> {
    use cli::args::TodoCommands;
    use commands::todo::manage::{
        add_todo as add_todo_impl, complete_todo, interactive_mode, list_todos, sync_todos,
    };

    match cmd {
        TodoCommands::Add { description } => {
            output.debug("Adding todo");
            add_todo_impl(description.clone())
        }
        TodoCommands::List => {
            output.debug("Listing todos");
            list_todos()
        }
        TodoCommands::Complete => {
            output.debug("Completing todo");
            complete_todo()
        }
        TodoCommands::Sync => {
            output.debug("Syncing todos with git");
            sync_todos()
        }
        TodoCommands::Interactive => {
            output.debug("Starting interactive mode");
            interactive_mode()
        }
    }
}

fn handle_session_command(cmd: &cli::SessionCommand, output: &OutputHandler) -> Result<()> {
    output.debug("Session command");
    cli::handle_session(cmd.clone())
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
        print_installation_instructions(shell, output);
    }

    Ok(())
}

/// Update session context if an active session exists
fn update_session_context(
    command_name: &str,
    result: &Result<()>,
    execution_time: std::time::Duration,
) {
    use crate::core::{CommandRecord, ErrorCapture, SessionContext};
    use std::path::PathBuf;

    // Get session path
    let session_path = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".cldev")
        .join("current-session.json");

    // Skip if no active session
    if !session_path.exists() {
        return;
    }

    // Load session context
    let Ok(json) = std::fs::read_to_string(&session_path) else {
        return;
    };
    let Ok(mut ctx) = serde_json::from_str::<SessionContext>(&json) else {
        return;
    };

    // Add command record
    let exit_code = if result.is_ok() { 0 } else { 1 };
    let working_dir = std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .display()
        .to_string();

    ctx.add_command(CommandRecord {
        command: command_name.to_string(),
        exit_code,
        execution_time_ms: execution_time.as_millis() as u64,
        timestamp: chrono::Local::now(),
        working_dir,
    });

    // Add error if command failed
    if let Err(e) = result {
        ctx.add_error(ErrorCapture {
            timestamp: chrono::Local::now(),
            error_type: "CommandError".to_string(),
            message: format!("{}", e),
            context: Some(command_name.to_string()),
            resolved: false,
        });
    }

    // Save updated context
    if let Ok(json) = serde_json::to_string_pretty(&ctx) {
        let _ = std::fs::write(&session_path, json);
    }
}
