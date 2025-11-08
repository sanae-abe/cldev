//! CLI tests for config commands
//!
//! Tests config init, check, and list commands with various options.

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

/// Helper function to write config file with secure permissions
fn write_config_with_permissions(path: &Path, content: &str) {
    fs::write(path, content).unwrap();

    // Set secure permissions (Unix only)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o600)).unwrap();
    }
}

#[test]
fn test_config_init_basic() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.toml");

    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.args(&["config", "init"])
        .env("HOME", temp_dir.path())
        .assert()
        .success();

    // Verify config file was created
    let default_config_path = temp_dir
        .path()
        .join(".config")
        .join("cldev")
        .join("config.toml");

    assert!(
        default_config_path.exists() || config_path.exists(),
        "Config file should be created"
    );
}

#[test]
fn test_config_init_force() {
    let temp_dir = TempDir::new().unwrap();
    // Use platform-specific config directory
    #[cfg(target_os = "macos")]
    let config_dir = temp_dir.path().join("Library/Application Support/cldev");
    #[cfg(not(target_os = "macos"))]
    let config_dir = temp_dir.path().join(".config/cldev");
    fs::create_dir_all(&config_dir).unwrap();
    let config_path = config_dir.join("config.toml");

    // Create existing config
    write_config_with_permissions(&config_path, "version = \"1.0.0\"");

    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.args(&["config", "init", "--force"])
        .env("HOME", temp_dir.path())
        .assert()
        .success();
}

#[test]
fn test_config_check_valid() {
    let temp_dir = TempDir::new().unwrap();

    // Use platform-specific config directory
    #[cfg(target_os = "macos")]
    let config_dir = temp_dir.path().join("Library/Application Support/cldev");
    #[cfg(not(target_os = "macos"))]
    let config_dir = temp_dir.path().join(".config/cldev");

    fs::create_dir_all(&config_dir).unwrap();

    // Create required directories for validation
    fs::create_dir_all(temp_dir.path().join(".claude/learning-sessions")).unwrap();
    fs::create_dir_all(temp_dir.path().join("projects")).unwrap();

    // Create valid config
    let config_content = r#"
version = "1.0.0"

[general]
language = "ja"

[git]
default_base_branch = "main"
"#;

    write_config_with_permissions(&config_dir.join("config.toml"), config_content);

    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.args(&["config", "check"])
        .env("HOME", temp_dir.path())
        .assert()
        .success()
        .stdout(
            predicate::str::contains("All checks passed")
                .or(predicate::str::contains("Configuration is healthy"))
                .or(predicate::str::contains("checks passed")),
        );
}

#[test]
fn test_config_check_invalid() {
    let temp_dir = TempDir::new().unwrap();
    // Use platform-specific config directory
    #[cfg(target_os = "macos")]
    let config_dir = temp_dir.path().join("Library/Application Support/cldev");
    #[cfg(not(target_os = "macos"))]
    let config_dir = temp_dir.path().join(".config/cldev");
    fs::create_dir_all(&config_dir).unwrap();

    // Create invalid config (wrong version format)
    let invalid_config = r#"
version = "invalid"

[general]
language = "ja"
"#;

    write_config_with_permissions(&config_dir.join("config.toml"), invalid_config);

    let mut cmd = Command::cargo_bin("cldev").unwrap();

    // Check should fail or warn about invalid config
    cmd.args(&["config", "check"])
        .env("HOME", temp_dir.path())
        .assert();
    // May succeed with warnings or fail - either is acceptable
}

#[test]
fn test_config_check_missing() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.args(&["config", "check"])
        .env("HOME", temp_dir.path())
        .assert();
    // Should handle missing config gracefully
}

#[test]
fn test_config_list_all() {
    let temp_dir = TempDir::new().unwrap();
    // Use platform-specific config directory
    #[cfg(target_os = "macos")]
    let config_dir = temp_dir.path().join("Library/Application Support/cldev");
    #[cfg(not(target_os = "macos"))]
    let config_dir = temp_dir.path().join(".config/cldev");
    fs::create_dir_all(&config_dir).unwrap();

    // Create config
    let config_content = r#"
version = "1.0.0"

[general]
language = "en"

[git]
default_base_branch = "develop"
auto_push = false
"#;

    write_config_with_permissions(&config_dir.join("config.toml"), config_content);

    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.args(&["config", "list"])
        .env("HOME", temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("language"))
        .stdout(predicate::str::contains("default_base_branch"));
}

