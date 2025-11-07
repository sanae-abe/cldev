//! Git utility functions for repository operations
//!
//! This module provides shared Git functionality for:
//! - Remote URL detection and parsing
//! - Remote type detection (GitHub/GitLab)
//! - CLI tool availability checks (gh/glab)
//! - Repository state inspection

use crate::core::error::{CldevError, Result};
use git2::{Repository, StatusOptions};
use std::path::Path;
use std::process::Command;

/// Remote hosting service type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemoteType {
    GitHub,
    GitLab,
    Other,
}

impl RemoteType {
    /// Get the display name for the remote type
    pub fn display_name(&self) -> &str {
        match self {
            RemoteType::GitHub => "GitHub",
            RemoteType::GitLab => "GitLab",
            RemoteType::Other => "Unknown",
        }
    }

    /// Get the CLI tool name for this remote type
    pub fn cli_tool(&self) -> Option<&str> {
        match self {
            RemoteType::GitHub => Some("gh"),
            RemoteType::GitLab => Some("glab"),
            RemoteType::Other => None,
        }
    }
}

/// Git repository wrapper for utility operations
pub struct GitUtils {
    repo: Repository,
}

impl GitUtils {
    /// Open a Git repository at the current directory or parent directories
    pub fn open_current() -> Result<Self> {
        let repo = Repository::open_from_env()
            .map_err(|e| CldevError::Git(format!("Failed to open Git repository: {}", e)))?;
        Ok(Self { repo })
    }

    /// Open a Git repository at a specific path
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let repo = Repository::open(path.as_ref())
            .map_err(|e| CldevError::Git(format!("Failed to open Git repository: {}", e)))?;
        Ok(Self { repo })
    }

    /// Get the remote URL for a given remote name
    pub fn get_remote_url(&self, remote_name: &str) -> Result<String> {
        let remote = self.repo.find_remote(remote_name).map_err(|e| {
            CldevError::Git(format!("Failed to find remote '{}': {}", remote_name, e))
        })?;

        let url = remote.url().ok_or_else(|| {
            CldevError::Git(format!("Remote '{}' has no URL configured", remote_name))
        })?;

        Ok(url.to_string())
    }

    /// Detect the remote type from a remote URL
    pub fn detect_remote_type(&self, remote_name: &str) -> Result<RemoteType> {
        let url = self.get_remote_url(remote_name)?;
        Ok(Self::detect_remote_type_from_url(&url))
    }

    /// Detect remote type from a URL string
    pub fn detect_remote_type_from_url(url: &str) -> RemoteType {
        let url_lower = url.to_lowercase();

        if url_lower.contains("github.com") {
            RemoteType::GitHub
        } else if url_lower.contains("gitlab.com") || url_lower.contains("gitlab") {
            RemoteType::GitLab
        } else {
            RemoteType::Other
        }
    }

    /// Get the current branch name
    pub fn current_branch(&self) -> Result<String> {
        let head = self
            .repo
            .head()
            .map_err(|e| CldevError::Git(format!("Failed to get HEAD reference: {}", e)))?;

        let branch_name = head
            .shorthand()
            .ok_or_else(|| CldevError::Git("Failed to get branch name from HEAD".to_string()))?;

        Ok(branch_name.to_string())
    }

    /// Check if the working directory is clean (no uncommitted changes)
    pub fn is_clean(&self) -> Result<bool> {
        let mut opts = StatusOptions::new();
        opts.include_untracked(true);
        opts.include_ignored(false);

        let statuses = self
            .repo
            .statuses(Some(&mut opts))
            .map_err(|e| CldevError::Git(format!("Failed to get repository status: {}", e)))?;

        Ok(statuses.is_empty())
    }

    /// Get a list of changed files
    pub fn changed_files(&self) -> Result<Vec<String>> {
        let mut opts = StatusOptions::new();
        opts.include_untracked(true);
        opts.include_ignored(false);

        let statuses = self
            .repo
            .statuses(Some(&mut opts))
            .map_err(|e| CldevError::Git(format!("Failed to get repository status: {}", e)))?;

        let mut files = Vec::new();
        for entry in statuses.iter() {
            if let Some(path) = entry.path() {
                files.push(path.to_string());
            }
        }

        Ok(files)
    }

    /// Get the count of unpushed commits
    pub fn unpushed_commits(&self, remote_name: &str) -> Result<usize> {
        let local_branch = self.current_branch()?;
        let remote_branch = format!("{}/{}", remote_name, local_branch);

        // Get local commit
        let local_ref = self
            .repo
            .find_reference(&format!("refs/heads/{}", local_branch))
            .map_err(|e| CldevError::Git(format!("Failed to find local branch: {}", e)))?;
        let local_oid = local_ref
            .target()
            .ok_or_else(|| CldevError::Git("Failed to get local commit OID".to_string()))?;

        // Try to get remote commit
        let remote_ref = match self
            .repo
            .find_reference(&format!("refs/remotes/{}", remote_branch))
        {
            Ok(r) => r,
            Err(_) => return Ok(0), // Remote branch doesn't exist yet
        };
        let remote_oid = remote_ref
            .target()
            .ok_or_else(|| CldevError::Git("Failed to get remote commit OID".to_string()))?;

        // Count commits between remote and local
        let mut revwalk = self
            .repo
            .revwalk()
            .map_err(|e| CldevError::Git(format!("Failed to create revision walker: {}", e)))?;
        revwalk
            .push(local_oid)
            .map_err(|e| CldevError::Git(format!("Failed to push local OID: {}", e)))?;
        revwalk
            .hide(remote_oid)
            .map_err(|e| CldevError::Git(format!("Failed to hide remote OID: {}", e)))?;

        Ok(revwalk.count())
    }

    /// Get the repository root path
    pub fn repo_path(&self) -> &Path {
        self.repo.path()
    }

    /// Get the working directory path
    pub fn workdir(&self) -> Result<&Path> {
        self.repo
            .workdir()
            .ok_or_else(|| CldevError::Git("Repository has no working directory".to_string()))
    }

    /// Create and checkout a new branch
    pub fn create_branch(&self, branch_name: &str) -> Result<()> {
        // Get the current HEAD commit
        let head = self
            .repo
            .head()
            .map_err(|e| CldevError::Git(format!("Failed to get HEAD reference: {}", e)))?;

        let head_commit = head
            .peel_to_commit()
            .map_err(|e| CldevError::Git(format!("Failed to get HEAD commit: {}", e)))?;

        // Create the new branch
        self.repo
            .branch(branch_name, &head_commit, false)
            .map_err(|e| {
                CldevError::Git(format!("Failed to create branch '{}': {}", branch_name, e))
            })?;

        // Checkout the new branch
        self.repo
            .set_head(&format!("refs/heads/{}", branch_name))
            .map_err(|e| {
                CldevError::Git(format!(
                    "Failed to checkout branch '{}': {}",
                    branch_name, e
                ))
            })?;

        // Update the working directory
        self.repo
            .checkout_head(None)
            .map_err(|e| CldevError::Git(format!("Failed to update working directory: {}", e)))?;

        Ok(())
    }
}

