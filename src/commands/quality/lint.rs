/// Linting command implementation
///
/// Automatically detects project type and runs the appropriate linter
use crate::cli::output::OutputHandler;
use crate::core::error::Result;
use crate::core::project_detector::ProjectDetector;
use std::path::Path;
use std::process::Command;

/// Run linter with project auto-detection
///
/// # Arguments
/// * `paths` - Specific files or patterns to lint (empty for all files)
/// * `fix` - Whether to auto-fix issues
/// * `output` - Output handler for user feedback
///
/// # Returns
/// Result indicating success or error
pub fn run_lint(paths: &[String], fix: bool, output: &OutputHandler) -> Result<()> {
    output.info("🔍 Detecting project type...");

    // Detect project type
    let detector = ProjectDetector::new(None)?;
    let project_type = detector.project_type();

    output.success(&format!("✅ Detected {} project", project_type.name()));

    // Get lint command based on project type
    let all = paths.is_empty();
    let command_parts = detector.get_lint_command(fix, all)?;

    if command_parts.is_empty() {
        return Err(crate::core::error::CldevError::Config(
            "No lint command generated".to_string(),
        ));
    }

    // Build command info message
    let cmd_str = command_parts.join(" ");
    output.info(&format!("🔧 Running: {}", cmd_str));

    // Execute lint command
    let mut cmd = Command::new(&command_parts[0]);
    cmd.current_dir(detector.root());

    // Add command arguments
    for arg in &command_parts[1..] {
        cmd.arg(arg);
    }

    // Add specific paths if provided
    if !paths.is_empty() {
        for path in paths {
            cmd.arg(path);
        }
    }

    output.debug(&format!("Executing command: {:?}", cmd));

    // Run the command
    let status = cmd.status().map_err(|e| {
        crate::core::error::CldevError::Config(format!(
            "Failed to execute lint command '{}': {}",
            command_parts[0], e
        ))
    })?;

    if status.success() {
        output.success("✅ Linting completed successfully");
        Ok(())
    } else {
        let exit_code = status.code().unwrap_or(-1);
        output.error(&format!("❌ Linting failed with exit code: {}", exit_code));
        Err(crate::core::error::CldevError::Config(format!(
            "Linting failed with exit code: {}",
            exit_code
        )))
    }
}

