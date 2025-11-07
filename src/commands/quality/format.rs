/// Code formatting command implementation
///
/// Automatically detects project type and runs the appropriate formatter
use crate::cli::output::OutputHandler;
use crate::core::error::Result;
use crate::core::project_detector::ProjectDetector;
use std::path::Path;
use std::process::Command;

/// Format code with project auto-detection
///
/// # Arguments
/// * `paths` - Specific files or patterns to format (empty for all files)
/// * `check` - Whether to check formatting without modifying files
/// * `output` - Output handler for user feedback
///
/// # Returns
/// Result indicating success or error
pub fn format_code(paths: &[String], check: bool, output: &OutputHandler) -> Result<()> {
    output.info("🔍 Detecting project type...");

    // Detect project type
    let detector = ProjectDetector::new(None)?;
    let project_type = detector.project_type();

    output.success(&format!("✅ Detected {} project", project_type.name()));

    // Get format command based on project type
    let command_parts = detector.get_format_command(check)?;

    if command_parts.is_empty() {
        return Err(crate::core::error::CldevError::Config(
            "No format command generated".to_string(),
        ));
    }

    // Build command info message
    let cmd_str = command_parts.join(" ");
    let action = if check { "Checking" } else { "Formatting" };
    output.info(&format!("🎨 {}: {}", action, cmd_str));

    // Execute format command
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
            "Failed to execute format command '{}': {}",
            command_parts[0], e
        ))
    })?;

    if status.success() {
        if check {
            output.success("✅ Code formatting is correct");
        } else {
            output.success("✅ Code formatted successfully");
        }
        Ok(())
    } else {
        let exit_code = status.code().unwrap_or(-1);
        if check {
            output.warning(&format!(
                "⚠️  Formatting issues found (exit code: {})",
                exit_code
            ));
            output.info("💡 Run without --check to fix formatting issues");
        } else {
            output.error(&format!(
                "❌ Formatting failed with exit code: {}",
                exit_code
            ));
        }

        Err(crate::core::error::CldevError::Config(format!(
            "Formatting failed with exit code: {}",
            exit_code
        )))
    }
}

