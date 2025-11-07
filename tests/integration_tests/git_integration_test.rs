//! Integration tests for Git operations
//!
//! Tests Git utility functions with actual Git repositories,
//! including remote detection, branch operations, and status checks.

use cldev::core::error::Result;
use cldev::core::git_utils::{GitUtils, RemoteType};
use git2::Repository;
use std::fs;
use std::process::Command;
use tempfile::TempDir;

/// Helper function to initialize a Git repository
fn init_git_repo(path: &std::path::Path) -> Result<Repository> {
    let repo = Repository::init(path)?;

    // Configure user (required for commits)
    let mut config = repo.config()?;
    config.set_str("user.name", "Test User")?;
    config.set_str("user.email", "test@example.com")?;

    Ok(repo)
}

/// Helper function to create an initial commit
fn create_initial_commit(repo: &Repository) -> Result<()> {
    let path = repo.workdir().unwrap();

    // Create a file
    let file_path = path.join("README.md");
    fs::write(&file_path, "# Test Repository\n")?;

    // Stage the file
    let mut index = repo.index()?;
    index.add_path(std::path::Path::new("README.md"))?;
    index.write()?;

    // Create commit
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    let signature = repo.signature()?;

    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        "Initial commit",
        &tree,
        &[],
    )?;

    Ok(())
}

#[test]
fn test_git_utils_open_repository() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let repo_path = temp_dir.path();

    // Initialize repository
    init_git_repo(repo_path)?;

    // Open with GitUtils
    let git_utils = GitUtils::open(repo_path)?;
    assert!(git_utils.repo_path().exists());

    Ok(())
}

#[test]
fn test_git_utils_current_branch() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let repo = init_git_repo(temp_dir.path())?;
    create_initial_commit(&repo)?;

    let git_utils = GitUtils::open(temp_dir.path())?;
    let branch = git_utils.current_branch()?;

    // Default branch should be 'main' or 'master'
    assert!(branch == "main" || branch == "master");

    Ok(())
}

#[test]
fn test_git_utils_create_branch() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let repo = init_git_repo(temp_dir.path())?;
    create_initial_commit(&repo)?;

    let git_utils = GitUtils::open(temp_dir.path())?;

    // Create new branch
    git_utils.create_branch("feature/test-branch")?;

    // Verify we're on the new branch
    let current = git_utils.current_branch()?;
    assert_eq!(current, "feature/test-branch");

    Ok(())
}

#[test]
fn test_git_utils_is_clean() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let repo = init_git_repo(temp_dir.path())?;
    create_initial_commit(&repo)?;

    let git_utils = GitUtils::open(temp_dir.path())?;

    // Should be clean after commit
    assert!(git_utils.is_clean()?);

    // Create untracked file
    let untracked = temp_dir.path().join("untracked.txt");
    fs::write(&untracked, "test")?;

    // Should not be clean
    assert!(!git_utils.is_clean()?);

    Ok(())
}

#[test]
fn test_git_utils_changed_files() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let repo = init_git_repo(temp_dir.path())?;
    create_initial_commit(&repo)?;

    let git_utils = GitUtils::open(temp_dir.path())?;

    // No changes initially
    assert_eq!(git_utils.changed_files()?.len(), 0);

    // Create new file
    fs::write(temp_dir.path().join("new.txt"), "content")?;

    // Should detect the new file
    let changed = git_utils.changed_files()?;
    assert_eq!(changed.len(), 1);
    assert!(changed.contains(&"new.txt".to_string()));

    // Modify existing file
    fs::write(temp_dir.path().join("README.md"), "# Modified\n")?;

    // Should detect both files
    let changed = git_utils.changed_files()?;
    assert_eq!(changed.len(), 2);

    Ok(())
}

#[test]
fn test_git_utils_detect_github_remote() -> Result<()> {
    let github_urls = vec![
        "https://github.com/user/repo.git",
        "git@github.com:user/repo.git",
        "https://github.com/org/project",
    ];

    for url in github_urls {
        let remote_type = GitUtils::detect_remote_type_from_url(url);
        assert_eq!(remote_type, RemoteType::GitHub);
    }

    Ok(())
}

#[test]
fn test_git_utils_detect_gitlab_remote() -> Result<()> {
    let gitlab_urls = vec![
        "https://gitlab.com/user/repo.git",
        "git@gitlab.com:user/repo.git",
        "https://gitlab.example.com/org/project.git",
    ];

    for url in gitlab_urls {
        let remote_type = GitUtils::detect_remote_type_from_url(url);
        assert_eq!(remote_type, RemoteType::GitLab);
    }

    Ok(())
}

