//! Conventional Commits implementation
//!
//! This module provides functionality for creating Git commits following
//! the Conventional Commits specification with emoji support and
//! Co-Authored-By attribution for Claude.

use crate::cli::output::OutputHandler;
use crate::core::error::Result;
use crate::core::git_utils::GitUtils;
use dialoguer::{Input, Select};
use std::process::Command;

/// Conventional commit types with their corresponding emojis
#[derive(Debug, Clone, Copy)]
enum CommitType {
    Feat,
    Fix,
    Docs,
    Style,
    Refactor,
    Perf,
    Test,
    Build,
    Ci,
    Chore,
    Revert,
}

impl CommitType {
    /// Get all commit types
    fn all() -> Vec<Self> {
        vec![
            Self::Feat,
            Self::Fix,
            Self::Docs,
            Self::Style,
            Self::Refactor,
            Self::Perf,
            Self::Test,
            Self::Build,
            Self::Ci,
            Self::Chore,
            Self::Revert,
        ]
    }

    /// Get the commit type prefix (e.g., "feat", "fix")
    fn prefix(&self) -> &str {
        match self {
            Self::Feat => "feat",
            Self::Fix => "fix",
            Self::Docs => "docs",
            Self::Style => "style",
            Self::Refactor => "refactor",
            Self::Perf => "perf",
            Self::Test => "test",
            Self::Build => "build",
            Self::Ci => "ci",
            Self::Chore => "chore",
            Self::Revert => "revert",
        }
    }

    /// Get the emoji for this commit type
    fn emoji(&self) -> &str {
        match self {
            Self::Feat => "✨",
            Self::Fix => "🐛",
            Self::Docs => "📝",
            Self::Style => "💄",
            Self::Refactor => "♻️",
            Self::Perf => "⚡",
            Self::Test => "✅",
            Self::Build => "📦",
            Self::Ci => "👷",
            Self::Chore => "🔧",
            Self::Revert => "⏪",
        }
    }

    /// Get the description of this commit type
    fn description(&self, output: &OutputHandler) -> String {
        match self {
            Self::Feat => output.t("git-commit-type-feat-desc"),
            Self::Fix => output.t("git-commit-type-fix-desc"),
            Self::Docs => output.t("git-commit-type-docs-desc"),
            Self::Style => output.t("git-commit-type-style-desc"),
            Self::Refactor => output.t("git-commit-type-refactor-desc"),
            Self::Perf => output.t("git-commit-type-perf-desc"),
            Self::Test => output.t("git-commit-type-test-desc"),
            Self::Build => output.t("git-commit-type-build-desc"),
            Self::Ci => output.t("git-commit-type-ci-desc"),
            Self::Chore => output.t("git-commit-type-chore-desc"),
            Self::Revert => output.t("git-commit-type-revert-desc"),
        }
    }

    /// Get display string for selection menu
    fn display(&self, output: &OutputHandler) -> String {
        format!(
            "{} {} - {}",
            self.emoji(),
            self.prefix(),
            self.description(output)
        )
    }

    /// Detect commit type from changed files
    fn detect_from_files(files: &[String]) -> Option<Self> {
        let has_test_files = files.iter().any(|f| {
            f.contains("test")
                || f.contains("spec")
                || f.ends_with("_test.rs")
                || f.ends_with(".test.ts")
        });

        let has_doc_files = files
            .iter()
            .any(|f| f.ends_with(".md") || f.starts_with("docs/") || f == "README.md");

        let has_ci_files = files.iter().any(|f| {
            f.starts_with(".github/")
                || f.starts_with(".gitlab/")
                || f.ends_with(".yml")
                || f.ends_with(".yaml")
        });

        let has_build_files = files.iter().any(|f| {
            f == "Cargo.toml" || f == "package.json" || f == "Dockerfile" || f == "Makefile"
        });

        // Prioritize by specificity
        if has_test_files && files.len() == files.iter().filter(|f| f.contains("test")).count() {
            Some(Self::Test)
        } else if has_doc_files
            && files.len() == files.iter().filter(|f| f.ends_with(".md")).count()
        {
            Some(Self::Docs)
        } else if has_ci_files {
            Some(Self::Ci)
        } else if has_build_files {
            Some(Self::Build)
        } else {
            None // Let user choose
        }
    }
}