/// Run linter with advanced options
///
/// # Arguments
/// * `paths` - Specific files or patterns to lint
/// * `fix` - Whether to auto-fix issues
/// * `project_path` - Custom project path (None for current directory)
/// * `output` - Output handler for user feedback
///
/// # Returns
/// Result indicating success or error
pub fn run_lint_advanced(
    paths: &[String],
    fix: bool,
    project_path: Option<&Path>,
    output: &OutputHandler,
) -> Result<()> {
    output.info("🔍 Detecting project type...");

    // Detect project type
    let detector = ProjectDetector::new(project_path)?;
    let project_type = detector.project_type();

    output.success(&format!("✅ Detected {} project", project_type.name()));

    // Show project-specific tips
    match project_type {
        crate::core::project_detector::ProjectType::NodeJs => {
            if fix {
                output.info("💡 Tip: Auto-fix enabled. ESLint will fix issues automatically.");
            } else {
                output.info("💡 Tip: Use --fix flag to automatically fix issues.");
            }
        }
        crate::core::project_detector::ProjectType::Rust => {
            output.info("💡 Tip: Clippy is running with deny warnings.");
            if !paths.is_empty() {
                output
                    .warning("⚠️  Specific paths ignored for Rust projects. Checking all targets.");
            }
        }
        crate::core::project_detector::ProjectType::Go => {
            output.info(
                "💡 Tip: Using 'go vet' for linting. Consider 'golangci-lint' for more checks.",
            );
        }
        crate::core::project_detector::ProjectType::Python => {
            output
                .info("💡 Tip: Using Ruff/Pylint/Flake8. Configure in pyproject.toml or .flake8.");
        }
        crate::core::project_detector::ProjectType::Ruby
        | crate::core::project_detector::ProjectType::Java
        | crate::core::project_detector::ProjectType::Php
        | crate::core::project_detector::ProjectType::DotNet
        | crate::core::project_detector::ProjectType::Elixir
        | crate::core::project_detector::ProjectType::Kotlin
        | crate::core::project_detector::ProjectType::Swift
        | crate::core::project_detector::ProjectType::Scala => {
            // Language-specific linters will be auto-detected
        }
        crate::core::project_detector::ProjectType::Unknown => {
            // Error will be returned by get_lint_command
        }
    }

    // Get lint command based on project type
    let all = paths.is_empty();
    let command_parts = detector.get_lint_command(fix, all)?;

    if command_parts.is_empty() {
        return Err(crate::core::error::CldevError::Config(
            "No lint command generated".to_string(),
        ));
    }

    // Build command info message
    let cmd_str = command_parts.join(" ");
    output.info(&format!("🔧 Running: {}", cmd_str));

    // Execute lint command
    let mut cmd = Command::new(&command_parts[0]);
    cmd.current_dir(detector.root());

    // Add command arguments
    for arg in &command_parts[1..] {
        cmd.arg(arg);
    }

    // Add specific paths if provided and supported
    if !paths.is_empty() && project_type != crate::core::project_detector::ProjectType::Rust {
        for path in paths {
            cmd.arg(path);
        }
    }

    output.debug(&format!("Executing command: {:?}", cmd));

    // Run the command with output streaming
    let output_result = cmd.output().map_err(|e| {
        crate::core::error::CldevError::Config(format!(
            "Failed to execute lint command '{}': {}",
            command_parts[0], e
        ))
    })?;

    // Show stdout
    if !output_result.stdout.is_empty() {
        let stdout = String::from_utf8_lossy(&output_result.stdout);
        println!("{}", stdout);
    }

    // Show stderr
    if !output_result.stderr.is_empty() {
        let stderr = String::from_utf8_lossy(&output_result.stderr);
        eprintln!("{}", stderr);
    }

    if output_result.status.success() {
        output.success("✅ Linting completed successfully");
        Ok(())
    } else {
        let exit_code = output_result.status.code().unwrap_or(-1);
        output.error(&format!("❌ Linting failed with exit code: {}", exit_code));

        // Provide helpful error messages
        match project_type {
            crate::core::project_detector::ProjectType::NodeJs => {
                output.info("💡 Common fixes:");
                output.list_item("Run 'npm install' to ensure dependencies are installed");
                output.list_item("Check .eslintrc.* configuration files");
                if !fix {
                    output.list_item("Use --fix to automatically fix some issues");
                }
            }
            crate::core::project_detector::ProjectType::Rust => {
                output.info("💡 Common fixes:");
                output.list_item("Review Clippy suggestions above");
                output.list_item("Run 'cargo fix' to auto-fix some issues");
                output.list_item("Check clippy.toml for rule configurations");
            }
            crate::core::project_detector::ProjectType::Go => {
                output.info("💡 Common fixes:");
                output.list_item("Review 'go vet' output above");
                output.list_item("Run 'go fmt' to fix formatting issues");
            }
            crate::core::project_detector::ProjectType::Python => {
                output.info("💡 Common fixes:");
                output.list_item("Review linter output above");
                output.list_item("Check pyproject.toml or .flake8 configuration");
            }
            crate::core::project_detector::ProjectType::Ruby
            | crate::core::project_detector::ProjectType::Java
            | crate::core::project_detector::ProjectType::Php
            | crate::core::project_detector::ProjectType::DotNet
            | crate::core::project_detector::ProjectType::Elixir
            | crate::core::project_detector::ProjectType::Kotlin
            | crate::core::project_detector::ProjectType::Swift
            | crate::core::project_detector::ProjectType::Scala => {
                output.info("💡 Common fixes:");
                output.list_item("Review linter output above");
                output.list_item("Check your linter configuration files");
            }
            crate::core::project_detector::ProjectType::Unknown => {}
        }

        Err(crate::core::error::CldevError::Config(format!(
            "Linting failed with exit code: {}",
            exit_code
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_lint_unknown_project() {
        let temp_dir = TempDir::new().unwrap();
        let output = OutputHandler::default();

        let result = run_lint(&[], false, &output);
        assert!(result.is_err());
    }

    #[test]
    fn test_lint_rust_project() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(
            temp_dir.path().join("Cargo.toml"),
            "[package]\nname = \"test\"",
        )
        .unwrap();

        let output = OutputHandler::default();

        // This may fail if cargo is not installed, but structure should be correct
        let _result = run_lint_advanced(&[], false, Some(temp_dir.path()), &output);
        // We don't assert success because we don't know if cargo is installed
    }
}
