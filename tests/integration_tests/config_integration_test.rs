//! Integration tests for configuration management
//!
//! Tests the full configuration lifecycle including loading, saving, merging,
//! and hierarchical configuration resolution.

use cldev::core::config::{Config, HierarchicalConfig};
use cldev::core::error::Result;
use cldev::core::project_config::ProjectConfig;
use cldev::core::stack_config::{StackConfig, TechStack};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_config_full_lifecycle() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config_path = temp_dir.path().join("config.toml");

    // Create a new config
    let mut config = Config::default();
    config.general.language = "en".to_string();
    config.git.default_base_branch = "develop".to_string();
    config.dev.auto_create_branch = false;
    config.performance.parallel_tasks = 8;

    // Save config
    config.save(Some(config_path.clone()))?;

    // Verify file exists
    assert!(config_path.exists());

    // Load config back
    let loaded = Config::load(Some(config_path.clone()))?;
    assert_eq!(loaded.general.language, "en");
    assert_eq!(loaded.git.default_base_branch, "develop");
    assert!(!loaded.dev.auto_create_branch);
    assert_eq!(loaded.performance.parallel_tasks, 8);

    // Modify and save again
    let mut modified = (*loaded).clone();
    modified.ui.color = false;
    modified.save(Some(config_path.clone()))?;

    // Reload and verify
    let reloaded = Config::load(Some(config_path))?;
    assert!(!reloaded.ui.color);
    assert_eq!(reloaded.general.language, "en");

    Ok(())
}

#[test]
fn test_config_defaults() -> Result<()> {
    let config = Config::default();

    // Verify default values
    assert_eq!(config.general.language, "ja");
    assert_eq!(config.git.default_base_branch, "main");
    assert!(config.git.github_cli);
    assert!(!config.git.gitlab_cli);
    assert!(config.dev.auto_create_branch);
    assert_eq!(config.dev.branch_prefix, "feature");
    assert!(config.ui.color);
    assert!(config.ui.emoji);
    assert_eq!(config.performance.parallel_tasks, 4);
    assert_eq!(config.performance.timeout_seconds, 300);

    Ok(())
}

#[test]
fn test_config_partial_override() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config_path = temp_dir.path().join("config.toml");

    // Write partial config (only override some fields)
    let partial_toml = r#"
        version = "1.0.0"

        [general]
        language = "en"

        [git]
        auto_push = false
    "#;

    fs::write(&config_path, partial_toml)?;

    // Set proper permissions (600) for security check
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&config_path)?.permissions();
        perms.set_mode(0o600);
        fs::set_permissions(&config_path, perms)?;
    }

    // Load config
    let config = Config::load(Some(config_path))?;

    // Overridden fields
    assert_eq!(config.general.language, "en");
    assert!(!config.git.auto_push);

    // Default fields should still be set
    assert_eq!(config.git.default_base_branch, "main");
    assert!(config.dev.auto_create_branch);
    assert_eq!(config.performance.parallel_tasks, 4);

    Ok(())
}

#[test]
#[cfg(unix)]
fn test_config_file_permissions() -> Result<()> {
    use std::os::unix::fs::PermissionsExt;

    let temp_dir = TempDir::new()?;
    let config_path = temp_dir.path().join("config.toml");

    // Save config
    let config = Config::default();
    config.save(Some(config_path.clone()))?;

    // Check permissions are 600
    let metadata = fs::metadata(&config_path)?;
    let mode = metadata.permissions().mode();
    assert_eq!(
        mode & 0o777,
        0o600,
        "Config file should have 600 permissions"
    );

    Ok(())
}

#[test]
fn test_config_tech_stack_getters_setters() -> Result<()> {
    let mut config = Config::default();

    // Initially no tech stack
    assert!(config.get_tech_stack().is_none());

    // Set tech stack
    config.set_tech_stack(Some(TechStack::FrontendWeb));
    assert_eq!(config.get_tech_stack(), Some(TechStack::FrontendWeb));

    // Change tech stack
    config.set_tech_stack(Some(TechStack::BackendApi));
    assert_eq!(config.get_tech_stack(), Some(TechStack::BackendApi));

    // Clear tech stack
    config.set_tech_stack(None);
    assert!(config.get_tech_stack().is_none());

    Ok(())
}

#[test]
fn test_config_project_name_getters_setters() -> Result<()> {
    let mut config = Config::default();

    // Initially no project name
    assert!(config.get_project_name().is_none());

    // Set project name
    config.set_project_name(Some("my-project".to_string()));
    assert_eq!(config.get_project_name(), Some("my-project"));

    // Change project name
    config.set_project_name(Some("another-project".to_string()));
    assert_eq!(config.get_project_name(), Some("another-project"));

    // Clear project name
    config.set_project_name(None);
    assert!(config.get_project_name().is_none());

    Ok(())
}