/// Create a conventional commit
pub fn create_commit(
    message: Option<String>,
    no_verify: bool,
    amend: bool,
    output: &OutputHandler,
) -> Result<()> {
    output.info(&output.t("git-commit-creating"));

    // Open the Git repository
    let git_utils = GitUtils::open_current()?;

    // Check if there are changes to commit (unless amending)
    if !amend {
        let files = git_utils.changed_files()?;
        if files.is_empty() {
            output.warning(&output.t("git-commit-no-changes"));
            output.info(&output.t("git-commit-hint-stage"));
            return Ok(());
        }

        output.info(&output.t_format("git-commit-files-count", "count", &files.len().to_string()));
        for file in &files {
            output.list_item(file);
        }
    }

    // If message is provided, use it directly
    let commit_message = if let Some(msg) = message {
        msg
    } else {
        // Interactive mode: build commit message
        build_commit_message_interactive(&git_utils, output)?
    };

    // Add Claude attribution (localized)
    let full_message = format!(
        "{}\n\n🤖 {}\n\n{}",
        commit_message,
        output.t("git-commit-attribution"),
        output.t("git-commit-coauthor")
    );

    // Execute git commit
    let mut cmd = Command::new("git");
    cmd.arg("commit").arg("-m").arg(&full_message);

    if no_verify {
        cmd.arg("--no-verify");
    }

    if amend {
        cmd.arg("--amend");
    }

    output.debug(&format!("Executing: git commit -m \"{}\"", commit_message));

    let status = cmd.status().map_err(|e| {
        crate::core::error::CldevError::command(format!("Failed to execute git commit: {}", e))
    })?;

    if status.success() {
        output.success(&output.t("git-commit-success"));
        output.info(&output.t_format("git-commit-message", "message", &commit_message));

        // Show next steps
        output.info(&format!("\n{}", output.t("git-commit-next-steps")));
        output.list_item(&output.t("git-commit-next-push"));
        output.list_item(&output.t("git-commit-next-status"));
    } else {
        output.error(&output.t("git-commit-failed"));
        return Err(crate::core::error::CldevError::git("Commit failed"));
    }

    Ok(())
}