#[test]
fn test_git_utils_detect_other_remote() -> Result<()> {
    let other_urls = vec![
        "https://bitbucket.org/user/repo.git",
        "https://example.com/git/repo.git",
        "git@example.com:repo.git",
    ];

    for url in other_urls {
        let remote_type = GitUtils::detect_remote_type_from_url(url);
        assert_eq!(remote_type, RemoteType::Other);
    }

    Ok(())
}

#[test]
fn test_git_utils_remote_url() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let repo = init_git_repo(temp_dir.path())?;
    create_initial_commit(&repo)?;

    // Add remote
    repo.remote("origin", "https://github.com/user/repo.git")?;

    let git_utils = GitUtils::open(temp_dir.path())?;
    let url = git_utils.get_remote_url("origin")?;

    assert_eq!(url, "https://github.com/user/repo.git");

    Ok(())
}

#[test]
fn test_git_utils_detect_remote_type() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let repo = init_git_repo(temp_dir.path())?;
    create_initial_commit(&repo)?;

    // Add GitHub remote
    repo.remote("origin", "https://github.com/user/repo.git")?;

    let git_utils = GitUtils::open(temp_dir.path())?;
    let remote_type = git_utils.detect_remote_type("origin")?;

    assert_eq!(remote_type, RemoteType::GitHub);

    Ok(())
}

#[test]
fn test_git_utils_workdir() -> Result<()> {
    let temp_dir = TempDir::new()?;
    init_git_repo(temp_dir.path())?;

    let git_utils = GitUtils::open(temp_dir.path())?;
    let workdir = git_utils.workdir()?;

    // Canonicalize both paths to handle symlinks (e.g., /var -> /private/var on macOS)
    let expected = temp_dir.path().canonicalize()?;
    let actual = workdir.canonicalize()?;
    assert_eq!(actual, expected);

    Ok(())
}

#[test]
fn test_git_utils_multiple_branches() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let repo = init_git_repo(temp_dir.path())?;
    create_initial_commit(&repo)?;

    let git_utils = GitUtils::open(temp_dir.path())?;

    // Create multiple branches
    let branches = vec!["feature/branch-1", "feature/branch-2", "bugfix/issue-123"];

    for branch in &branches {
        git_utils.create_branch(branch)?;
        let current = git_utils.current_branch()?;
        assert_eq!(&current, branch);
    }

    Ok(())
}

#[test]
fn test_remote_type_display_name() {
    assert_eq!(RemoteType::GitHub.display_name(), "GitHub");
    assert_eq!(RemoteType::GitLab.display_name(), "GitLab");
    assert_eq!(RemoteType::Other.display_name(), "Unknown");
}

#[test]
fn test_remote_type_cli_tool() {
    assert_eq!(RemoteType::GitHub.cli_tool(), Some("gh"));
    assert_eq!(RemoteType::GitLab.cli_tool(), Some("glab"));
    assert_eq!(RemoteType::Other.cli_tool(), None);
}

#[test]
fn test_git_utils_with_modified_files() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let repo = init_git_repo(temp_dir.path())?;
    create_initial_commit(&repo)?;

    let git_utils = GitUtils::open(temp_dir.path())?;

    // Modify existing file
    fs::write(temp_dir.path().join("README.md"), "# Modified Content\n")?;

    // Check status
    assert!(!git_utils.is_clean()?);
    let changed = git_utils.changed_files()?;
    assert!(changed.contains(&"README.md".to_string()));

    Ok(())
}

#[test]
fn test_git_utils_staged_and_unstaged() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let repo = init_git_repo(temp_dir.path())?;
    create_initial_commit(&repo)?;

    // Create and stage file
    let new_file = temp_dir.path().join("staged.txt");
    fs::write(&new_file, "staged content")?;

    let mut index = repo.index()?;
    index.add_path(std::path::Path::new("staged.txt"))?;
    index.write()?;

    // Create unstaged file
    fs::write(temp_dir.path().join("unstaged.txt"), "unstaged content")?;

    let git_utils = GitUtils::open(temp_dir.path())?;
    let changed = git_utils.changed_files()?;

    // Should detect both staged and unstaged files
    assert!(changed.len() >= 2);
    assert!(changed.contains(&"staged.txt".to_string()));
    assert!(changed.contains(&"unstaged.txt".to_string()));

    Ok(())
}
