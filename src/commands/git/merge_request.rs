//! Merge Request / Pull Request creation
//!
//! This module provides functionality for creating merge requests (GitLab)
//! or pull requests (GitHub) with automatic platform detection and CLI tool usage.

use crate::cli::output::OutputHandler;
use crate::core::error::Result;
use crate::core::git_utils::{check_cli_for_remote, GitUtils, RemoteType};
use dialoguer::Input;
use std::process::Command;

/// Create a merge request (GitLab) or pull request (GitHub)
pub fn create_merge_request(
    target: &str,
    title: Option<String>,
    detailed: bool,
    output: &OutputHandler,
) -> Result<()> {
    output.info(&output.t("git-mr-creating"));

    // Open the Git repository
    let git_utils = GitUtils::open_current()?;

    // Get current branch
    let current_branch = git_utils.current_branch()?;
    output.info(&output.t_format("git-mr-current-branch", "branch", &current_branch));

    // Check if we're on the target branch
    if current_branch == target {
        output.error(&output.t_format("git-mr-error-same-branch", "target", target));
        output.info(&output.t("git-mr-hint-branch"));
        return Err(crate::core::error::CldevError::validation(
            "Cannot create MR/PR from target branch",
        ));
    }

    // Detect remote type
    let remote_type = git_utils.detect_remote_type("origin")?;
    output.info(&output.t_format("git-mr-remote-detected", "type", remote_type.display_name()));

    // Check if CLI tool is available
    if !check_cli_for_remote(remote_type)? {
        let tool = remote_type.cli_tool().unwrap_or("unknown");
        output.error(&output.t_format("git-mr-tool-not-found", "tool", tool));
        output.info(&output.t_format("git-mr-tool-install", "type", remote_type.display_name()));

        match remote_type {
            RemoteType::GitHub => {
                output.list_item("Visit: https://cli.github.com/");
                output.list_item("Or: brew install gh");
            }
            RemoteType::GitLab => {
                output.list_item("Visit: https://gitlab.com/gitlab-org/cli");
                output.list_item("Or: brew install glab");
            }
            RemoteType::Other => {
                output.list_item("Manual MR/PR creation required");
            }
        }

        return Err(crate::core::error::CldevError::command(format!(
            "Required CLI tool '{}' not found",
            tool
        )));
    }

    // Check for unpushed commits
    let unpushed = git_utils.unpushed_commits("origin")?;
    if unpushed > 0 {
        output.warning(&output.t_format("git-mr-unpushed", "count", &unpushed.to_string()));
        output.info(&output.t("git-mr-pushing"));

        let status = Command::new("git")
            .args(["push", "-u", "origin", &current_branch])
            .status()
            .map_err(|e| {
                crate::core::error::CldevError::command(format!("Failed to push: {}", e))
            })?;

        if !status.success() {
            output.error(&output.t("git-mr-push-failed"));
            return Err(crate::core::error::CldevError::git("Push failed"));
        }

        output.success(&output.t("git-mr-push-success"));
    }

    // Get or generate MR/PR title
    let mr_title = if let Some(t) = title {
        t
    } else {
        generate_mr_title(&current_branch, output)?
    };

    // Generate MR/PR body
    let body = generate_mr_body(&git_utils, &current_branch, target, detailed, output)?;

    // Create MR/PR based on remote type
    match remote_type {
        RemoteType::GitHub => {
            create_github_pr(&mr_title, &body, target, output)?;
        }
        RemoteType::GitLab => {
            create_gitlab_mr(&mr_title, &body, target, output)?;
        }
        RemoteType::Other => {
            output.error("Unsupported remote type for automatic MR/PR creation");
            return Err(crate::core::error::CldevError::validation(
                "Unsupported remote type",
            ));
        }
    }

    Ok(())
}

/// Generate MR/PR title from branch name
fn generate_mr_title(branch: &str, output: &OutputHandler) -> Result<String> {
    // Extract meaningful title from branch name
    let suggested_title = branch
        .split('/')
        .next_back()
        .unwrap_or(branch)
        .split('-')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ");

    output.info(&format!(
        "\n{}",
        output.t_format("git-mr-suggested-title", "title", &suggested_title)
    ));
    output.info(&output.t("git-mr-title-prompt"));

    let title: String = Input::new()
        .default(suggested_title.clone())
        .interact_text()
        .map_err(|e| {
            crate::core::error::CldevError::command(format!("Failed to read title: {}", e))
        })?;

    Ok(title)
}

