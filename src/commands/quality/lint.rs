#![allow(dead_code)]

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
    output.info(&output.t("quality-lint-detecting"));

    // Detect project type
    let detector = ProjectDetector::new(None)?;
    let project_type = detector.project_type();

    let mut vars = std::collections::HashMap::new();
    vars.insert("type", project_type.name());
    output.success(&output.t_with_vars("quality-lint-detected", &vars));

    // Get lint command based on project type
    let all = paths.is_empty();
    let command_parts = detector.get_lint_command(fix, all)?;

    if command_parts.is_empty() {
        return Err(crate::core::error::CldevError::Config(
            output.t("quality-lint-no-command"),
        ));
    }

    // Build command info message
    let cmd_str = command_parts.join(" ");
    let mut vars = std::collections::HashMap::new();
    vars.insert("command", cmd_str.as_str());
    output.info(&output.t_with_vars("quality-lint-running", &vars));

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
        output.success(&output.t("quality-lint-success"));
        Ok(())
    } else {
        let exit_code = status.code().unwrap_or(-1);
        let exit_code_str = exit_code.to_string();
        let mut vars = std::collections::HashMap::new();
        vars.insert("code", exit_code_str.as_str());
        output.error(&output.t_with_vars("quality-lint-failed", &vars));
        Err(crate::core::error::CldevError::Config(
            output.t_with_vars("quality-lint-failed", &vars),
        ))
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
    output.info(&output.t("quality-lint-detecting"));

    // Detect project type
    let detector = ProjectDetector::new(project_path)?;
    let project_type = detector.project_type();

    let mut vars = std::collections::HashMap::new();
    vars.insert("type", project_type.name());
    output.success(&output.t_with_vars("quality-lint-detected", &vars));

    // Show project-specific tips
    match project_type {
        crate::core::project_detector::ProjectType::NodeJs => {
            if fix {
                output.info(&output.t("quality-lint-tip-nodejs-fix"));
            } else {
                output.info(&output.t("quality-lint-tip-nodejs-no-fix"));
            }
        }
        crate::core::project_detector::ProjectType::Rust => {
            output.info(&output.t("quality-lint-tip-rust"));
            if !paths.is_empty() {
                output.warning(&output.t("quality-lint-warn-rust-paths"));
            }
        }
        crate::core::project_detector::ProjectType::Go => {
            output.info(&output.t("quality-lint-tip-go"));
        }
        crate::core::project_detector::ProjectType::Python => {
            output.info(&output.t("quality-lint-tip-python"));
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
            output.t("quality-lint-no-command"),
        ));
    }

    // Build command info message
    let cmd_str = command_parts.join(" ");
    let mut vars = std::collections::HashMap::new();
    vars.insert("command", cmd_str.as_str());
    output.info(&output.t_with_vars("quality-lint-running", &vars));

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
        output.success(&output.t("quality-lint-success"));
        Ok(())
    } else {
        let exit_code = output_result.status.code().unwrap_or(-1);
        let exit_code_str = exit_code.to_string();
        let mut vars = std::collections::HashMap::new();
        vars.insert("code", exit_code_str.as_str());
        output.error(&output.t_with_vars("quality-lint-failed", &vars));

        // Provide helpful error messages
        output.info(&output.t("quality-lint-common-fixes"));
        match project_type {
            crate::core::project_detector::ProjectType::NodeJs => {
                output.list_item(&output.t("quality-lint-fix-nodejs-install"));
                output.list_item(&output.t("quality-lint-fix-nodejs-config"));
                if !fix {
                    output.list_item(&output.t("quality-lint-fix-nodejs-use-fix"));
                }
            }
            crate::core::project_detector::ProjectType::Rust => {
                output.list_item(&output.t("quality-lint-fix-rust-review"));
                output.list_item(&output.t("quality-lint-fix-rust-cargo-fix"));
                output.list_item(&output.t("quality-lint-fix-rust-config"));
            }
            crate::core::project_detector::ProjectType::Go => {
                output.list_item(&output.t("quality-lint-fix-go-review"));
                output.list_item(&output.t("quality-lint-fix-go-fmt"));
            }
            crate::core::project_detector::ProjectType::Python => {
                output.list_item(&output.t("quality-lint-fix-python-review"));
                output.list_item(&output.t("quality-lint-fix-python-config"));
            }
            crate::core::project_detector::ProjectType::Ruby
            | crate::core::project_detector::ProjectType::Java
            | crate::core::project_detector::ProjectType::Php
            | crate::core::project_detector::ProjectType::DotNet
            | crate::core::project_detector::ProjectType::Elixir
            | crate::core::project_detector::ProjectType::Kotlin
            | crate::core::project_detector::ProjectType::Swift
            | crate::core::project_detector::ProjectType::Scala => {
                output.list_item(&output.t("quality-lint-fix-generic-review"));
                output.list_item(&output.t("quality-lint-fix-generic-config"));
            }
            crate::core::project_detector::ProjectType::Unknown => {}
        }

        Err(crate::core::error::CldevError::Config(
            output.t_with_vars("quality-lint-failed", &vars),
        ))
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

        // Change to temp directory to avoid detecting current project
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let result = run_lint(&[], false, &output);
        assert!(result.is_err());

        // Restore original directory
        std::env::set_current_dir(original_dir).unwrap();
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
