/// Testing command implementation
///
/// Automatically detects project type and runs the appropriate test framework
use crate::cli::output::OutputHandler;
use crate::core::error::Result;
use crate::core::project_detector::ProjectDetector;
use std::path::Path;
use std::process::Command;

/// Run tests with project auto-detection
///
/// # Arguments
/// * `pattern` - Test pattern to filter tests
/// * `coverage` - Whether to generate coverage report
/// * `watch` - Whether to run in watch mode
/// * `output` - Output handler for user feedback
///
/// # Returns
/// Result indicating success or error
pub fn run_tests(
    pattern: Option<&str>,
    coverage: bool,
    watch: bool,
    output: &OutputHandler,
) -> Result<()> {
    output.info(&output.t("quality-test-detecting"));

    // Detect project type
    let detector = ProjectDetector::new(None)?;
    let project_type = detector.project_type();

    output.success(&output.t_format("quality-test-detected", "type", project_type.name()));

    // Get test command based on project type
    let command_parts = detector.get_test_command(pattern, coverage, watch)?;

    if command_parts.is_empty() {
        return Err(crate::core::error::CldevError::Config(
            "No test command generated".to_string(),
        ));
    }

    // Build command info message
    let cmd_str = command_parts.join(" ");
    output.info(&output.t_format("quality-test-running", "command", &cmd_str));

    // Execute test command
    let mut cmd = Command::new(&command_parts[0]);
    cmd.current_dir(detector.root());

    // Add command arguments
    for arg in &command_parts[1..] {
        cmd.arg(arg);
    }

    output.debug(&format!("Executing command: {:?}", cmd));

    // Run the command
    let status = cmd.status().map_err(|e| {
        crate::core::error::CldevError::Config(format!(
            "Failed to execute test command '{}': {}",
            command_parts[0], e
        ))
    })?;

    if status.success() {
        output.success(&output.t("quality-test-success"));

        if coverage {
            output.info(&output.t("quality-test-coverage-generated"));
            match project_type {
                crate::core::project_detector::ProjectType::NodeJs => {
                    output.list_item("Check coverage/ directory for detailed report");
                }
                crate::core::project_detector::ProjectType::Rust => {
                    output.list_item("Check tarpaulin-report.html or coverage/ directory");
                }
                crate::core::project_detector::ProjectType::Go => {
                    output.list_item("Check coverage.out file");
                    output.list_item("View with: go tool cover -html=coverage.out");
                }
                crate::core::project_detector::ProjectType::Python => {
                    output.list_item("Check htmlcov/ directory for detailed report");
                }
                crate::core::project_detector::ProjectType::Ruby
                | crate::core::project_detector::ProjectType::Java
                | crate::core::project_detector::ProjectType::Php
                | crate::core::project_detector::ProjectType::DotNet
                | crate::core::project_detector::ProjectType::Elixir
                | crate::core::project_detector::ProjectType::Kotlin
                | crate::core::project_detector::ProjectType::Swift
                | crate::core::project_detector::ProjectType::Scala => {
                    output.list_item("Check your project's coverage output directory");
                }
                crate::core::project_detector::ProjectType::Unknown => {}
            }
        }

        Ok(())
    } else {
        let exit_code = status.code().unwrap_or(-1);
        output.error(&output.t_format("quality-test-failed", "code", &exit_code.to_string()));

        Err(crate::core::error::CldevError::Config(format!(
            "Tests failed with exit code: {}",
            exit_code
        )))
    }
}