/// Generate MR/PR body
fn generate_mr_body(
    git_utils: &GitUtils,
    branch: &str,
    target: &str,
    detailed: bool,
    output: &OutputHandler,
) -> Result<String> {
    let mut body = String::new();

    // Add summary section
    body.push_str("## Summary\n\n");

    if detailed {
        // Get commit messages for summary
        let commit_log = get_commit_log(git_utils, branch, target)?;
        body.push_str("Changes in this MR/PR:\n\n");
        for commit in commit_log.lines() {
            body.push_str(&format!("- {}\n", commit));
        }
    } else {
        body.push_str("<!-- Describe your changes here -->\n");
    }

    body.push('\n');

    // Add test plan section
    body.push_str("## Test Plan\n\n");
    body.push_str("- [ ] Unit tests pass\n");
    body.push_str("- [ ] Integration tests pass\n");
    body.push_str("- [ ] Manual testing completed\n");
    body.push('\n');

    // Add checklist section
    body.push_str("## Checklist\n\n");
    body.push_str("- [ ] Code follows project style guidelines\n");
    body.push_str("- [ ] Self-review completed\n");
    body.push_str("- [ ] Documentation updated (if needed)\n");
    body.push_str("- [ ] No breaking changes (or documented)\n");
    body.push('\n');

    // Add attribution
    body.push_str("---\n\n");
    body.push_str("🤖 Generated with [Claude Code](https://claude.com/claude-code)\n");

    output.debug(&format!("Generated body:\n{}", body));

    Ok(body)
}

/// Get commit log between target and current branch
fn get_commit_log(_git_utils: &GitUtils, branch: &str, target: &str) -> Result<String> {
    let output = Command::new("git")
        .args(["log", "--oneline", &format!("{}..{}", target, branch)])
        .output()
        .map_err(|e| {
            crate::core::error::CldevError::command(format!("Failed to get commit log: {}", e))
        })?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Create GitHub pull request
fn create_github_pr(title: &str, body: &str, target: &str, output: &OutputHandler) -> Result<()> {
    output.info(&output.t("git-mr-creating-github"));

    let status = Command::new("gh")
        .args([
            "pr", "create", "--title", title, "--body", body, "--base", target,
        ])
        .status()
        .map_err(|e| {
            crate::core::error::CldevError::command(format!("Failed to create PR: {}", e))
        })?;

    if status.success() {
        output.success(&output.t("git-mr-github-success"));

        // Get PR URL
        let pr_output = Command::new("gh")
            .args(["pr", "view", "--json", "url", "--jq", ".url"])
            .output()
            .ok();

        if let Some(pr) = pr_output {
            if let Ok(url) = String::from_utf8(pr.stdout) {
                output.info(&output.t_format("git-mr-pr-url", "url", url.trim()));
            }
        }

        output.info(&format!("\n{}", output.t("git-mr-next-steps")));
        output.list_item(&output.t("git-mr-next-github-view"));
        output.list_item(&output.t("git-mr-next-github-checks"));
    } else {
        output.error(&output.t("git-mr-github-failed"));
        return Err(crate::core::error::CldevError::git("PR creation failed"));
    }

    Ok(())
}

/// Create GitLab merge request
fn create_gitlab_mr(title: &str, body: &str, target: &str, output: &OutputHandler) -> Result<()> {
    output.info(&output.t("git-mr-creating-gitlab"));

    let status = Command::new("glab")
        .args([
            "mr",
            "create",
            "--title",
            title,
            "--description",
            body,
            "--target-branch",
            target,
        ])
        .status()
        .map_err(|e| {
            crate::core::error::CldevError::command(format!("Failed to create MR: {}", e))
        })?;

    if status.success() {
        output.success(&output.t("git-mr-gitlab-success"));

        output.info(&format!("\n{}", output.t("git-mr-next-steps")));
        output.list_item(&output.t("git-mr-next-gitlab-view"));
        output.list_item(&output.t("git-mr-next-gitlab-ci"));
    } else {
        output.error(&output.t("git-mr-gitlab-failed"));
        return Err(crate::core::error::CldevError::git("MR creation failed"));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    

    #[test]
    fn test_generate_title_from_branch() {
        // This is a simplified test - actual function is interactive
        let branch = "feature/add-user-authentication";
        let parts: Vec<&str> = branch.split('/').collect();
        let suggested = parts
            .last()
            .unwrap_or(&branch)
            .split('-')
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ");

        assert_eq!(suggested, "Add User Authentication");
    }
}