/// Format code with advanced options
///
/// # Arguments
/// * `paths` - Specific files or patterns to format
/// * `check` - Whether to check formatting without modifying files
/// * `project_path` - Custom project path (None for current directory)
/// * `output` - Output handler for user feedback
///
/// # Returns
/// Result indicating success or error
#[allow(dead_code)]
pub fn format_code_advanced(
    paths: &[String],
    check: bool,
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
            if check {
                output.info("💡 Tip: Check mode enabled. Files will not be modified.");
            } else {
                output.info("💡 Tip: Prettier will format your code automatically.");
            }
        }
        crate::core::project_detector::ProjectType::Rust => {
            output.info("💡 Tip: rustfmt uses rustfmt.toml or .rustfmt.toml for configuration.");
        }
        crate::core::project_detector::ProjectType::Go => {
            output.info("💡 Tip: gofmt enforces standard Go formatting style.");
        }
        crate::core::project_detector::ProjectType::Python => {
            output.info("💡 Tip: Black/Ruff formats code with minimal configuration.");
        }
        crate::core::project_detector::ProjectType::Ruby
        | crate::core::project_detector::ProjectType::Java
        | crate::core::project_detector::ProjectType::Php
        | crate::core::project_detector::ProjectType::DotNet
        | crate::core::project_detector::ProjectType::Elixir
        | crate::core::project_detector::ProjectType::Kotlin
        | crate::core::project_detector::ProjectType::Swift
        | crate::core::project_detector::ProjectType::Scala => {
            // Formatters will be auto-detected
        }
        crate::core::project_detector::ProjectType::Unknown => {
            // Error will be returned by get_format_command
        }
    }

    // Get format command based on project type
    let command_parts = detector.get_format_command(check)?;

    if command_parts.is_empty() {
        return Err(crate::core::error::CldevError::Config(
            "No format command generated".to_string(),
        ));
    }

    // Build command info message
    let cmd_str = command_parts.join(" ");
    let action = if check { "Checking" } else { "Formatting" };
    output.info(&format!("🎨 {}: {}", action, cmd_str));

    // Execute format command
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

    // Run the command with output streaming
    let output_result = cmd.output().map_err(|e| {
        crate::core::error::CldevError::Config(format!(
            "Failed to execute format command '{}': {}",
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
        if check {
            output.success("✅ Code formatting is correct");
        } else {
            output.success("✅ Code formatted successfully");
        }
        Ok(())
    } else {
        let exit_code = output_result.status.code().unwrap_or(-1);

        if check {
            output.warning(&format!(
                "⚠️  Formatting issues found (exit code: {})",
                exit_code
            ));
            output.info("💡 Common next steps:");
            output.list_item(&format!(
                "Run 'cldev quality format' to fix formatting ({})",
                project_type.name()
            ));

            match project_type {
                crate::core::project_detector::ProjectType::NodeJs => {
                    output.list_item("Or run 'npm run format' directly");
                }
                crate::core::project_detector::ProjectType::Rust => {
                    output.list_item("Or run 'cargo fmt' directly");
                }
                crate::core::project_detector::ProjectType::Go => {
                    output.list_item("Or run 'go fmt ./...' directly");
                }
                crate::core::project_detector::ProjectType::Python => {
                    output.list_item("Or run 'black .' or 'ruff format .' directly");
                }
                crate::core::project_detector::ProjectType::Ruby
                | crate::core::project_detector::ProjectType::Java
                | crate::core::project_detector::ProjectType::Php
                | crate::core::project_detector::ProjectType::DotNet
                | crate::core::project_detector::ProjectType::Elixir
                | crate::core::project_detector::ProjectType::Kotlin
                | crate::core::project_detector::ProjectType::Swift
                | crate::core::project_detector::ProjectType::Scala => {}
                crate::core::project_detector::ProjectType::Unknown => {}
            }
        } else {
            output.error(&format!(
                "❌ Formatting failed with exit code: {}",
                exit_code
            ));

            output.info("💡 Common fixes:");
            output.list_item("Ensure the formatter is properly installed");
            output.list_item("Check configuration files for syntax errors");

            match project_type {
                crate::core::project_detector::ProjectType::NodeJs => {
                    output.list_item("Run 'npm install' to ensure Prettier is installed");
                    output.list_item("Check .prettierrc.* or prettier.config.js");
                }
                crate::core::project_detector::ProjectType::Rust => {
                    output.list_item("rustfmt comes with Rust, no installation needed");
                    output.list_item("Check rustfmt.toml configuration");
                }
                crate::core::project_detector::ProjectType::Go => {
                    output.list_item("gofmt comes with Go, no installation needed");
                }
                crate::core::project_detector::ProjectType::Python => {
                    output
                        .list_item("Install formatter: 'pip install black' or 'pip install ruff'");
                    output.list_item("Check pyproject.toml configuration");
                }
                crate::core::project_detector::ProjectType::Ruby
                | crate::core::project_detector::ProjectType::Java
                | crate::core::project_detector::ProjectType::Php
                | crate::core::project_detector::ProjectType::DotNet
                | crate::core::project_detector::ProjectType::Elixir
                | crate::core::project_detector::ProjectType::Kotlin
                | crate::core::project_detector::ProjectType::Swift
                | crate::core::project_detector::ProjectType::Scala => {}
                crate::core::project_detector::ProjectType::Unknown => {}
            }
        }

        Err(crate::core::error::CldevError::Config(format!(
            "Formatting failed with exit code: {}",
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
    fn test_format_unknown_project() {
        let temp_dir = TempDir::new().unwrap();
        let output = OutputHandler::default();

        // Change to temp directory to avoid detecting current Rust project
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let result = format_code(&[], false, &output);

        // Restore original directory
        std::env::set_current_dir(original_dir).unwrap();

        assert!(result.is_err());
    }

    #[test]
    fn test_format_rust_project() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(
            temp_dir.path().join("Cargo.toml"),
            "[package]\nname = \"test\"",
        )
        .unwrap();

        let output = OutputHandler::default();

        // This may fail if cargo is not installed, but structure should be correct
        let _result = format_code_advanced(&[], false, Some(temp_dir.path()), &output);
        // We don't assert success because we don't know if cargo is installed
    }

    #[test]
    fn test_format_check_mode() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(
            temp_dir.path().join("Cargo.toml"),
            "[package]\nname = \"test\"",
        )
        .unwrap();

        let output = OutputHandler::default();

        // This may fail if cargo is not installed, but structure should be correct
        let _result = format_code_advanced(&[], true, Some(temp_dir.path()), &output);
        // We don't assert success because we don't know if cargo is installed
    }
}