/// Run tests with advanced options
///
/// # Arguments
/// * `pattern` - Test pattern to filter tests
/// * `coverage` - Whether to generate coverage report
/// * `watch` - Whether to run in watch mode
/// * `project_path` - Custom project path (None for current directory)
/// * `output` - Output handler for user feedback
///
/// # Returns
/// Result indicating success or error
#[allow(dead_code)]
pub fn run_tests_advanced(
    pattern: Option<&str>,
    coverage: bool,
    watch: bool,
    project_path: Option<&Path>,
    output: &OutputHandler,
) -> Result<()> {
    output.info(&output.t("quality-test-detecting"));

    // Detect project type
    let detector = ProjectDetector::new(project_path)?;
    let project_type = detector.project_type();

    output.success(&output.t_format("quality-test-detected", "type", project_type.name()));

    // Show project-specific tips
    match project_type {
        crate::core::project_detector::ProjectType::NodeJs => {
            if watch {
                output.info("💡 Tip: Watch mode enabled. Tests will re-run on file changes.");
            }
            if coverage {
                output.info("💡 Tip: Coverage report will be generated in coverage/ directory.");
            }
            if let Some(p) = pattern {
                output.info(&format!("💡 Tip: Running tests matching pattern: {}", p));
            }
        }
        crate::core::project_detector::ProjectType::Rust => {
            if coverage {
                output.info("💡 Tip: cargo-tarpaulin will generate coverage report.");
                output.warning(
                    "⚠️  Make sure cargo-tarpaulin is installed: cargo install cargo-tarpaulin",
                );
            }
            if watch {
                output.warning("⚠️  Watch mode not natively supported. Consider cargo-watch.");
            }
        }
        crate::core::project_detector::ProjectType::Go => {
            if coverage {
                output.info("💡 Tip: Coverage data saved to coverage.out.");
            }
        }
        crate::core::project_detector::ProjectType::Python => {
            if coverage {
                output.info("💡 Tip: pytest-cov will generate HTML coverage report.");
            }
        }
        crate::core::project_detector::ProjectType::Ruby => {
            if coverage {
                output.info("💡 Tip: SimpleCov will generate coverage report (configure in spec_helper.rb).");
            }
        }
        crate::core::project_detector::ProjectType::Java => {
            output.info("💡 Tip: Using Maven or Gradle test runner.");
        }
        crate::core::project_detector::ProjectType::Php => {
            if coverage {
                output.info(
                    "💡 Tip: PHPUnit coverage report will be generated in coverage/ directory.",
                );
            }
        }
        crate::core::project_detector::ProjectType::DotNet => {
            if coverage {
                output.info("💡 Tip: Code coverage data will be collected.");
            }
        }
        crate::core::project_detector::ProjectType::Elixir => {
            if coverage {
                output.info("💡 Tip: ExCoveralls will generate coverage report.");
            }
        }
        crate::core::project_detector::ProjectType::Kotlin => {
            output.info("💡 Tip: Using Gradle test runner.");
        }
        crate::core::project_detector::ProjectType::Swift => {
            if coverage {
                output.info("💡 Tip: Code coverage requires Xcode 13.3+ or Swift 5.6+.");
            }
        }
        crate::core::project_detector::ProjectType::Scala => {
            if coverage {
                output.info("💡 Tip: sbt-scoverage will generate coverage report.");
            }
        }
        crate::core::project_detector::ProjectType::Unknown => {
            // Error will be returned by get_test_command
        }
    }

    // Get test command based on project type
    let command_parts = detector.get_test_command(pattern, coverage, watch)?;

    if command_parts.is_empty() {
        return Err(crate::core::error::CldevError::Config(
            "No test command generated".to_string(),
        ));
    }

    // Build command info message
    let cmd_str = command_parts.join(" ");
    output.info(&output.t_format("quality-test-running", "command", &cmd_str));

    // Show additional context
    if let Some(p) = pattern {
        output.info(&format!("🔎 Test filter: {}", p));
    }
    if coverage {
        output.info("📊 Coverage: enabled");
    }
    if watch {
        output.info("👀 Watch mode: enabled");
    }

    // Execute test command
    let mut cmd = Command::new(&command_parts[0]);
    cmd.current_dir(detector.root());

    // Add command arguments
    for arg in &command_parts[1..] {
        cmd.arg(arg);
    }

    output.debug(&format!("Executing command: {:?}", cmd));

    // Run the command with output streaming
    let output_result = cmd.output().map_err(|e| {
        crate::core::error::CldevError::Config(format!(
            "Failed to execute test command '{}': {}",
            command_parts[0], e
        ))
    })?;

    // Show stdout (respects quiet mode)
    if !output_result.stdout.is_empty() {
        let stdout = String::from_utf8_lossy(&output_result.stdout);
        output.println_raw(&stdout);
    }

    // Show stderr (always shown for errors, respects quiet for non-errors)
    if !output_result.stderr.is_empty() {
        let stderr = String::from_utf8_lossy(&output_result.stderr);
        if !output_result.status.success() {
            // Errors always shown
            eprintln!("{}", stderr);
        } else {
            // Non-error output respects quiet mode
            output.eprintln_raw(&stderr);
        }
    }

    if output_result.status.success() {
        output.success(&output.t("quality-test-success"));

        if coverage {
            output.info(&output.t("quality-test-coverage-generated"));
            match project_type {
                crate::core::project_detector::ProjectType::NodeJs => {
                    output.list_item("Open coverage/index.html in your browser");
                    output
                        .list_item("Check coverage thresholds in package.json or vitest.config.ts");
                }
                crate::core::project_detector::ProjectType::Rust => {
                    output.list_item("Open tarpaulin-report.html in your browser");
                    output.list_item("Check coverage with: open tarpaulin-report.html");
                }
                crate::core::project_detector::ProjectType::Go => {
                    output.list_item("View coverage: go tool cover -html=coverage.out");
                    output.list_item("Or open coverage.html if generated");
                }
                crate::core::project_detector::ProjectType::Python => {
                    output.list_item("Open htmlcov/index.html in your browser");
                    output.list_item("Check coverage thresholds in pyproject.toml");
                }
                crate::core::project_detector::ProjectType::Ruby => {
                    output.list_item("Open coverage/index.html in your browser");
                }
                crate::core::project_detector::ProjectType::Java => {
                    output.list_item("Check target/site/jacoco/index.html (Maven) or build/reports/jacoco/test/html/index.html (Gradle)");
                }
                crate::core::project_detector::ProjectType::Php => {
                    output.list_item("Open coverage/index.html in your browser");
                }
                crate::core::project_detector::ProjectType::DotNet => {
                    output.list_item(
                        "Coverage data collected. Use dotnet-coverage or Visual Studio to view.",
                    );
                }
                crate::core::project_detector::ProjectType::Elixir => {
                    output.list_item("ExCoveralls report generated");
                }
                crate::core::project_detector::ProjectType::Kotlin => {
                    output.list_item("Check build/reports/jacoco/test/html/index.html");
                }
                crate::core::project_detector::ProjectType::Swift => {
                    output.list_item("Use `xcov` or Xcode to view coverage");
                }
                crate::core::project_detector::ProjectType::Scala => {
                    output.list_item("Open target/scala-*/scoverage-report/index.html");
                }
                crate::core::project_detector::ProjectType::Unknown => {}
            }
        }

        Ok(())
    } else {
        let exit_code = output_result.status.code().unwrap_or(-1);
        output.error(&format!("❌ Tests failed with exit code: {}", exit_code));

        // Provide helpful error messages
        output.info("💡 Common next steps:");

        match project_type {
            crate::core::project_detector::ProjectType::NodeJs => {
                output.list_item("Review test output above for specific failures");
                output.list_item("Run 'npm run test' directly to see full output");
                output.list_item("Check test configuration in vitest.config.ts or jest.config.js");
                if !watch {
                    output.list_item("Use --watch to run tests in watch mode");
                }
            }
            crate::core::project_detector::ProjectType::Rust => {
                output.list_item("Review test failures above");
                output.list_item("Run 'cargo test' directly for detailed output");
                output.list_item("Use 'cargo test --lib' to test only library code");
                output.list_item(
                    "Use 'cargo test --test <test_name>' for specific integration tests",
                );
            }
            crate::core::project_detector::ProjectType::Go => {
                output.list_item("Review test failures above");
                output.list_item("Run 'go test -v ./...' for verbose output");
                output.list_item("Use 'go test -run <pattern>' to run specific tests");
            }
            crate::core::project_detector::ProjectType::Python => {
                output.list_item("Review test failures above");
                output.list_item("Run 'pytest -v' for verbose output");
                output.list_item("Use 'pytest -k <pattern>' to run specific tests");
                output.list_item("Check pytest.ini or pyproject.toml for configuration");
            }
            crate::core::project_detector::ProjectType::Ruby => {
                output.list_item("Review test failures above");
                output.list_item("Run 'bundle exec rspec' for detailed output");
            }
            crate::core::project_detector::ProjectType::Java => {
                output.list_item("Review test failures above");
                output.list_item("Run 'mvn test' or './gradlew test' for detailed output");
            }
            crate::core::project_detector::ProjectType::Php => {
                output.list_item("Review test failures above");
                output.list_item("Run 'vendor/bin/phpunit --verbose' for detailed output");
            }
            crate::core::project_detector::ProjectType::DotNet => {
                output.list_item("Review test failures above");
                output.list_item("Run 'dotnet test --logger:detailed' for verbose output");
            }
            crate::core::project_detector::ProjectType::Elixir => {
                output.list_item("Review test failures above");
                output.list_item("Run 'mix test --trace' for detailed output");
            }
            crate::core::project_detector::ProjectType::Kotlin => {
                output.list_item("Review test failures above");
                output.list_item("Run './gradlew test' for detailed output");
            }
            crate::core::project_detector::ProjectType::Swift => {
                output.list_item("Review test failures above");
                output.list_item("Run 'swift test' for detailed output");
            }
            crate::core::project_detector::ProjectType::Scala => {
                output.list_item("Review test failures above");
                output.list_item("Run 'sbt test' for detailed output");
            }
            crate::core::project_detector::ProjectType::Unknown => {}
        }

        Err(crate::core::error::CldevError::Config(format!(
            "Tests failed with exit code: {}",
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
    fn test_test_unknown_project() {
        let temp_dir = TempDir::new().unwrap();
        let output = OutputHandler::default();

        // Change to temp directory to avoid detecting current project
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let result = run_tests(None, false, false, &output);
        assert!(result.is_err());

        // Restore original directory
        std::env::set_current_dir(original_dir).unwrap();
    }

    #[test]
    fn test_test_rust_project() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(
            temp_dir.path().join("Cargo.toml"),
            "[package]\nname = \"test\"",
        )
        .unwrap();

        let output = OutputHandler::default();

        // This may fail if cargo is not installed, but structure should be correct
        let _result = run_tests_advanced(None, false, false, Some(temp_dir.path()), &output);
        // We don't assert success because we don't know if cargo is installed
    }

    #[test]
    fn test_test_with_coverage() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(
            temp_dir.path().join("Cargo.toml"),
            "[package]\nname = \"test\"",
        )
        .unwrap();

        let output = OutputHandler::default();

        // This will fail if cargo-tarpaulin is not installed, but that's expected
        let _result = run_tests_advanced(None, true, false, Some(temp_dir.path()), &output);
    }
}
