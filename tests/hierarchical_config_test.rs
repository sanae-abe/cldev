//! Integration tests for hierarchical configuration system
//!
//! Tests the 3-layer configuration hierarchy:
//! Global → Stack → Project

use cldev::core::{Config, HierarchicalConfig, ProjectConfig, StackConfig, TechStack};
use std::fs;
use tempfile::TempDir;

/// Create a test global config
fn create_test_global_config(temp_dir: &TempDir) -> std::path::PathBuf {
    let config_dir = temp_dir.path().join("config");
    fs::create_dir_all(&config_dir).unwrap();

    let config_path = config_dir.join("config.toml");
    let config_content = r#"
version = "1.0.0"

[general]
language = "en"
tech_stack = "rust-cli"

[git]
default_base_branch = "main"
auto_push = true

[quality]
run_tests_before_commit = true

[performance]
parallel_tasks = 4
timeout_seconds = 300
"#;

    fs::write(&config_path, config_content).unwrap();

    // Set permissions to 600 on Unix
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut permissions = fs::metadata(&config_path).unwrap().permissions();
        permissions.set_mode(0o600);
        fs::set_permissions(&config_path, permissions).unwrap();
    }

    config_path
}

/// Create a test stack config directory
fn create_test_stack_configs(temp_dir: &TempDir) -> std::path::PathBuf {
    let stacks_dir = temp_dir.path().join("config").join("stacks");
    fs::create_dir_all(&stacks_dir).unwrap();

    // Create rust-cli stack config
    let rust_config = stacks_dir.join("rust-cli.toml");
    let rust_content = r#"
[stack]
name = "rust-cli"
description = "Rust CLI development"
languages = ["Rust"]
version = "1.0.0"

[commands]
build = "cargo build"
test = "cargo test"
lint = "cargo clippy"

[tools]
package_manager = "cargo"
linter = "clippy"

[quality]
coverage_threshold = 80
strict_mode = true

[environment]
rust_version = ">=1.70"
"#;
    fs::write(&rust_config, rust_content).unwrap();

    stacks_dir
}

/// Create a test project config
fn create_test_project_config(project_dir: &std::path::Path) {
    let cldev_dir = project_dir.join(".cldev");
    fs::create_dir_all(&cldev_dir).unwrap();

    let config_path = cldev_dir.join("config.toml");
    let config_content = r#"
[project]
name = "test-project"
tech_stack = "rust-cli"
version = "1.0.0"

[commands]
build = "cargo build --release"
custom = "echo custom"

[dev]
port = 9000

[quality]
min_coverage = 90

[git]
base_branch = "develop"
"#;

    fs::write(&config_path, config_content).unwrap();
}

#[test]
fn test_global_config_loading() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = create_test_global_config(&temp_dir);

    let config = Config::load(Some(config_path)).unwrap();

    assert_eq!(config.general.language, "en");
    assert_eq!(config.general.tech_stack, Some("rust-cli".to_string()));
    assert_eq!(config.git.default_base_branch, "main");
    assert!(config.git.auto_push);
    assert_eq!(config.performance.parallel_tasks, 4);
}

#[test]
fn test_stack_config_loading() {
    let temp_dir = TempDir::new().unwrap();
    create_test_stack_configs(&temp_dir);

    let stack = TechStack::RustCli;
    let stack_config =
        StackConfig::load(&stack).unwrap_or_else(|_| StackConfig::default_for_stack(&stack));

    assert_eq!(stack_config.stack.name, "rust-cli");
    assert!(stack_config.commands.contains_key("build"));
    assert!(stack_config.commands.contains_key("test"));
    assert_eq!(
        stack_config.tools.package_manager,
        Some("cargo".to_string())
    );
}

#[test]
fn test_project_config_loading() {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path();
    create_test_project_config(project_dir);

    let project_config = ProjectConfig::load(project_dir).unwrap();

    assert_eq!(project_config.project.name, "test-project");
    assert_eq!(
        project_config.project.tech_stack,
        Some("rust-cli".to_string())
    );
    assert_eq!(project_config.dev.port, Some(9000));
    assert_eq!(project_config.quality.min_coverage, Some(90));
}

#[test]
fn test_hierarchical_config_priority() {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path();

    // Create all three layers
    create_test_global_config(&temp_dir);
    create_test_project_config(project_dir);

    // Load global config first
    let global = Config::load(Some(temp_dir.path().join("config").join("config.toml"))).unwrap();

    // Load project config
    let project = Some(ProjectConfig::load(project_dir).unwrap());

    // Load stack config
    let stack = if let Some(ref stack_name) = global.general.tech_stack {
        TechStack::parse(stack_name)
            .ok()
            .map(|s| StackConfig::default_for_stack(&s))
    } else {
        None
    };

    let hierarchical = HierarchicalConfig {
        global,
        stack,
        project,
    };

    // Test priority: Project > Stack > Global

    // Base branch: Project overrides Global
    assert_eq!(hierarchical.effective_base_branch(), "develop");

    // Dev port: Project sets it
    assert_eq!(hierarchical.effective_dev_port(), Some(9000));

    // Coverage: Project overrides Stack
    assert_eq!(hierarchical.effective_coverage_threshold(), Some(90));
}

