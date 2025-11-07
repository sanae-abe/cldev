//! Enhanced Git status with recommendations
//!
//! This module provides an enhanced Git status display with:
//! - Clear visualization of repository state
//! - Recommended next actions
//! - Branch information and remote tracking
//! - Staged/unstaged/untracked file summary

use crate::cli::output::OutputHandler;
use crate::core::error::Result;
use crate::core::git_utils::GitUtils;
use comfy_table::{Cell, Color, Table};
use git2::{Status, StatusOptions};

/// Show enhanced Git status
pub fn show_status(detailed: bool, output: &OutputHandler) -> Result<()> {
    output.info("Git Repository Status\n");

    // Open the Git repository
    let git_utils = GitUtils::open_current()?;

    // Display branch information
    display_branch_info(&git_utils, output)?;

    // Display remote information
    display_remote_info(&git_utils, output)?;

    // Display file status
    display_file_status(&git_utils, detailed, output)?;

    // Display recommended next actions
    display_recommendations(&git_utils, output)?;

    Ok(())
}

/// Display branch information
fn display_branch_info(git_utils: &GitUtils, output: &OutputHandler) -> Result<()> {
    let branch = git_utils.current_branch()?;

    output.section("Branch Information");
    output.info(&format!("  Current branch: {}", branch));

    // Check if branch is tracking a remote
    let unpushed = match git_utils.unpushed_commits("origin") {
        Ok(count) => count,
        Err(_) => 0, // Remote might not exist
    };

    if unpushed > 0 {
        output.warning(&format!("  {} unpushed commit(s)", unpushed));
    } else {
        output.success("  Up to date with remote");
    }

    println!();
    Ok(())
}

/// Display remote information
fn display_remote_info(git_utils: &GitUtils, output: &OutputHandler) -> Result<()> {
    match git_utils.get_remote_url("origin") {
        Ok(url) => {
            output.section("Remote Information");
            output.info(&format!("  Remote URL: {}", url));

            let remote_type = git_utils.detect_remote_type("origin")?;
            output.info(&format!("  Remote type: {}", remote_type.display_name()));

            println!();
        }
        Err(_) => {
            output.warning("No remote 'origin' configured");
            println!();
        }
    }

    Ok(())
}

/// Display file status
fn display_file_status(
    _git_utils: &GitUtils,
    detailed: bool,
    output: &OutputHandler,
) -> Result<()> {
    output.section("Working Directory Status");

    let repo = git2::Repository::open_from_env().map_err(|e| {
        crate::core::error::CldevError::git(format!("Failed to open repository: {}", e))
    })?;

    let mut opts = StatusOptions::new();
    opts.include_untracked(true);
    opts.include_ignored(false);

    let statuses = repo
        .statuses(Some(&mut opts))
        .map_err(|e| crate::core::error::CldevError::git(format!("Failed to get status: {}", e)))?;

    if statuses.is_empty() {
        output.success("  Working directory clean");
        println!();
        return Ok(());
    }

    // Categorize files
    let mut staged = Vec::new();
    let mut unstaged = Vec::new();
    let mut untracked = Vec::new();

    for entry in statuses.iter() {
        let status = entry.status();
        let path = entry.path().unwrap_or("unknown");

        if status.contains(Status::INDEX_NEW)
            || status.contains(Status::INDEX_MODIFIED)
            || status.contains(Status::INDEX_DELETED)
            || status.contains(Status::INDEX_RENAMED)
        {
            staged.push((path.to_string(), status));
        }

        if status.contains(Status::WT_MODIFIED)
            || status.contains(Status::WT_DELETED)
            || status.contains(Status::WT_RENAMED)
        {
            unstaged.push((path.to_string(), status));
        }

        if status.contains(Status::WT_NEW) {
            untracked.push(path.to_string());
        }
    }

    // Display staged files
    if !staged.is_empty() {
        output.success(&format!("  Staged files ({}):", staged.len()));
        if detailed {
            for (path, status) in &staged {
                let status_str = format_file_status(*status);
                output.list_item(&format!("{:12} {}", status_str, path));
            }
        } else {
            display_file_table(&staged, output);
        }
        println!();
    }

    // Display unstaged files
    if !unstaged.is_empty() {
        output.warning(&format!("  Unstaged changes ({}):", unstaged.len()));
        if detailed {
            for (path, status) in &unstaged {
                let status_str = format_file_status(*status);
                output.list_item(&format!("{:12} {}", status_str, path));
            }
        } else {
            display_file_table(&unstaged, output);
        }
        println!();
    }

    // Display untracked files
    if !untracked.is_empty() {
        output.info(&format!("  Untracked files ({}):", untracked.len()));
        if detailed {
            for path in &untracked {
                output.list_item(path);
            }
        } else {
            let first_five: Vec<_> = untracked.iter().take(5).cloned().collect();
            for path in &first_five {
                output.list_item(path);
            }
            if untracked.len() > 5 {
                output.list_item(&format!("... and {} more", untracked.len() - 5));
            }
        }
        println!();
    }

    Ok(())
}

