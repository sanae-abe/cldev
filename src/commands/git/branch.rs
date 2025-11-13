//! Conventional Branch naming implementation
//!
//! This module provides functionality for creating Git branches following
//! conventional naming patterns (feature/, fix/, docs/, etc.)

use crate::cli::args::BranchType;
use crate::cli::output::OutputHandler;
use crate::core::error::Result;
use crate::core::git_utils::GitUtils;
use dialoguer::{Input, Select};
use std::process::Command;

impl BranchType {
    /// Get the branch prefix (e.g., "feature/", "fix/")
    fn prefix(&self) -> &str {
        match self {
            Self::Feature => "feature/",
            Self::Fix => "fix/",
            Self::Hotfix => "hotfix/",
            Self::Refactor => "refactor/",
            Self::Docs => "docs/",
            Self::Test => "test/",
        }
    }

    /// Get the description of this branch type
    fn description(&self, output: &OutputHandler) -> String {
        match self {
            Self::Feature => output.t("git-branch-type-feature-desc"),
            Self::Fix => output.t("git-branch-type-fix-desc"),
            Self::Hotfix => output.t("git-branch-type-hotfix-desc"),
            Self::Refactor => output.t("git-branch-type-refactor-desc"),
            Self::Docs => output.t("git-branch-type-docs-desc"),
            Self::Test => output.t("git-branch-type-test-desc"),
        }
    }

    /// Get display string for selection menu
    fn display(&self, output: &OutputHandler) -> String {
        format!("{} - {}", self.prefix(), self.description(output))
    }

    /// Get all branch types
    fn all() -> Vec<Self> {
        vec![
            Self::Feature,
            Self::Fix,
            Self::Hotfix,
            Self::Refactor,
            Self::Docs,
            Self::Test,
        ]
    }
}

/// Create a conventional branch
pub fn create_branch(
    name: Option<String>,
    branch_type: Option<BranchType>,
    output: &OutputHandler,
) -> Result<()> {
    output.info(&output.t("git-branch-creating"));

    // Open the Git repository
    let git_utils = GitUtils::open_current()?;

    // Check if working directory is clean
    if !git_utils.is_clean()? {
        output.warning(&output.t("git-branch-uncommitted"));
        output.info(&output.t("git-branch-hint-clean"));

        // Ask if user wants to continue
        let cont: String = Input::new()
            .with_prompt(output.t("git-branch-confirm-continue"))
            .default("n".to_string())
            .interact_text()
            .map_err(|e| {
                crate::core::error::CldevError::command(format!("Failed to read input: {}", e))
            })?;

        if !cont.to_lowercase().starts_with('y') {
            output.info(&output.t("git-branch-cancelled"));
            return Ok(());
        }
    }

    // Get current branch for reference
    let current_branch = git_utils.current_branch()?;
    output.info(&output.t_format("git-branch-current", "branch", &current_branch));

    // Build branch name
    let branch_name = if let Some(n) = name {
        n
    } else {
        build_branch_name_interactive(branch_type, output)?
    };

    // Validate branch name
    validate_branch_name(&branch_name)?;

    // Create the branch
    output.info(&output.t_format("git-branch-creating-name", "name", &branch_name));

    let status = Command::new("git")
        .args(["checkout", "-b", &branch_name])
        .status()
        .map_err(|e| {
            crate::core::error::CldevError::command(format!("Failed to create branch: {}", e))
        })?;

    if status.success() {
        output.success(&output.t_format("git-branch-success", "name", &branch_name));

        // Show next steps
        output.info(&format!("\n{}", output.t("git-branch-next-steps")));
        output.list_item(&output.t_format("git-branch-next-work", "name", &branch_name));
        output.list_item(&output.t("git-branch-next-commit"));
        output.list_item(&output.t_format("git-branch-next-push", "name", &branch_name));
    } else {
        output.error(&output.t("git-branch-failed"));
        return Err(crate::core::error::CldevError::git(
            "Branch creation failed",
        ));
    }

    Ok(())
}

