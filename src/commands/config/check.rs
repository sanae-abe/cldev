//! Configuration validation and health check
//!
//! This module provides comprehensive configuration file validation including:
//! - TOML syntax validation
//! - Version compatibility check
//! - Required fields verification
//! - Path existence validation
//! - Security checks (file permissions)

use crate::cli::output::OutputHandler;
use crate::core::config::{validate_version, Config, CONFIG_VERSION};
use crate::core::error::{CldevError, Result};
use colored::Colorize;
use std::path::PathBuf;

/// Validation result for individual checks
#[derive(Debug)]
pub struct ValidationResult {
    pub category: String,
    pub passed: bool,
    pub message: String,
}

impl ValidationResult {
    fn success(category: &str, message: &str) -> Self {
        Self {
            category: category.to_string(),
            passed: true,
            message: message.to_string(),
        }
    }

    fn failure(category: &str, message: &str) -> Self {
        Self {
            category: category.to_string(),
            passed: false,
            message: message.to_string(),
        }
    }
}

/// Check and validate configuration file
///
/// # Arguments
///
/// * `path` - Optional path to config file (uses default if None)
/// * `detailed` - Show detailed validation information
/// * `fix` - Attempt to fix issues automatically
/// * `output` - Output handler for formatted messages
///
/// # Returns
///
/// Returns Ok(()) if all validations pass, Err otherwise
pub fn check_config(
    path: Option<PathBuf>,
    detailed: bool,
    fix: bool,
    output: &OutputHandler,
) -> Result<()> {
    output.info(&output.i18n().get("config-check-header"));

    let config_path = path.unwrap_or_else(|| {
        Config::default_path().unwrap_or_else(|_| PathBuf::from("~/.config/cldev/config.toml"))
    });

    let mut results = Vec::new();

    // Check 1: File existence
    if !config_path.exists() {
        results.push(ValidationResult::failure(
            &output.i18n().get("config-check-category-file"),
            &output.i18n().format(
                "config-check-file-not-found",
                "path",
                &config_path.display().to_string(),
            ),
        ));

        if fix {
            output.info(&output.i18n().get("config-check-auto-fix"));
            let default_config = Config::default();
            default_config.save(Some(config_path.clone()))?;
            results.push(ValidationResult::success(
                &output.i18n().get("config-check-category-auto-fix"),
                &output.i18n().format(
                    "config-check-created-default",
                    "path",
                    &config_path.display().to_string(),
                ),
            ));
        } else {
            print_results(&results, detailed, output);
            return Err(CldevError::config(
                output.i18n().get("config-check-error-not-found"),
            ));
        }
    }

    // Check 2: TOML parsing
    let config = match Config::load(Some(config_path.clone())) {
        Ok(cfg) => {
            results.push(ValidationResult::success(
                &output.i18n().get("config-check-category-toml"),
                &output.i18n().get("config-check-toml-valid"),
            ));
            cfg
        }
        Err(e) => {
            results.push(ValidationResult::failure(
                &output.i18n().get("config-check-category-toml"),
                &output
                    .i18n()
                    .format("config-check-toml-error", "error", &e.to_string()),
            ));
            print_results(&results, detailed, output);
            return Err(e);
        }
    };

    // Check 3: Version validation
    match validate_version(&config.version) {
        Ok(_) => {
            results.push(ValidationResult::success(
                &output.i18n().get("config-check-category-version"),
                &output
                    .i18n()
                    .format(
                        "config-check-version-compatible",
                        "version",
                        &config.version,
                    )
                    .replace("{current}", CONFIG_VERSION),
            ));
        }
        Err(e) => {
            results.push(ValidationResult::failure(
                &output.i18n().get("config-check-category-version"),
                &e.to_string(),
            ));
        }
    }

    // Check 4: Required fields validation
    validate_required_fields(&config, &mut results);

    // Check 5: Path existence validation
    validate_paths(&config, &mut results);

    // Check 6: File permissions (Unix only)
    #[cfg(unix)]
    validate_permissions(&config_path, &mut results)?;

    // Check 7: Git CLI availability
    validate_git_cli(&config, &mut results);

    // Print results
    print_results(&results, detailed, output);

    // Determine overall result
    let all_passed = results.iter().all(|r| r.passed);
    if all_passed {
        output.success(&output.i18n().get("config-check-all-passed"));
        Ok(())
    } else {
        let failed_count = results.iter().filter(|r| !r.passed).count();
        output.error(&output.i18n().format(
            "config-check-failed-count",
            "count",
            &failed_count.to_string(),
        ));
        Err(CldevError::validation(
            output.i18n().get("config-check-validation-failed"),
        ))
    }
}

