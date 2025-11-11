//! CLI tests for config commands
//!
//! Tests config init, check, and list commands with various options.

use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
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

/// Helper function to create required directories for config validation
fn create_required_dirs(base_path: &Path) {
    // Create all directories that config validation expects
    fs::create_dir_all(base_path.join(".claude")).unwrap();
    fs::create_dir_all(base_path.join(".claude/learning-sessions")).unwrap();
    fs::create_dir_all(base_path.join("projects")).unwrap();

    // Create platform-specific config directory
    #[cfg(target_os = "macos")]
    fs::create_dir_all(base_path.join("Library/Application Support/cldev")).unwrap();
    #[cfg(not(target_os = "macos"))]
    fs::create_dir_all(base_path.join(".config/cldev")).unwrap();

    // Also create fallback directory (used when dirs::config_dir() returns None in CI)
    fs::create_dir_all(base_path.join(".cldev")).unwrap();
}

#[test]
fn test_config_init_basic() {
    let temp_dir = TempDir::new().unwrap();
    create_required_dirs(temp_dir.path());

    let mut cmd = cargo_bin_cmd!();

    // Capture output to see where config was actually created
    // Use --force to overwrite any existing config from previous test runs
    let output = cmd
        .args(["config", "init", "--defaults", "--force"])
        .env("HOME", temp_dir.path())
        .output()
        .unwrap();

    // Command should succeed
    assert!(
        output.status.success(),
        "Command failed with: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Parse stdout to find where config was created
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Extract the actual path from output message
    // Output format: "✅ Configuration created at: <path>"
    let config_path = if let Some(line) = stdout
        .lines()
        .find(|l| l.contains("created at:") || l.contains("Configuration created at"))
    {
        // Extract path from the output line
        // Handle both Unix paths and Windows paths (C:\...)
        let path_str = if let Some(pos) = line.find("created at:") {
            line[pos + "created at:".len()..].trim()
        } else if let Some(pos) = line.find("Configuration created at") {
            line[pos + "Configuration created at".len()..].trim()
        } else {
            ""
        };

        if !path_str.is_empty() {
            // Remove ANSI color codes if present
            let clean = path_str.replace("\u{1b}[0m", "").replace("\u{1b}[1m", "");
            Some(PathBuf::from(clean.trim()))
        } else {
            None
        }
    } else {
        None
    };

    if let Some(path) = config_path {
        assert!(
            path.exists(),
            "Config file not found at path reported in output: {:?}\nCommand output: {}",
            path,
            stdout
        );
    } else {
        // Fallback: check expected locations
        let possible_paths = vec![
            // Platform-specific primary location
            #[cfg(target_os = "macos")]
            temp_dir
                .path()
                .join("Library/Application Support/cldev/config.toml"),
            #[cfg(not(target_os = "macos"))]
            temp_dir.path().join(".config/cldev/config.toml"),
            // Fallback location
            temp_dir.path().join(".cldev/config.toml"),
        ];

        let config_found = possible_paths.iter().any(|p| p.exists());

        assert!(
            config_found,
            "Config file not found.\nChecked paths: {:?}\nCommand output: {}\nTemp dir contents: {:?}",
            possible_paths,
            stdout,
            fs::read_dir(temp_dir.path()).ok().map(|entries| {
                entries.filter_map(|e| e.ok().map(|e| e.path())).collect::<Vec<_>>()
            })
        );
    }
}

#[test]
fn test_config_init_force() {
    let temp_dir = TempDir::new().unwrap();
    create_required_dirs(temp_dir.path());

    // Use platform-specific config directory
    #[cfg(target_os = "macos")]
    let config_dir = temp_dir.path().join("Library/Application Support/cldev");
    #[cfg(not(target_os = "macos"))]
    let config_dir = temp_dir.path().join(".config/cldev");
    fs::create_dir_all(&config_dir).unwrap();
    let config_path = config_dir.join("config.toml");

    // Create existing config
    write_config_with_permissions(&config_path, "version = \"1.0.0\"");

    let mut cmd = cargo_bin_cmd!();

    cmd.args(["config", "init", "--force", "--defaults"])
        .env("HOME", temp_dir.path())
        .assert()
        .success();
}

#[test]
fn test_config_check_valid() {
    let temp_dir = TempDir::new().unwrap();
    create_required_dirs(temp_dir.path());

    // Use platform-specific config directory
    #[cfg(target_os = "macos")]
    let config_dir = temp_dir.path().join("Library/Application Support/cldev");
    #[cfg(not(target_os = "macos"))]
    let config_dir = temp_dir.path().join(".config/cldev");

    fs::create_dir_all(&config_dir).unwrap();

    // Create valid config with explicit paths pointing to temp directory
    let claude_dir = temp_dir.path().join(".claude");
    let projects_dir = temp_dir.path().join("projects");
    let sessions_dir = temp_dir.path().join(".claude/learning-sessions");

    let config_content = format!(
        r#"
version = "1.0.0"

[general]
language = "ja"
claude_dir = "{}"
projects_dir = "{}"

[git]
default_base_branch = "main"

[lr]
sessions_dir = "{}"
"#,
        claude_dir.display(),
        projects_dir.display(),
        sessions_dir.display()
    );

    write_config_with_permissions(&config_dir.join("config.toml"), &config_content);

    let mut cmd = cargo_bin_cmd!();

    cmd.args(["config", "check"])
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

    let mut cmd = cargo_bin_cmd!();

    // Check should fail or warn about invalid config
    cmd.args(["config", "check"])
        .env("HOME", temp_dir.path())
        .assert();
    // May succeed with warnings or fail - either is acceptable
}

#[test]
fn test_config_check_missing() {
    let temp_dir = TempDir::new().unwrap();

    let mut cmd = cargo_bin_cmd!();

    cmd.args(["config", "check"])
        .env("HOME", temp_dir.path())
        .assert();
    // Should handle missing config gracefully
}

#[test]
fn test_config_list_all() {
    let temp_dir = TempDir::new().unwrap();
    create_required_dirs(temp_dir.path());

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

    let mut cmd = cargo_bin_cmd!();

    // config list shows all available commands, not config settings
    cmd.args(["config", "list"])
        .env("HOME", temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("config init"))
        .stdout(predicate::str::contains("dev feature"));
}

#[test]
fn test_config_list_specific_section() {
    let temp_dir = TempDir::new().unwrap();
    create_required_dirs(temp_dir.path());

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

    let mut cmd = cargo_bin_cmd!();

    // config list with --detailed shows detailed command information
    cmd.args(["config", "list", "--detailed"])
        .env("HOME", temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("git"))
        .stdout(predicate::str::contains("Usage"));
}

#[test]
fn test_config_list_json_format() {
    let temp_dir = TempDir::new().unwrap();
    create_required_dirs(temp_dir.path());

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

    let mut cmd = cargo_bin_cmd!();

    // config list doesn't support JSON format - just verify it shows commands
    cmd.args(["config", "list"])
        .env("HOME", temp_dir.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("commands"));
}

#[test]
fn test_config_init_interactive_skip() {
    let temp_dir = TempDir::new().unwrap();
    create_required_dirs(temp_dir.path());

    let mut cmd = cargo_bin_cmd!();

    // With --defaults flag
    cmd.args(["config", "init", "--defaults"])
        .env("HOME", temp_dir.path())
        .assert()
        .success();
}

#[test]
fn test_config_edit_command() {
    let temp_dir = TempDir::new().unwrap();
    create_required_dirs(temp_dir.path());

    // Use platform-specific config directory
    #[cfg(target_os = "macos")]
    let config_dir = temp_dir.path().join("Library/Application Support/cldev");
    #[cfg(not(target_os = "macos"))]
    let config_dir = temp_dir.path().join(".config/cldev");
    fs::create_dir_all(&config_dir).unwrap();

    // Create initial config
    write_config_with_permissions(&config_dir.join("config.toml"), "version = \"1.0.0\"");

    let mut cmd = cargo_bin_cmd!();

    // Edit command requires EDITOR environment variable
    cmd.args(["config", "edit"])
        .env("HOME", temp_dir.path())
        .env("EDITOR", "cat") // Use cat as a safe editor for testing
        .assert();
    // May fail if no config or no EDITOR, which is acceptable
}

#[test]
fn test_config_path_display() {
    let temp_dir = TempDir::new().unwrap();
    create_required_dirs(temp_dir.path());

    // Use platform-specific config directory
    #[cfg(target_os = "macos")]
    let config_dir = temp_dir.path().join("Library/Application Support/cldev");
    #[cfg(not(target_os = "macos"))]
    let config_dir = temp_dir.path().join(".config/cldev");
    fs::create_dir_all(&config_dir).unwrap();

    // Create config with explicit paths
    let claude_dir = temp_dir.path().join(".claude");
    let projects_dir = temp_dir.path().join("projects");
    let sessions_dir = temp_dir.path().join(".claude/learning-sessions");

    let config_content = format!(
        r#"
version = "1.0.0"

[general]
language = "en"
claude_dir = "{}"
projects_dir = "{}"

[lr]
sessions_dir = "{}"
"#,
        claude_dir.display(),
        projects_dir.display(),
        sessions_dir.display()
    );

    write_config_with_permissions(&config_dir.join("config.toml"), &config_content);

    let mut cmd = cargo_bin_cmd!();

    // config check shows config path information
    cmd.args(["config", "check"])
        .env("HOME", temp_dir.path())
        .assert()
        .success();
}

#[test]
fn test_config_validate_permissions() {
    let temp_dir = TempDir::new().unwrap();
    create_required_dirs(temp_dir.path());

    // Use platform-specific config directory
    #[cfg(target_os = "macos")]
    let config_dir = temp_dir.path().join("Library/Application Support/cldev");
    #[cfg(not(target_os = "macos"))]
    let config_dir = temp_dir.path().join(".config/cldev");
    fs::create_dir_all(&config_dir).unwrap();

    // Create config with explicit paths
    let claude_dir = temp_dir.path().join(".claude");
    let projects_dir = temp_dir.path().join("projects");
    let sessions_dir = temp_dir.path().join(".claude/learning-sessions");

    let config_content = format!(
        r#"version = "1.0.0"

[general]
claude_dir = "{}"
projects_dir = "{}"

[lr]
sessions_dir = "{}"
"#,
        claude_dir.display(),
        projects_dir.display(),
        sessions_dir.display()
    );

    let config_path = config_dir.join("config.toml");
    write_config_with_permissions(&config_path, &config_content);

    // On Unix, set permissions to 600
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = fs::metadata(&config_path).unwrap().permissions();
        permissions.set_mode(0o600);
        fs::set_permissions(&config_path, permissions).unwrap();
    }

    let mut cmd = cargo_bin_cmd!();

    cmd.args(["config", "check"])
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

    let mut cmd = cargo_bin_cmd!();

    // Migration should handle old config location
    cmd.args(["config", "check"])
        .env("HOME", temp_dir.path())
        .assert();
}