/// Format file status as a string
fn format_file_status(status: Status) -> &'static str {
    if status.contains(Status::INDEX_NEW) || status.contains(Status::WT_NEW) {
        "new file"
    } else if status.contains(Status::INDEX_MODIFIED) || status.contains(Status::WT_MODIFIED) {
        "modified"
    } else if status.contains(Status::INDEX_DELETED) || status.contains(Status::WT_DELETED) {
        "deleted"
    } else if status.contains(Status::INDEX_RENAMED) || status.contains(Status::WT_RENAMED) {
        "renamed"
    } else {
        "unknown"
    }
}

/// Display files in a compact table format
fn display_file_table(files: &[(String, Status)], _output: &OutputHandler) {
    let mut table = Table::new();
    table.load_preset(comfy_table::presets::NOTHING);

    // Group files by status
    let mut new_files = Vec::new();
    let mut modified_files = Vec::new();
    let mut deleted_files = Vec::new();

    for (path, status) in files {
        if status.contains(Status::INDEX_NEW) || status.contains(Status::WT_NEW) {
            new_files.push(path);
        } else if status.contains(Status::INDEX_MODIFIED) || status.contains(Status::WT_MODIFIED) {
            modified_files.push(path);
        } else if status.contains(Status::INDEX_DELETED) || status.contains(Status::WT_DELETED) {
            deleted_files.push(path);
        }
    }

    if !new_files.is_empty() {
        table.add_row(vec![Cell::new(format!(
            "  {} new file(s)",
            new_files.len()
        ))
        .fg(Color::Green)]);
    }
    if !modified_files.is_empty() {
        table.add_row(vec![Cell::new(format!(
            "  {} modified",
            modified_files.len()
        ))
        .fg(Color::Yellow)]);
    }
    if !deleted_files.is_empty() {
        table.add_row(vec![Cell::new(format!(
            "  {} deleted",
            deleted_files.len()
        ))
        .fg(Color::Red)]);
    }

    println!("{}", table);
}

/// Display recommended next actions
fn display_recommendations(git_utils: &GitUtils, output: &OutputHandler) -> Result<()> {
    output.section("Recommended Next Actions");

    let is_clean = git_utils.is_clean()?;
    let unpushed = git_utils.unpushed_commits("origin").unwrap_or(0);

    if is_clean && unpushed == 0 {
        output.success("  ✅ All up to date! Ready to start new work.");
        output.info("\n  Suggestions:");
        output.list_item("cldev git branch - Create a new branch");
        output.list_item("Start working on a new feature");
    } else if !is_clean {
        output.info("  You have uncommitted changes:");
        output.list_item("git add <files> - Stage specific files");
        output.list_item("git add . - Stage all changes");
        output.list_item("cldev git commit - Create a conventional commit");
        output.list_item("git stash - Temporarily save changes");
    } else if unpushed > 0 {
        output.info("  You have unpushed commits:");
        output.list_item(&format!("git push - Push {} commit(s) to remote", unpushed));
        output.list_item("cldev git merge-request - Create a MR/PR");
    }

    println!();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use git2::Status;

    #[test]
    fn test_format_file_status() {
        assert_eq!(format_file_status(Status::INDEX_NEW), "new file");
        assert_eq!(format_file_status(Status::INDEX_MODIFIED), "modified");
        assert_eq!(format_file_status(Status::INDEX_DELETED), "deleted");
        assert_eq!(format_file_status(Status::WT_MODIFIED), "modified");
    }
}