/// Validate required fields in configuration
fn validate_required_fields(config: &Config, results: &mut Vec<ValidationResult>) {
    use crate::core::i18n::I18n;
    let i18n = I18n::new();

    // Check version field
    if config.version.is_empty() {
        results.push(ValidationResult::failure(
            &i18n.get("config-check-category-required"),
            &i18n.get("config-check-version-empty"),
        ));
    } else {
        results.push(ValidationResult::success(
            &i18n.get("config-check-category-required"),
            &i18n.get("config-check-required-ok"),
        ));
    }
}

/// Validate paths in configuration
fn validate_paths(config: &Config, results: &mut Vec<ValidationResult>) {
    // Validate claude_dir
    if !config.general.claude_dir.exists() {
        results.push(ValidationResult::failure(
            "Path Validation",
            &format!(
                "Claude directory does not exist: {}",
                config.general.claude_dir.display()
            ),
        ));
    } else {
        results.push(ValidationResult::success(
            "Path Validation",
            &format!(
                "Claude directory exists: {}",
                config.general.claude_dir.display()
            ),
        ));
    }

    // Validate projects_dir
    if !config.general.projects_dir.exists() {
        results.push(ValidationResult::failure(
            "Path Validation",
            &format!(
                "Projects directory does not exist: {}",
                config.general.projects_dir.display()
            ),
        ));
    }

    // Validate learning sessions directory
    if !config.lr.sessions_dir.exists() {
        results.push(ValidationResult::failure(
            "Path Validation",
            &format!(
                "Learning sessions directory does not exist: {}",
                config.lr.sessions_dir.display()
            ),
        ));
    }
}

/// Validate file permissions (Unix only)
#[cfg(unix)]
fn validate_permissions(path: &PathBuf, results: &mut Vec<ValidationResult>) -> Result<()> {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    let metadata = fs::metadata(path)?;
    let permissions = metadata.permissions();
    let mode = permissions.mode();

    // Check if permissions are 600 (owner read/write only)
    if mode & 0o177 != 0 {
        results.push(ValidationResult::failure(
            "Security",
            &format!(
                "Insecure file permissions: {:o}. Expected 600 (owner read/write only)",
                mode & 0o777
            ),
        ));
    } else {
        results.push(ValidationResult::success(
            "Security",
            "File permissions are secure (600)",
        ));
    }

    Ok(())
}

/// Validate Git CLI tools availability
fn validate_git_cli(config: &Config, results: &mut Vec<ValidationResult>) {
    use std::process::Command;

    // Check GitHub CLI (gh)
    if config.git.github_cli {
        match Command::new("gh").arg("--version").output() {
            Ok(output) if output.status.success() => {
                results.push(ValidationResult::success(
                    "Git CLI",
                    "GitHub CLI (gh) is available",
                ));
            }
            _ => {
                results.push(ValidationResult::failure(
                    "Git CLI",
                    "GitHub CLI (gh) is enabled in config but not found in PATH",
                ));
            }
        }
    }

    // Check GitLab CLI (glab)
    if config.git.gitlab_cli {
        match Command::new("glab").arg("--version").output() {
            Ok(output) if output.status.success() => {
                results.push(ValidationResult::success(
                    "Git CLI",
                    "GitLab CLI (glab) is available",
                ));
            }
            _ => {
                results.push(ValidationResult::failure(
                    "Git CLI",
                    "GitLab CLI (glab) is enabled in config but not found in PATH",
                ));
            }
        }
    }
}

/// Print validation results with formatted output
fn print_results(results: &[ValidationResult], detailed: bool, output: &OutputHandler) {
    if detailed {
        output.info(&format!(
            "\n{}",
            output.i18n().get("config-check-detailed-results")
        ));
        output.raw(&"=".repeat(60));

        for result in results {
            let status = if result.passed {
                "✓".green().bold()
            } else {
                "✗".red().bold()
            };

            let category = format!("[{}]", result.category).bold();
            let message = if result.passed {
                result.message.green()
            } else {
                result.message.red()
            };

            output.raw(&format!("{} {} {}", status, category, message));
        }

        output.raw(&"=".repeat(60));
    }

    // Summary
    let passed = results.iter().filter(|r| r.passed).count();
    let total = results.len();

    let summary = if passed == total {
        format!("✅ {}/{} checks passed", passed, total).green()
    } else {
        format!("⚠️  {}/{} checks passed", passed, total).yellow()
    };

    output.raw(&format!("\n{}", summary));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_result_creation() {
        let success = ValidationResult::success("Test", "All good");
        assert!(success.passed);
        assert_eq!(success.category, "Test");

        let failure = ValidationResult::failure("Test", "Something wrong");
        assert!(!failure.passed);
        assert_eq!(failure.category, "Test");
    }
}
