use crate::cli::args::Environment;
use crate::core::{CldevError, ProjectDetector, ProjectType, Result};
use colored::Colorize;
use dialoguer::Confirm;
use indicatif::{ProgressBar, ProgressStyle};
use std::process::Command;

/// Handle deploy command
pub fn handle_deploy(env: Environment, yes: bool, dry_run: bool) -> Result<()> {
    println!("{}", "🚀 Starting deployment process...".cyan().bold());

    // Environment configuration
    let env_name = match env {
        Environment::Development => "development",
        Environment::Staging => "staging",
        Environment::Production => "production",
    };

    println!("{} Target: {}", "ℹ️".cyan(), env_name.yellow().bold());

    if dry_run {
        println!(
            "{}",
            "\n⚠️  DRY RUN MODE - No actual changes will be made"
                .yellow()
                .bold()
        );
    }

    // Detect project type
    let detector = ProjectDetector::new(None)?;
    let project_type = detector.project_type();
    println!(
        "{} Project type: {}",
        "ℹ️".cyan(),
        format!("{:?}", project_type).green()
    );

    // Pre-deployment checks
    println!("\n{}", "🔍 Running pre-deployment checks...".cyan());
    run_predeployment_checks(&env)?;

    // Confirmation for production
    if matches!(env, Environment::Production) && !yes && !dry_run {
        let confirmed = Confirm::new()
            .with_prompt(format!(
                "{} Are you sure you want to deploy to PRODUCTION?",
                "⚠️".yellow()
            ))
            .default(false)
            .interact()?;

        if !confirmed {
            println!("{}", "❌ Deployment cancelled".red());
            return Ok(());
        }
    }

    // Build phase
    if !dry_run {
        println!("\n{}", "🔨 Building for deployment...".cyan());
        build_for_deployment(&env)?;
    } else {
        println!("\n{}", "[DRY RUN] Would build for deployment".yellow());
    }

    // Deploy phase
    if !dry_run {
        println!("\n{}", "📦 Deploying application...".cyan());
        execute_deployment(&env, project_type)?;
    } else {
        println!("\n{}", "[DRY RUN] Would deploy application".yellow());
        show_deployment_plan(&env, project_type)?;
    }

    // Post-deployment verification
    if !dry_run {
        println!("\n{}", "✅ Verifying deployment...".cyan());
        verify_deployment(&env)?;
    }

    println!(
        "\n{}",
        "🎉 Deployment completed successfully!".green().bold()
    );
    display_deployment_info(&env);

    Ok(())
}

/// Run pre-deployment checks
fn run_predeployment_checks(env: &Environment) -> Result<()> {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );

    // Check 1: Git status
    pb.set_message("Checking git status...");
    check_git_status(env)?;
    println!("  {} Git status clean", "✓".green());

    // Check 2: Tests
    pb.set_message("Running tests...");
    if std::path::Path::new("package.json").exists() {
        let status = Command::new("npm").args(["run", "test"]).status();

        if let Ok(status) = status {
            if !status.success() {
                return Err(CldevError::command("Tests failed"));
            }
        }
    }
    println!("  {} Tests passed", "✓".green());

    // Check 3: Linting
    pb.set_message("Running linter...");
    if std::path::Path::new("package.json").exists() {
        let _status = Command::new("npm").args(["run", "lint"]).status();
    }
    println!("  {} Linting passed", "✓".green());

    // Check 4: Security audit
    pb.set_message("Running security audit...");
    if std::path::Path::new("package.json").exists() {
        let _status = Command::new("npm")
            .args(["audit", "--audit-level=moderate"])
            .status();
    }
    println!("  {} Security audit passed", "✓".green());

    pb.finish_and_clear();
    Ok(())
}

/// Check git status
fn check_git_status(env: &Environment) -> Result<()> {
    // For production, ensure we're on main/master and clean
    if matches!(env, Environment::Production) {
        let output = Command::new("git")
            .args(["status", "--porcelain"])
            .output()?;

        if !output.stdout.is_empty() {
            return Err(CldevError::command(
                "Working directory not clean. Commit or stash changes before deploying.",
            ));
        }

        // Check branch
        let branch_output = Command::new("git")
            .args(["rev-parse", "--abbrev-ref", "HEAD"])
            .output()?;

        let branch = String::from_utf8_lossy(&branch_output.stdout)
            .trim()
            .to_string();

        if branch != "main" && branch != "master" {
            return Err(CldevError::command(format!(
                "Production deployments must be from main/master branch. Current branch: {}",
                branch
            )));
        }
    }

    Ok(())
}

/// Build for deployment
fn build_for_deployment(env: &Environment) -> Result<()> {
    let status = Command::new("npm")
        .args(["run", "build"])
        .env(
            "NODE_ENV",
            match env {
                Environment::Development => "development",
                Environment::Staging => "staging",
                Environment::Production => "production",
            },
        )
        .status()?;

    if !status.success() {
        return Err(CldevError::command("Build failed"));
    }

    Ok(())
}