#[test]
fn test_hierarchical_config_global_only() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let config_path = temp_dir.path().join("config.toml");

    // Create global config
    let mut global = Config::default();
    global.general.language = "en".to_string();
    global.git.default_base_branch = "main".to_string();
    global.save(Some(config_path))?;

    // Load hierarchical config (no project root)
    let hierarchical = HierarchicalConfig {
        global: Config::load(Some(temp_dir.path().join("config.toml")))?,
        stack: None,
        project: None,
    };

    // Verify effective values come from global
    assert_eq!(hierarchical.effective_base_branch(), "main");
    assert!(hierarchical.effective_tech_stack().is_none());
    assert!(hierarchical.effective_project_name().is_none());

    Ok(())
}

#[test]
fn test_hierarchical_config_with_project() -> Result<()> {
    let temp_dir = TempDir::new()?;

    // Create global config
    let global_config_path = temp_dir.path().join("global.toml");
    let mut global = Config::default();
    global.git.default_base_branch = "main".to_string();
    global.save(Some(global_config_path.clone()))?;

    // Create project directory and config
    let project_dir = temp_dir.path().join("project");
    fs::create_dir_all(project_dir.join(".cldev"))?;

    let project_config_toml = r#"
        version = "1.0.0"

        [project]
        name = "test-project"
        description = "Test project"

        [git]
        base_branch = "develop"

        [dev]
        port = 3000
    "#;

    fs::write(project_dir.join(".cldev/config.toml"), project_config_toml)?;

    // Load hierarchical config
    let hierarchical = HierarchicalConfig {
        global: Config::load(Some(global_config_path))?,
        stack: None,
        project: Some(ProjectConfig::load(&project_dir)?),
    };

    // Project config should override global
    assert_eq!(hierarchical.effective_base_branch(), "develop");
    assert_eq!(
        hierarchical.effective_project_name(),
        Some("test-project".to_string())
    );
    assert_eq!(hierarchical.effective_dev_port(), Some(3000));

    Ok(())
}

#[test]
fn test_hierarchical_config_priority() -> Result<()> {
    let temp_dir = TempDir::new()?;

    // Create global config
    let global_config_path = temp_dir.path().join("global.toml");
    let mut global = Config::default();
    global.git.default_base_branch = "main".to_string();
    global.save(Some(global_config_path.clone()))?;

    // Create stack config directory
    let stack_dir = temp_dir.path().join("stacks");
    fs::create_dir_all(&stack_dir)?;

    // Create project config
    let project_dir = temp_dir.path().join("project");
    fs::create_dir_all(project_dir.join(".cldev"))?;

    let project_config_toml = r#"
        version = "1.0.0"

        [project]
        name = "priority-test"

        [git]
        base_branch = "project-branch"

        [dev]
        port = 3000

        [quality]
        min_coverage = 90
    "#;

    fs::write(project_dir.join(".cldev/config.toml"), project_config_toml)?;

    // Load all configs
    let hierarchical = HierarchicalConfig {
        global: Config::load(Some(global_config_path))?,
        stack: None,
        project: Some(ProjectConfig::load(&project_dir)?),
    };

    // Verify priority: Project > Stack > Global
    assert_eq!(hierarchical.effective_base_branch(), "project-branch"); // From project
    assert_eq!(hierarchical.effective_dev_port(), Some(3000)); // From project
    assert_eq!(hierarchical.effective_coverage_threshold(), Some(90)); // From project

    Ok(())
}

#[test]
fn test_config_version_compatibility() -> Result<()> {
    use cldev::core::config::{validate_version, CONFIG_VERSION};

    // Same version should be compatible
    assert!(validate_version(CONFIG_VERSION).is_ok());

    // Same major, lower minor should be compatible
    assert!(validate_version("1.0.0").is_ok());

    // Different major version should fail
    assert!(validate_version("2.0.0").is_err());

    // Invalid format should fail
    assert!(validate_version("1.0").is_err());
    assert!(validate_version("invalid").is_err());

    Ok(())
}

#[test]
fn test_merged_commands() -> Result<()> {
    let temp_dir = TempDir::new()?;

    // Create project config with custom commands
    let project_dir = temp_dir.path().join("project");
    fs::create_dir_all(project_dir.join(".cldev"))?;

    let project_config_toml = r#"
        version = "1.0.0"

        [project]
        name = "cmd-test"

        [commands]
        dev = "npm run dev"
        test = "npm test"
        build = "npm run build"
    "#;

    fs::write(project_dir.join(".cldev/config.toml"), project_config_toml)?;

    // Load hierarchical config
    let hierarchical = HierarchicalConfig {
        global: Config::load(None)?,
        stack: None,
        project: Some(ProjectConfig::load(&project_dir)?),
    };

    // Get merged commands
    let commands = hierarchical.merged_commands();

    // Verify project commands are present
    assert!(commands.contains_key("dev"));
    assert!(commands.contains_key("test"));
    assert!(commands.contains_key("build"));
    assert_eq!(commands.get("dev"), Some(&"npm run dev".to_string()));

    Ok(())
}
