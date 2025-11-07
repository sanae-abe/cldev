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

    // Initialize output handler with global flags
    let output = OutputHandler::new(cli.verbose, cli.quiet, cli.no_color);

    // Route to appropriate command handler
    match &cli.command {
        Commands::Config(cmd) => {
            output.info("Config command execution - implementation pending");
            handle_config_command(cmd, &output)
        }
        Commands::Dev(cmd) => {
            output.info("Dev command execution - implementation pending");
            handle_dev_command(cmd, &output)
        }
        Commands::Git(cmd) => {
            output.info("Git command execution - implementation pending");
            handle_git_command(cmd, &output)
        }
        Commands::Quality(cmd) => {
            output.info("Quality command execution - implementation pending");
            handle_quality_command(cmd, &output)
        }
        Commands::Tech(cmd) => {
            output.info("Tech command execution - implementation pending");
            handle_tech_command(cmd, &output)
        }
        Commands::Ops(cmd) => {
            output.info("Ops command execution - implementation pending");
            handle_ops_command(cmd, &output)
        }
        Commands::Analysis(cmd) => {
            output.info("Analysis command execution - implementation pending");
            handle_analysis_command(cmd, &output)
        }
        Commands::Lr(cmd) => {
            output.info("Learning Record command execution - implementation pending");
            handle_lr_command(cmd, &output)
        }
        Commands::Todo(cmd) => {
            output.info("Todo command execution - implementation pending");
            handle_todo_command(cmd, &output)
        }
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
fn handle_config_maintain(_backup: bool, _cleanup: bool, output: &OutputHandler) -> Result<()> {
    output.info("Configuration maintenance not yet fully implemented");
    output.info("This will include:");
    output.list_item("Backup configuration files");
    output.list_item("Clean up old backups");
    output.list_item("Validate all configuration files");
    output.list_item("Report configuration health");
    Ok(())
}

/// Update documentation
fn handle_config_update_docs(
    _doc_type: Option<&cli::args::DocType>,
    _validate: bool,
    output: &OutputHandler,
) -> Result<()> {
    output.info("Documentation update not yet fully implemented");
    output.info("This will include:");
    output.list_item("Update implementation documentation");
    output.list_item("Update API documentation");
    output.list_item("Update architecture documentation");
    output.list_item("Validate documentation completeness");
    Ok(())
}

fn handle_dev_command(cmd: &cli::args::DevCommands, output: &OutputHandler) -> Result<()> {
    use cli::args::DevCommands;

    match cmd {
        DevCommands::Urgent { problem, yes: _ } => {
            commands::dev::handle_urgent(Some(problem.clone()))
        }
        DevCommands::Fix { target, branch: _ } => commands::dev::handle_fix(Some(target.clone())),
        DevCommands::Debug {
            symptom,
            verbose: _,
        } => commands::dev::handle_debug(Some(symptom.clone())),
        DevCommands::Feature {
            name,
            skip_confirm: _,
        } => commands::dev::handle_feature(Some(name.clone())),
        DevCommands::Refactor { target, scope: _ } => {
            commands::dev::handle_refactor(Some(target.clone()))
        }
        DevCommands::Optimize { target, focus: _ } => {
            commands::dev::handle_optimize(Some(target.clone()))
        }
        DevCommands::Research { topic, format: _ } => {
            commands::dev::handle_research(Some(topic.clone()))
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