/// Build branch name interactively
fn build_branch_name_interactive(
    branch_type: Option<BranchType>,
    output: &OutputHandler,
) -> Result<String> {
    // Select branch type if not provided
    let selected_type = if let Some(bt) = branch_type {
        bt
    } else {
        output.info(&format!("\n{}", output.t("git-branch-select-type")));
        let types = BranchType::all();
        let items: Vec<String> = types.iter().map(|t| t.display(output)).collect();

        let selection = Select::new()
            .items(&items)
            .default(0)
            .interact()
            .map_err(|e| {
                crate::core::error::CldevError::command(format!(
                    "Failed to select branch type: {}",
                    e
                ))
            })?;

        types[selection]
    };

    // Ask for branch description
    output.info(&format!("\n{}", output.t("git-branch-description-prompt")));
    let description: String = Input::new().interact_text().map_err(|e| {
        crate::core::error::CldevError::command(format!("Failed to read description: {}", e))
    })?;

    // Sanitize description to kebab-case
    let sanitized = sanitize_branch_name(&description);

    // Build full branch name
    let branch_name = format!("{}{}", selected_type.prefix(), sanitized);

    output.info(&format!(
        "\n{}",
        output.t_format("git-branch-preview", "name", &branch_name)
    ));

    Ok(branch_name)
}

/// Sanitize branch name to kebab-case
fn sanitize_branch_name(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

/// Validate branch name according to Git rules
fn validate_branch_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(crate::core::error::CldevError::validation(
            "Branch name cannot be empty",
        ));
    }

    if name.starts_with('-') || name.starts_with('.') {
        return Err(crate::core::error::CldevError::validation(
            "Branch name cannot start with '-' or '.'",
        ));
    }

    if name.ends_with('/') || name.ends_with('.') {
        return Err(crate::core::error::CldevError::validation(
            "Branch name cannot end with '/' or '.'",
        ));
    }

    if name.contains("..") {
        return Err(crate::core::error::CldevError::validation(
            "Branch name cannot contain '..'",
        ));
    }

    if name.contains("//") {
        return Err(crate::core::error::CldevError::validation(
            "Branch name cannot contain '//'",
        ));
    }

    // Check for invalid characters
    if name
        .chars()
        .any(|c| matches!(c, ' ' | '~' | '^' | ':' | '?' | '*' | '[' | '\\' | '\x7f'))
    {
        return Err(crate::core::error::CldevError::validation(
            "Branch name contains invalid characters",
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_branch_type_prefix() {
        assert_eq!(BranchType::Feature.prefix(), "feature/");
        assert_eq!(BranchType::Fix.prefix(), "fix/");
        assert_eq!(BranchType::Hotfix.prefix(), "hotfix/");
    }

    #[test]
    fn test_sanitize_branch_name() {
        assert_eq!(sanitize_branch_name("Add User Auth"), "add-user-auth");
        assert_eq!(sanitize_branch_name("fix_bug_123"), "fix-bug-123");
        assert_eq!(
            sanitize_branch_name("Feature/New-Thing"),
            "feature-new-thing"
        );
        assert_eq!(sanitize_branch_name("update   docs"), "update-docs");
    }

    #[test]
    fn test_validate_branch_name_valid() {
        assert!(validate_branch_name("feature/add-auth").is_ok());
        assert!(validate_branch_name("fix/bug-123").is_ok());
        assert!(validate_branch_name("hotfix/critical-fix").is_ok());
    }

    #[test]
    fn test_validate_branch_name_invalid() {
        assert!(validate_branch_name("").is_err());
        assert!(validate_branch_name("-invalid").is_err());
        assert!(validate_branch_name(".invalid").is_err());
        assert!(validate_branch_name("invalid/").is_err());
        assert!(validate_branch_name("in..valid").is_err());
        assert!(validate_branch_name("in//valid").is_err());
        assert!(validate_branch_name("in valid").is_err());
    }
}