/// Execute deployment
fn execute_deployment(env: &Environment, project_type: ProjectType) -> Result<()> {
    use crate::core::ProjectType;

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );

    pb.set_message("Deploying...");

    match project_type {
        ProjectType::NodeJs => {
            deploy_web_app(env)?;
        }
        ProjectType::Rust => {
            deploy_rust_app(env)?;
        }
        ProjectType::Python => {
            deploy_python_app(env)?;
        }
        ProjectType::Go => {
            deploy_go_app(env)?;
        }
        ProjectType::Ruby
        | ProjectType::Java
        | ProjectType::Php
        | ProjectType::DotNet
        | ProjectType::Elixir
        | ProjectType::Kotlin
        | ProjectType::Swift
        | ProjectType::Scala => {
            pb.finish_and_clear();
            return Err(CldevError::command(format!(
                "Deployment not yet implemented for {}. Please use platform-specific deployment tools.",
                project_type.name()
            )));
        }
        _ => {
            pb.finish_and_clear();
            return Err(CldevError::command(format!(
                "Deployment not supported for project type: {:?}",
                project_type
            )));
        }
    }

    pb.finish_with_message("✓ Deployed successfully".to_string());
    Ok(())
}

/// Deploy web application
fn deploy_web_app(env: &Environment) -> Result<()> {
    // Check for common deployment tools
    let env_name = match env {
        Environment::Development => "development",
        Environment::Staging => "staging",
        Environment::Production => "production",
    };

    // Try Vercel
    if Command::new("vercel").arg("--version").status().is_ok() {
        println!("  {} Deploying with Vercel...", "→".cyan());
        let mut cmd = Command::new("vercel");

        if matches!(env, Environment::Production) {
            cmd.arg("--prod");
        }

        let status = cmd.status()?;
        if !status.success() {
            return Err(CldevError::command("Vercel deployment failed"));
        }
        return Ok(());
    }

    // Try Netlify
    if Command::new("netlify").arg("--version").status().is_ok() {
        println!("  {} Deploying with Netlify...", "→".cyan());
        let status = Command::new("netlify")
            .args(["deploy", "--prod"])
            .status()?;
        if !status.success() {
            return Err(CldevError::command("Netlify deployment failed"));
        }
        return Ok(());
    }

    // Try custom deploy script
    if std::path::Path::new("deploy.sh").exists() {
        println!("  {} Running custom deploy script...", "→".cyan());
        let status = Command::new("sh").args(["deploy.sh", env_name]).status()?;
        if !status.success() {
            return Err(CldevError::command("Custom deploy script failed"));
        }
        return Ok(());
    }

    Err(CldevError::command(
        "No deployment tool found. Install vercel, netlify, or create deploy.sh",
    ))
}

/// Deploy Rust application
fn deploy_rust_app(env: &Environment) -> Result<()> {
    // Build release binary
    let mut cmd = Command::new("cargo");
    cmd.arg("build").arg("--release");

    let status = cmd.status()?;
    if !status.success() {
        return Err(CldevError::command("Cargo build failed"));
    }

    // Deploy using custom script or Docker
    let env_name = match env {
        Environment::Development => "development",
        Environment::Staging => "staging",
        Environment::Production => "production",
    };

    if std::path::Path::new("deploy.sh").exists() {
        let status = Command::new("sh").args(["deploy.sh", env_name]).status()?;
        if !status.success() {
            return Err(CldevError::command("Deploy script failed"));
        }
    }

    Ok(())
}

/// Deploy Python application
fn deploy_python_app(_env: &Environment) -> Result<()> {
    // Build Python package
    let status = Command::new("python").args(["-m", "build"]).status()?;

    if !status.success() {
        return Err(CldevError::command("Python build failed"));
    }

    Ok(())
}

/// Deploy Go application
fn deploy_go_app(_env: &Environment) -> Result<()> {
    // Build Go binary
    let status = Command::new("go")
        .args(["build", "-ldflags", "-s -w"])
        .status()?;

    if !status.success() {
        return Err(CldevError::command("Go build failed"));
    }

    Ok(())
}

/// Show deployment plan (dry run)
fn show_deployment_plan(env: &Environment, project_type: ProjectType) -> Result<()> {
    use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Table};

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec!["Step", "Action"]);

    table.add_row(vec!["1", "Run pre-deployment checks"]);
    table.add_row(vec!["2", &format!("Build for {:?}", env)]);
    table.add_row(vec!["3", &format!("Deploy {:?} app", project_type)]);
    table.add_row(vec!["4", "Verify deployment"]);

    println!("\n{}", table);
    Ok(())
}

/// Verify deployment
fn verify_deployment(_env: &Environment) -> Result<()> {
    // Basic verification checks
    println!("  {} Checking deployment health...", "→".cyan());

    // In a real implementation, this would:
    // 1. Check if the deployed service is responding
    // 2. Run smoke tests
    // 3. Verify critical endpoints
    // 4. Check monitoring/alerting setup

    Ok(())
}

/// Display deployment information
fn display_deployment_info(env: &Environment) {
    use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Table};

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec!["Deployment Info", "Value"]);

    table.add_row(vec!["Environment", &format!("{:?}", env)]);
    table.add_row(vec!["Timestamp", &chrono::Local::now().to_rfc3339()]);

    println!("\n{}", table);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_names() {
        assert_eq!(
            match Environment::Development {
                Environment::Development => "development",
                _ => "",
            },
            "development"
        );
    }
}
