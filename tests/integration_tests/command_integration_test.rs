//! Integration tests for command execution flows
//!
//! Tests the interaction between commands and the configuration system,
//! ensuring commands properly utilize configuration settings.

use cldev::core::config::Config;
use cldev::core::error::Result;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_command_config_integration() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config_path = temp_dir.path().join("config.toml");

    // Create config with specific settings
    let mut config = Config::default();
    config.dev.auto_create_branch = true;
    config.dev.branch_prefix = "feature".to_string();
    config.git.auto_push = false;
    config.save(Some(config_path.clone()))?;

    // Load config
    let loaded = Config::load(Some(config_path))?;

    // Verify command-related settings
    assert!(loaded.dev.auto_create_branch);
    assert_eq!(loaded.dev.branch_prefix, "feature");
    assert!(!loaded.git.auto_push);

    Ok(())
}

#[test]
fn test_quality_command_config() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config_path = temp_dir.path().join("config.toml");

    // Create config with quality settings
    let mut config = Config::default();
    config.quality.auto_fix = true;
    config.quality.run_tests_before_commit = true;
    config.save(Some(config_path.clone()))?;

    // Load and verify
    let loaded = Config::load(Some(config_path))?;
    assert!(loaded.quality.auto_fix);
    assert!(loaded.quality.run_tests_before_commit);

    Ok(())
}

#[test]
fn test_performance_tuning_integration() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config_path = temp_dir.path().join("config.toml");

    // Test different performance configurations
    let test_cases = vec![(2, 60), (4, 300), (8, 600), (16, 1200)];

    for (tasks, timeout) in test_cases {
        let mut config = Config::default();
        config.performance.parallel_tasks = tasks;
        config.performance.timeout_seconds = timeout;
        config.save(Some(config_path.clone()))?;

        let loaded = Config::load(Some(config_path.clone()))?;
        assert_eq!(loaded.performance.parallel_tasks, tasks);
        assert_eq!(loaded.performance.timeout_seconds, timeout);
    }

    Ok(())
}

#[test]
fn test_ui_preferences_integration() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config_path = temp_dir.path().join("config.toml");

    // Test UI configurations
    let mut config = Config::default();
    config.ui.color = false;
    config.ui.emoji = false;
    config.ui.progress_bar = false;
    config.save(Some(config_path.clone()))?;

    let loaded = Config::load(Some(config_path))?;
    assert!(!loaded.ui.color);
    assert!(!loaded.ui.emoji);
    assert!(!loaded.ui.progress_bar);

    Ok(())
}

#[test]
fn test_learning_record_integration() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config_path = temp_dir.path().join("config.toml");
    let sessions_dir = temp_dir.path().join("sessions");

    // Create config with learning record settings
    let mut config = Config::default();
    config.lr.sessions_dir = sessions_dir.clone();
    config.lr.auto_save = true;
    config.lr.default_tags = vec!["test".to_string(), "integration".to_string()];
    config.save(Some(config_path.clone()))?;

    // Load and verify
    let loaded = Config::load(Some(config_path))?;
    assert_eq!(loaded.lr.sessions_dir, sessions_dir);
    assert!(loaded.lr.auto_save);
    assert_eq!(loaded.lr.default_tags, vec!["test", "integration"]);

    Ok(())
}

#[test]
fn test_git_cli_detection_integration() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config_path = temp_dir.path().join("config.toml");

    // Test GitHub CLI configuration
    let mut config = Config::default();
    config.git.github_cli = true;
    config.git.gitlab_cli = false;
    config.save(Some(config_path.clone()))?;

    let loaded = Config::load(Some(config_path.clone()))?;
    assert!(loaded.git.github_cli);
    assert!(!loaded.git.gitlab_cli);

    // Test GitLab CLI configuration
    let mut config2 = Config::default();
    config2.git.github_cli = false;
    config2.git.gitlab_cli = true;
    config2.save(Some(config_path.clone()))?;

    let loaded2 = Config::load(Some(config_path))?;
    assert!(!loaded2.git.github_cli);
    assert!(loaded2.git.gitlab_cli);

    Ok(())
}

#[test]
fn test_branch_naming_integration() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config_path = temp_dir.path().join("config.toml");

    // Test different branch prefix configurations
    let prefixes = vec!["feature", "feat", "task", "story", "work"];

    for prefix in prefixes {
        let mut config = Config::default();
        config.dev.branch_prefix = prefix.to_string();
        config.save(Some(config_path.clone()))?;

        let loaded = Config::load(Some(config_path.clone()))?;
        assert_eq!(loaded.dev.branch_prefix, prefix);
    }

    Ok(())
}

#[test]
fn test_session_recording_toggle() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config_path = temp_dir.path().join("config.toml");

    // Test with session recording enabled
    let mut config = Config::default();
    config.dev.session_recording = true;
    config.save(Some(config_path.clone()))?;

    let loaded = Config::load(Some(config_path.clone()))?;
    assert!(loaded.dev.session_recording);

    // Test with session recording disabled
    let mut config2 = Config::default();
    config2.dev.session_recording = false;
    config2.save(Some(config_path.clone()))?;

    let loaded2 = Config::load(Some(config_path))?;
    assert!(!loaded2.dev.session_recording);

    Ok(())
}

#[test]
fn test_config_language_switching() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config_path = temp_dir.path().join("config.toml");

    // Test language configurations
    for lang in &["ja", "en"] {
        let mut config = Config::default();
        config.general.language = lang.to_string();
        config.save(Some(config_path.clone()))?;

        let loaded = Config::load(Some(config_path.clone()))?;
        assert_eq!(loaded.general.language, *lang);
    }

    Ok(())
}

#[test]
fn test_multiple_config_updates() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config_path = temp_dir.path().join("config.toml");

    // Initial save
    let config = Config::default();
    config.save(Some(config_path.clone()))?;

    // Multiple updates
    for i in 1..=5 {
        let mut loaded = (*Config::load(Some(config_path.clone()))?).clone();
        loaded.performance.parallel_tasks = i * 2;
        loaded.save(Some(config_path.clone()))?;

        let verified = Config::load(Some(config_path.clone()))?;
        assert_eq!(verified.performance.parallel_tasks, i * 2);
    }

    Ok(())
}

#[test]
fn test_config_directory_creation() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let deep_path = temp_dir
        .path()
        .join("level1")
        .join("level2")
        .join("level3")
        .join("config.toml");

    // Save should create all parent directories
    let config = Config::default();
    config.save(Some(deep_path.clone()))?;

    // Verify directory structure was created
    assert!(deep_path.exists());
    assert!(deep_path.parent().unwrap().exists());

    Ok(())
}

#[test]
fn test_config_toml_formatting() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config_path = temp_dir.path().join("config.toml");

    // Save config
    let config = Config::default();
    config.save(Some(config_path.clone()))?;

    // Read raw TOML content
    let content = fs::read_to_string(&config_path)?;

    // Verify TOML structure
    assert!(content.contains("version = "));
    assert!(content.contains("[general]"));
    assert!(content.contains("[git]"));
    assert!(content.contains("[quality]"));
    assert!(content.contains("[dev]"));
    assert!(content.contains("[lr]"));
    assert!(content.contains("[ui]"));
    assert!(content.contains("[performance]"));

    Ok(())
}