/// Check if the GitHub CLI (gh) is installed and available
pub fn check_gh_cli() -> Result<bool> {
    match which::which("gh") {
        Ok(_) => {
            // Verify it's actually working by running --version
            match Command::new("gh").arg("--version").output() {
                Ok(output) => Ok(output.status.success()),
                Err(_) => Ok(false),
            }
        }
        Err(_) => Ok(false),
    }
}

/// Check if the GitLab CLI (glab) is installed and available
pub fn check_glab_cli() -> Result<bool> {
    match which::which("glab") {
        Ok(_) => {
            // Verify it's actually working by running --version
            match Command::new("glab").arg("--version").output() {
                Ok(output) => Ok(output.status.success()),
                Err(_) => Ok(false),
            }
        }
        Err(_) => Ok(false),
    }
}

/// Check if a CLI tool is available for the given remote type
pub fn check_cli_for_remote(remote_type: RemoteType) -> Result<bool> {
    match remote_type {
        RemoteType::GitHub => check_gh_cli(),
        RemoteType::GitLab => check_glab_cli(),
        RemoteType::Other => Ok(false),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_remote_type_github() {
        assert_eq!(
            GitUtils::detect_remote_type_from_url("https://github.com/user/repo.git"),
            RemoteType::GitHub
        );
        assert_eq!(
            GitUtils::detect_remote_type_from_url("git@github.com:user/repo.git"),
            RemoteType::GitHub
        );
    }

    #[test]
    fn test_detect_remote_type_gitlab() {
        assert_eq!(
            GitUtils::detect_remote_type_from_url("https://gitlab.com/user/repo.git"),
            RemoteType::GitLab
        );
        assert_eq!(
            GitUtils::detect_remote_type_from_url("git@gitlab.com:user/repo.git"),
            RemoteType::GitLab
        );
        assert_eq!(
            GitUtils::detect_remote_type_from_url("https://gitlab.example.com/user/repo.git"),
            RemoteType::GitLab
        );
    }

    #[test]
    fn test_detect_remote_type_other() {
        assert_eq!(
            GitUtils::detect_remote_type_from_url("https://bitbucket.org/user/repo.git"),
            RemoteType::Other
        );
    }
}
