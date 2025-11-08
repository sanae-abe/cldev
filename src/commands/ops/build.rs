use crate::cli::args::Environment;
use crate::core::{CldevError, ProjectDetector, ProjectType, Result};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::process::Command;

/// Handle build command
pub fn handle_build(env: Environment, analyze: bool, clean: bool) -> Result<()> {
    println!("{}", "🏗️  Starting build process...".cyan().bold());

    // Detect project type
    let detector = ProjectDetector::new(None)?;
    let project_type = detector.project_type();
    println!(
        "{} Project type: {}",
        "ℹ️".cyan(),
        format!("{:?}", project_type).green()
    );

    // Environment configuration
    let env_name = match env {
        Environment::Development => "development",
        Environment::Staging => "staging",
        Environment::Production => "production",
    };
    println!("{} Environment: {}", "ℹ️".cyan(), env_name.yellow());

    // Clean if requested
    if clean {
        println!("\n{}", "🧹 Cleaning build artifacts...".yellow());
        clean_build()?;
    }

    // Create progress bar
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );

    // Pre-build checks
    pb.set_message("Running pre-build checks...");
    run_prebuild_checks()?;
    pb.finish_with_message("✓ Pre-build checks passed".to_string());

    // Execute build
    println!("\n{}", "🔨 Building project...".cyan().bold());
    let build_result = execute_build(env, project_type)?;

    if build_result.success {
        println!("{}", "\n✅ Build completed successfully!".green().bold());

        // Analyze bundle if requested
        if analyze {
            println!("\n{}", "📊 Analyzing bundle...".cyan());
            analyze_bundle()?;
        }

        // Show build stats
        display_build_stats(&build_result);

        Ok(())
    } else {
        Err(CldevError::command("Build failed"))
    }
}

/// Clean build artifacts
fn clean_build() -> Result<()> {
    let paths_to_clean = vec!["dist", "build", "target", ".next", "out"];

    for path in paths_to_clean {
        if std::path::Path::new(path).exists() {
            std::fs::remove_dir_all(path)?;
            println!("  {} Removed {}", "✓".green(), path);
        }
    }

    Ok(())
}

/// Run pre-build checks
fn run_prebuild_checks() -> Result<()> {
    // Check if node_modules exists for Node.js projects
    if std::path::Path::new("package.json").exists()
        && !std::path::Path::new("node_modules").exists() {
            println!("  {} Installing dependencies...", "⚠️".yellow());
            let status = Command::new("npm").arg("install").status()?;
            if !status.success() {
                return Err(CldevError::command("Failed to install dependencies"));
            }
        }

    // Check if Cargo.lock exists for Rust projects
    if std::path::Path::new("Cargo.toml").exists() {
        let status = Command::new("cargo").arg("check").status()?;
        if !status.success() {
            return Err(CldevError::command("Cargo check failed"));
        }
    }

    Ok(())
}

/// Build result
struct BuildResult {
    success: bool,
    duration_secs: f64,
    output_size: Option<u64>,
}