/// Build commit message interactively
fn build_commit_message_interactive(
    git_utils: &GitUtils,
    output: &OutputHandler,
) -> Result<String> {
    // Detect commit type from changed files
    let files = git_utils.changed_files()?;
    let detected_type = CommitType::detect_from_files(&files);

    // Select commit type
    let types = CommitType::all();
    let items: Vec<String> = types.iter().map(|t| t.display(output)).collect();

    let default_index = if let Some(detected) = detected_type {
        types
            .iter()
            .position(|t| std::mem::discriminant(t) == std::mem::discriminant(&detected))
            .unwrap_or(0)
    } else {
        0
    };

    output.info(&format!("\n{}", output.t("git-commit-select-type")));
    let selection = Select::new()
        .items(&items)
        .default(default_index)
        .interact()
        .map_err(|e| {
            crate::core::error::CldevError::command(format!("Failed to select commit type: {}", e))
        })?;

    let commit_type = &types[selection];

    // Display selected commit type
    output.success(&format!(
        "Selected: {} {}",
        commit_type.emoji(),
        commit_type.prefix()
    ));

    // Ask for scope (optional)
    output.info(&format!("\n{}", output.t("git-commit-scope-prompt")));
    let scope: String = Input::new()
        .allow_empty(true)
        .interact_text()
        .map_err(|e| {
            crate::core::error::CldevError::command(format!("Failed to read scope: {}", e))
        })?;

    // Suggest description based on changes
    let suggested_description = git_utils
        .suggest_commit_description()
        .unwrap_or_else(|_| String::new());

    // Ask for description (with suggestion as default if available)
    output.info(&format!("\n{}", output.t("git-commit-description-prompt")));
    let description: String = if !suggested_description.is_empty() {
        Input::new()
            .with_initial_text(&suggested_description)
            .interact_text()
            .map_err(|e| {
                crate::core::error::CldevError::command(format!(
                    "Failed to read description: {}",
                    e
                ))
            })?
    } else {
        Input::new().interact_text().map_err(|e| {
            crate::core::error::CldevError::command(format!("Failed to read description: {}", e))
        })?
    };

    // Ask if breaking change
    output.info(&format!("\n{}", output.t("git-commit-breaking-prompt")));
    let breaking: String = Input::new()
        .default("n".to_string())
        .interact_text()
        .map_err(|e| {
            crate::core::error::CldevError::command(format!(
                "Failed to read breaking change: {}",
                e
            ))
        })?;

    let is_breaking = breaking.to_lowercase().starts_with('y');

    // Build the commit message
    let scope_part = if scope.is_empty() {
        String::new()
    } else {
        format!("({})", scope)
    };

    let breaking_part = if is_breaking { "!" } else { "" };

    let message = format!(
        "{} {}{}{}: {}",
        commit_type.emoji(),
        commit_type.prefix(),
        scope_part,
        breaking_part,
        description
    );

    output.info(&format!(
        "\n{}\n{}",
        output.t("git-commit-preview"),
        message
    ));

    Ok(message)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commit_type_prefix() {
        assert_eq!(CommitType::Feat.prefix(), "feat");
        assert_eq!(CommitType::Fix.prefix(), "fix");
        assert_eq!(CommitType::Docs.prefix(), "docs");
    }

    #[test]
    fn test_commit_type_emoji() {
        assert_eq!(CommitType::Feat.emoji(), "✨");
        assert_eq!(CommitType::Fix.emoji(), "🐛");
        assert_eq!(CommitType::Docs.emoji(), "📝");
    }

    #[test]
    fn test_detect_from_test_files() {
        let files = vec![
            "src/lib_test.rs".to_string(),
            "tests/integration.rs".to_string(),
        ];
        let detected = CommitType::detect_from_files(&files);
        assert!(matches!(detected, Some(CommitType::Test)));
    }

    #[test]
    fn test_detect_from_doc_files() {
        let files = vec!["README.md".to_string(), "docs/guide.md".to_string()];
        let detected = CommitType::detect_from_files(&files);
        assert!(matches!(detected, Some(CommitType::Docs)));
    }

    #[test]
    fn test_detect_from_ci_files() {
        let files = vec![".github/workflows/ci.yml".to_string()];
        let detected = CommitType::detect_from_files(&files);
        assert!(matches!(detected, Some(CommitType::Ci)));
    }

    #[test]
    fn test_detect_from_build_files() {
        let files = vec!["Cargo.toml".to_string()];
        let detected = CommitType::detect_from_files(&files);
        assert!(matches!(detected, Some(CommitType::Build)));
    }

    #[test]
    fn test_commit_message_localization() {
        use crate::cli::output::OutputHandler;

        // Test English
        let output_en = OutputHandler::new(false, false, Some("en".to_string()));
        let attribution_en = output_en.t("git-commit-attribution");
        let coauthor_en = output_en.t("git-commit-coauthor");
        assert!(attribution_en.contains("Generated with"));
        assert!(coauthor_en.contains("Co-Authored-By"));

        // Test Japanese
        let output_ja = OutputHandler::new(false, false, Some("ja".to_string()));
        let attribution_ja = output_ja.t("git-commit-attribution");
        let coauthor_ja = output_ja.t("git-commit-coauthor");
        assert!(attribution_ja.contains("で生成"));
        assert!(coauthor_ja.contains("共同著者"));
    }
}