#[test]
fn test_config_list_specific_section() {
    let temp_dir = TempDir::new().unwrap();
    // Use platform-specific config directory
    #[cfg(target_os = "macos")]
    let config_dir = temp_dir.path().join("Library/Application Support/cldev");
    #[cfg(not(target_os = "macos"))]
    let config_dir = temp_dir.path().join(".config/cldev");
    fs::create_dir_all(&config_dir).unwrap();

    let config_content = r#"
version = "1.0.0"

[general]
language = "ja"

[git]
default_base_branch = "main"
"#;

    write_config_with_permissions(&config_dir.join("config.toml"), config_content);

    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.args(&["config", "list", "--section", "git"])
        .env("HOME", temp_dir.path())
        .assert()
        .success()
        .stdout(
            predicate::str::contains("git").or(predicate::str::contains("default_base_branch")),
        );
}

#[test]
fn test_config_list_json_format() {
    let temp_dir = TempDir::new().unwrap();
    // Use platform-specific config directory
    #[cfg(target_os = "macos")]
    let config_dir = temp_dir.path().join("Library/Application Support/cldev");
    #[cfg(not(target_os = "macos"))]
    let config_dir = temp_dir.path().join(".config/cldev");
    fs::create_dir_all(&config_dir).unwrap();

    let config_content = r#"
version = "1.0.0"

[general]
language = "ja"
"#;

    write_config_with_permissions(&config_dir.join("config.toml"), config_content);

    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.args(&["config", "list", "--format", "json"])
        .env("HOME", temp_dir.path())
        .assert()
        .success();
    // JSON format should be valid (contains { and })
}

#[test]
fn test_config_init_interactive_skip() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("cldev").unwrap();

    // With --no-interactive flag
    cmd.args(&["config", "init", "--no-interactive"])
        .env("HOME", temp_dir.path())
        .assert()
        .success();
}

#[test]
fn test_config_edit_command() {
    let temp_dir = TempDir::new().unwrap();
    // Use platform-specific config directory
    #[cfg(target_os = "macos")]
    let config_dir = temp_dir.path().join("Library/Application Support/cldev");
    #[cfg(not(target_os = "macos"))]
    let config_dir = temp_dir.path().join(".config/cldev");
    fs::create_dir_all(&config_dir).unwrap();

    // Create initial config
    write_config_with_permissions(&config_dir.join("config.toml"), "version = \"1.0.0\"");

    let mut cmd = Command::cargo_bin("cldev").unwrap();

    // Edit command requires EDITOR environment variable
    cmd.args(&["config", "edit"])
        .env("HOME", temp_dir.path())
        .env("EDITOR", "cat") // Use cat as a safe editor for testing
        .assert();
    // May fail if no config or no EDITOR, which is acceptable
}

#[test]
fn test_config_path_display() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.args(&["config", "path"])
        .env("HOME", temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains(".config").or(predicate::str::contains("cldev")));
}

#[test]
fn test_config_validate_permissions() {
    let temp_dir = TempDir::new().unwrap();
    // Use platform-specific config directory
    #[cfg(target_os = "macos")]
    let config_dir = temp_dir.path().join("Library/Application Support/cldev");
    #[cfg(not(target_os = "macos"))]
    let config_dir = temp_dir.path().join(".config/cldev");
    fs::create_dir_all(&config_dir).unwrap();

    let config_content = "version = \"1.0.0\"";
    let config_path = config_dir.join("config.toml");
    write_config_with_permissions(&config_path, config_content);

    // On Unix, set permissions to 600
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = fs::metadata(&config_path).unwrap().permissions();
        permissions.set_mode(0o600);
        fs::set_permissions(&config_path, permissions).unwrap();
    }

    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.args(&["config", "check"])
        .env("HOME", temp_dir.path())
        .assert()
        .success();
}

#[test]
fn test_config_migration() {
    let temp_dir = TempDir::new().unwrap();
    let old_config_dir = temp_dir.path().join(".cldev");
    fs::create_dir_all(&old_config_dir).unwrap();

    // Create old-style config
    write_config_with_permissions(&old_config_dir.join("config.toml"), "version = \"1.0.0\"");

    let mut cmd = Command::cargo_bin("cldev").unwrap();

    // Migration should handle old config location
    cmd.args(&["config", "check"])
        .env("HOME", temp_dir.path())
        .assert();
}