/// Execute build based on project type
fn execute_build(env: Environment, project_type: ProjectType) -> Result<BuildResult> {
    use crate::core::ProjectType;

    let start_time = std::time::Instant::now();

    let status = match project_type {
        ProjectType::NodeJs => {
            let mut cmd = Command::new("npm");
            cmd.arg("run");

            match env {
                Environment::Development => cmd.arg("build:dev"),
                Environment::Staging => cmd.arg("build:staging"),
                Environment::Production => cmd.arg("build"),
            };

            cmd.status()?
        }
        ProjectType::Rust => {
            let mut cmd = Command::new("cargo");
            cmd.arg("build");

            if matches!(env, Environment::Production) {
                cmd.arg("--release");
            }

            cmd.status()?
        }
        ProjectType::Python => {
            // For Python, we might build wheels or containers
            Command::new("python").args(["-m", "build"]).status()?
        }
        ProjectType::Go => {
            let mut cmd = Command::new("go");
            cmd.arg("build");

            if matches!(env, Environment::Production) {
                cmd.args(["-ldflags", "-s -w"]); // Strip debug info
            }

            cmd.status()?
        }
        ProjectType::Ruby => Command::new("bundle")
            .args(["exec", "rake", "build"])
            .status()?,
        ProjectType::Java => {
            if std::path::Path::new("pom.xml").exists() {
                Command::new("mvn").arg("package").status()?
            } else {
                Command::new("./gradlew").arg("build").status()?
            }
        }
        ProjectType::Php => Command::new("composer")
            .arg("install")
            .arg("--no-dev")
            .status()?,
        ProjectType::DotNet => {
            let mut cmd = Command::new("dotnet");
            cmd.arg("build");
            if matches!(env, Environment::Production) {
                cmd.arg("--configuration").arg("Release");
            }
            cmd.status()?
        }
        ProjectType::Elixir => Command::new("mix").arg("compile").status()?,
        ProjectType::Kotlin => Command::new("./gradlew").arg("build").status()?,
        ProjectType::Swift => {
            let mut cmd = Command::new("swift");
            cmd.arg("build");
            if matches!(env, Environment::Production) {
                cmd.arg("-c").arg("release");
            }
            cmd.status()?
        }
        ProjectType::Scala => Command::new("sbt").arg("compile").status()?,
        _ => {
            return Err(CldevError::command(format!(
                "Build not supported for project type: {:?}",
                project_type
            )))
        }
    };

    let duration = start_time.elapsed();
    let duration_secs = duration.as_secs_f64();

    Ok(BuildResult {
        success: status.success(),
        duration_secs,
        output_size: calculate_output_size(),
    })
}

/// Calculate output directory size
fn calculate_output_size() -> Option<u64> {
    let output_dirs = vec!["dist", "build", "target/release", ".next"];

    for dir in output_dirs {
        if std::path::Path::new(dir).exists() {
            if let Ok(size) = calculate_dir_size(dir) {
                return Some(size);
            }
        }
    }

    None
}

/// Calculate directory size recursively
fn calculate_dir_size(path: &str) -> Result<u64> {
    let mut total_size = 0u64;

    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;

        if metadata.is_dir() {
            total_size += calculate_dir_size(entry.path().to_str().unwrap())?;
        } else {
            total_size += metadata.len();
        }
    }

    Ok(total_size)
}

/// Analyze bundle size and composition
fn analyze_bundle() -> Result<()> {
    // For Node.js projects with webpack or vite
    if std::path::Path::new("package.json").exists() {
        println!("  {} Running bundle analyzer...", "→".cyan());

        // Try webpack-bundle-analyzer
        let webpack_status = Command::new("npx")
            .args(["webpack-bundle-analyzer", "--help"])
            .output();

        if webpack_status.is_ok() {
            let _status = Command::new("npm").args(["run", "analyze"]).status()?;
        } else {
            // Try source-map-explorer
            let _status = Command::new("npx")
                .args(["source-map-explorer", "dist/**/*.js"])
                .status()?;
        }
    }

    Ok(())
}

/// Display build statistics
fn display_build_stats(result: &BuildResult) {
    use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Table};

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec!["Metric", "Value"]);

    table.add_row(vec![
        "Build Duration",
        &format!("{:.2}s", result.duration_secs),
    ]);

    if let Some(size) = result.output_size {
        let size_mb = size as f64 / 1_048_576.0;
        table.add_row(vec!["Output Size", &format!("{:.2} MB", size_mb)]);
    }

    println!("\n{}", table);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_result_creation() {
        let result = BuildResult {
            success: true,
            duration_secs: 10.5,
            output_size: Some(1048576),
        };

        assert!(result.success);
        assert_eq!(result.duration_secs, 10.5);
        assert_eq!(result.output_size, Some(1048576));
    }
}