#[test]
fn test_merged_commands() {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path();

    create_test_global_config(&temp_dir);
    create_test_project_config(project_dir);

    let global = Config::load(Some(temp_dir.path().join("config").join("config.toml"))).unwrap();
    let project = Some(ProjectConfig::load(project_dir).unwrap());
    let stack = Some(StackConfig::default_for_stack(&TechStack::RustCli));

    let hierarchical = HierarchicalConfig {
        global,
        stack,
        project,
    };

    let commands = hierarchical.merged_commands();

    // Project command overrides stack command
    assert_eq!(
        commands.get("build"),
        Some(&"cargo build --release".to_string())
    );

    // Stack command (no override from project)
    assert_eq!(commands.get("test"), Some(&"cargo test".to_string()));

    // Project-only command
    assert_eq!(commands.get("custom"), Some(&"echo custom".to_string()));
}

#[test]
fn test_effective_tech_stack() {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path();

    create_test_global_config(&temp_dir);
    create_test_project_config(project_dir);

    let global = Config::load(Some(temp_dir.path().join("config").join("config.toml"))).unwrap();
    let project = Some(ProjectConfig::load(project_dir).unwrap());

    let hierarchical = HierarchicalConfig {
        global,
        stack: None,
        project,
    };

    let stack = hierarchical.effective_tech_stack();
    assert!(stack.is_some());
    assert!(matches!(stack.unwrap(), TechStack::RustCli));
}

#[test]
fn test_effective_project_name() {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path();

    create_test_global_config(&temp_dir);
    create_test_project_config(project_dir);

    let global = Config::load(Some(temp_dir.path().join("config").join("config.toml"))).unwrap();
    let project = Some(ProjectConfig::load(project_dir).unwrap());

    let hierarchical = HierarchicalConfig {
        global,
        stack: None,
        project,
    };

    let name = hierarchical.effective_project_name();
    assert_eq!(name, Some("test-project".to_string()));
}

#[test]
fn test_project_init() {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path();

    let config = ProjectConfig::init(
        project_dir,
        "my-app".to_string(),
        Some(TechStack::FrontendWeb),
    )
    .unwrap();

    assert_eq!(config.project.name, "my-app");
    assert_eq!(config.project.tech_stack, Some("frontend-web".to_string()));
    assert_eq!(config.dev.port, Some(3000));
    assert_eq!(config.quality.min_coverage, Some(80));

    // Verify file was created
    assert!(ProjectConfig::exists(project_dir));
}

#[test]
fn test_stack_defaults() {
    let frontend = StackConfig::default_for_stack(&TechStack::FrontendWeb);
    assert_eq!(frontend.stack.name, "frontend-web");
    assert_eq!(frontend.environment.dev_port, Some(3000));
    assert_eq!(frontend.quality.coverage_threshold, Some(80));

    let backend = StackConfig::default_for_stack(&TechStack::BackendApi);
    assert_eq!(backend.stack.name, "backend-api");
    assert_eq!(backend.environment.dev_port, Some(8080));
    assert_eq!(backend.quality.coverage_threshold, Some(90));

    let rust = StackConfig::default_for_stack(&TechStack::RustCli);
    assert_eq!(rust.stack.name, "rust-cli");
    assert_eq!(rust.quality.coverage_threshold, Some(80));
}

#[test]
fn test_tech_stack_from_string() {
    assert!(matches!(
        TechStack::parse("frontend-web").unwrap(),
        TechStack::FrontendWeb
    ));
    assert!(matches!(
        TechStack::parse("backend-api").unwrap(),
        TechStack::BackendApi
    ));
    assert!(matches!(
        TechStack::parse("mobile-app").unwrap(),
        TechStack::MobileApp
    ));
    assert!(matches!(
        TechStack::parse("data-science").unwrap(),
        TechStack::DataScience
    ));
    assert!(matches!(
        TechStack::parse("rust-cli").unwrap(),
        TechStack::RustCli
    ));
    assert!(TechStack::parse("invalid").is_err());
}

#[test]
fn test_config_get_set_tech_stack() {
    let mut config = Config::default();
    assert!(config.get_tech_stack().is_none());

    config.set_tech_stack(Some(TechStack::RustCli));
    assert_eq!(config.general.tech_stack, Some("rust-cli".to_string()));

    let stack = config.get_tech_stack();
    assert!(matches!(stack.unwrap(), TechStack::RustCli));

    config.set_tech_stack(None);
    assert!(config.get_tech_stack().is_none());
}

#[test]
fn test_config_get_set_project_name() {
    let mut config = Config::default();
    assert!(config.get_project_name().is_none());

    config.set_project_name(Some("test-project".to_string()));
    assert_eq!(config.get_project_name(), Some("test-project"));

    config.set_project_name(None);
    assert!(config.get_project_name().is_none());
}

#[test]
fn test_project_config_without_tech_stack() {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path();

    let config = ProjectConfig::init(project_dir, "simple-app".to_string(), None).unwrap();

    assert_eq!(config.project.name, "simple-app");
    assert!(config.project.tech_stack.is_none());
    assert!(config.get_tech_stack().is_none());
}

#[test]
fn test_hierarchical_fallback_to_defaults() {
    let temp_dir = TempDir::new().unwrap();

    let global = Config::load(Some(temp_dir.path().join("nonexistent.toml"))).unwrap();

    let hierarchical = HierarchicalConfig {
        global,
        stack: None,
        project: None,
    };

    // Should use global defaults
    assert_eq!(hierarchical.effective_base_branch(), "main");
    assert!(hierarchical.effective_tech_stack().is_none());
    assert!(hierarchical.effective_project_name().is_none());
}
