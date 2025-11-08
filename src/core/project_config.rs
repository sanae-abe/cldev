//! Project-specific configuration management
//!
//! This module manages project-level settings stored in `.cldev/config.toml`
//! within each project directory. Project configs have the highest priority
//! in the configuration hierarchy.
//!
//! # Configuration Hierarchy
//!
//! Project configs override both global and stack configs:
//! Global → Stack → **Project** (highest priority)
//!
//! # Project Directory Structure
//!
//! ```text
//! project_root/
//! ├── .cldev/
//! │   ├── config.toml       # Project configuration
//! │   ├── sessions/         # Learning sessions
//! │   └── cache/            # Build cache
//! └── src/
//! ```

#![allow(dead_code)]

use crate::core::error::{CldevError, Result};
use crate::core::stack_config::TechStack;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Project-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[allow(dead_code)]
pub struct ProjectConfig {
    /// Project metadata
    #[serde(default)]
    pub project: ProjectMetadata,

    /// Custom commands specific to this project
    #[serde(default)]
    pub commands: HashMap<String, String>,

    /// Project-specific development settings
    #[serde(default)]
    pub dev: ProjectDevConfig,

    /// Project-specific quality settings
    #[serde(default)]
    pub quality: ProjectQualityConfig,

    /// Git workflow settings
    #[serde(default)]
    pub git: ProjectGitConfig,

    /// Project-specific paths
    #[serde(default)]
    pub paths: ProjectPaths,
}

/// Project metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct ProjectMetadata {
    /// Project name
    pub name: String,

    /// Tech stack identifier (frontend-web, backend-api, etc.)
    #[serde(default)]
    pub tech_stack: Option<String>,

    /// Project description
    #[serde(default)]
    pub description: String,

    /// Project version
    #[serde(default = "default_project_version")]
    pub version: String,

    /// Repository URL
    #[serde(default)]
    pub repository: Option<String>,

    /// Project tags for categorization
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Project-specific development settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct ProjectDevConfig {
    /// Development server port (overrides stack default)
    #[serde(default)]
    pub port: Option<u16>,

    /// Hot reload enabled
    #[serde(default = "default_true")]
    pub hot_reload: bool,

    /// Source maps enabled
    #[serde(default = "default_true")]
    pub source_maps: bool,

    /// Custom environment variables for development
    #[serde(default)]
    pub env_vars: HashMap<String, String>,
}

/// Project-specific quality settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct ProjectQualityConfig {
    /// Enable pre-commit hooks
    #[serde(default = "default_true")]
    pub pre_commit_hooks: bool,

    /// Required code coverage percentage (0-100)
    #[serde(default)]
    pub min_coverage: Option<u8>,

    /// Custom linting rules
    #[serde(default)]
    pub custom_rules: Vec<String>,

    /// Files to exclude from quality checks
    #[serde(default)]
    pub exclude_patterns: Vec<String>,
}

/// Project-specific Git workflow settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[allow(dead_code)]
pub struct ProjectGitConfig {
    /// Default base branch for this project
    #[serde(default)]
    pub base_branch: Option<String>,

    /// Branch naming convention
    #[serde(default)]
    pub branch_prefix: Option<String>,

    /// Commit message template
    #[serde(default)]
    pub commit_template: Option<String>,

    /// Required reviewers
    #[serde(default)]
    pub required_reviewers: Vec<String>,

    /// Protected branches
    #[serde(default)]
    pub protected_branches: Vec<String>,
}

/// Project-specific paths
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct ProjectPaths {
    /// Source code directory
    #[serde(default = "default_src_dir")]
    pub src: PathBuf,

    /// Test directory
    #[serde(default = "default_test_dir")]
    pub tests: PathBuf,

    /// Documentation directory
    #[serde(default = "default_docs_dir")]
    pub docs: PathBuf,

    /// Build output directory
    #[serde(default = "default_dist_dir")]
    pub dist: PathBuf,

    /// Additional custom paths
    #[serde(default)]
    pub custom: HashMap<String, PathBuf>,
}

// Default value functions
fn default_project_version() -> String {
    "0.1.0".to_string()
}

fn default_true() -> bool {
    true
}

fn default_src_dir() -> PathBuf {
    PathBuf::from("src")
}

fn default_test_dir() -> PathBuf {
    PathBuf::from("tests")
}

fn default_docs_dir() -> PathBuf {
    PathBuf::from("docs")
}

fn default_dist_dir() -> PathBuf {
    PathBuf::from("dist")
}

impl Default for ProjectMetadata {
    fn default() -> Self {
        Self {
            name: String::new(),
            tech_stack: None,
            description: String::new(),
            version: default_project_version(),
            repository: None,
            tags: Vec::new(),
        }
    }
}

impl Default for ProjectDevConfig {
    fn default() -> Self {
        Self {
            port: None,
            hot_reload: true,
            source_maps: true,
            env_vars: HashMap::new(),
        }
    }
}

impl Default for ProjectQualityConfig {
    fn default() -> Self {
        Self {
            pre_commit_hooks: true,
            min_coverage: None,
            custom_rules: Vec::new(),
            exclude_patterns: Vec::new(),
        }
    }
}

impl Default for ProjectPaths {
    fn default() -> Self {
        Self {
            src: default_src_dir(),
            tests: default_test_dir(),
            docs: default_docs_dir(),
            dist: default_dist_dir(),
            custom: HashMap::new(),
        }
    }
}

impl ProjectConfig {
    /// Get the project configuration directory path
    ///
    /// Returns: `.cldev/` in the project root
    pub fn project_dir(project_root: &Path) -> PathBuf {
        project_root.join(".cldev")
    }

    /// Get the project configuration file path
    ///
    /// Returns: `.cldev/config.toml` in the project root
    pub fn config_path(project_root: &Path) -> PathBuf {
        Self::project_dir(project_root).join("config.toml")
    }

    /// Load project configuration from the specified project root
    ///
    /// # Arguments
    ///
    /// * `project_root` - Path to the project root directory
    ///
    /// # Returns
    ///
    /// Returns the loaded ProjectConfig, or default if file doesn't exist
    pub fn load(project_root: &Path) -> Result<Self> {
        let config_path = Self::config_path(project_root);

        // Return default if file doesn't exist
        if !config_path.exists() {
            return Ok(Self::default());
        }

        // Read and parse TOML
        let content = fs::read_to_string(&config_path).map_err(|e| {
            CldevError::config(format!(
                "Failed to read project config {}: {}",
                config_path.display(),
                e
            ))
        })?;

        let config: ProjectConfig = toml::from_str(&content).map_err(|e| {
            CldevError::config(format!(
                "Failed to parse project config {}: {}",
                config_path.display(),
                e
            ))
        })?;

        Ok(config)
    }

    /// Save project configuration to the project root
    ///
    /// # Arguments
    ///
    /// * `project_root` - Path to the project root directory
    pub fn save(&self, project_root: &Path) -> Result<()> {
        let config_path = Self::config_path(project_root);

        // Create .cldev directory if it doesn't exist
        let cldev_dir = Self::project_dir(project_root);
        if !cldev_dir.exists() {
            fs::create_dir_all(&cldev_dir).map_err(|e| {
                CldevError::io(format!(
                    "Failed to create .cldev directory {}: {}",
                    cldev_dir.display(),
                    e
                ))
            })?;
        }

        // Serialize to TOML
        let content = toml::to_string_pretty(self).map_err(|e| {
            CldevError::config(format!("Failed to serialize project config: {}", e))
        })?;

        // Write to file
        fs::write(&config_path, content).map_err(|e| {
            CldevError::io(format!(
                "Failed to write project config {}: {}",
                config_path.display(),
                e
            ))
        })?;

        Ok(())
    }

    /// Initialize a new project configuration
    ///
    /// # Arguments
    ///
    /// * `project_root` - Path to the project root directory
    /// * `name` - Project name
    /// * `tech_stack` - Optional tech stack identifier
    pub fn init(project_root: &Path, name: String, tech_stack: Option<TechStack>) -> Result<Self> {
        let mut config = Self::default();
        config.project.name = name;

        // Auto-detect project type and set defaults
        if let Some(ref stack) = tech_stack {
            config.project.tech_stack = Some(stack.as_str().to_string());
            config.apply_stack_defaults(stack);
        }

        // Save the initial configuration
        config.save(project_root)?;

        Ok(config)
    }

    /// Apply tech stack-specific defaults to project config
    fn apply_stack_defaults(&mut self, stack: &TechStack) {
        match stack {
            TechStack::FrontendWeb => {
                self.dev.port = Some(3000);
                self.quality.min_coverage = Some(80);
                self.paths.dist = PathBuf::from("dist");
            }
            TechStack::BackendApi => {
                self.dev.port = Some(8080);
                self.quality.min_coverage = Some(90);
                self.paths.dist = PathBuf::from("build");
            }
            TechStack::MobileApp => {
                self.dev.port = Some(8081);
                self.quality.min_coverage = Some(70);
            }
            TechStack::DataScience => {
                self.dev.port = Some(8888);
                self.quality.min_coverage = Some(75);
                self.paths
                    .custom
                    .insert("notebooks".to_string(), PathBuf::from("notebooks"));
            }
            TechStack::RustCli => {
                self.quality.min_coverage = Some(80);
                self.paths.dist = PathBuf::from("target");
            }
        }
    }

    /// Check if project config exists at the specified project root
    pub fn exists(project_root: &Path) -> bool {
        Self::config_path(project_root).exists()
    }

    /// Get the tech stack as an enum if set
    pub fn get_tech_stack(&self) -> Option<TechStack> {
        self.project
            .tech_stack
            .as_ref()
            .and_then(|s| TechStack::parse(s).ok())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_default_project_config() {
        let config = ProjectConfig::default();
        assert_eq!(config.project.version, "0.1.0");
        assert!(config.dev.hot_reload);
        assert!(config.quality.pre_commit_hooks);
        assert_eq!(config.paths.src, PathBuf::from("src"));
    }

    #[test]
    fn test_project_config_serialization() {
        let config = ProjectConfig::default();
        let toml_str = toml::to_string_pretty(&config).unwrap();
        assert!(toml_str.contains("[project]"));
        assert!(toml_str.contains("[dev]"));
        assert!(toml_str.contains("[paths]"));
    }

    #[test]
    fn test_project_config_save_and_load() {
        let temp_dir = TempDir::new().unwrap();
        let project_root = temp_dir.path();

        // Create and save config
        let mut config = ProjectConfig::default();
        config.project.name = "test-project".to_string();
        config.project.tech_stack = Some("rust-cli".to_string());
        config.save(project_root).unwrap();

        // Verify .cldev directory was created
        assert!(ProjectConfig::project_dir(project_root).exists());

        // Load config
        let loaded = ProjectConfig::load(project_root).unwrap();
        assert_eq!(loaded.project.name, "test-project");
        assert_eq!(loaded.project.tech_stack, Some("rust-cli".to_string()));
    }

    #[test]
    fn test_project_init() {
        let temp_dir = TempDir::new().unwrap();
        let project_root = temp_dir.path();

        let config = ProjectConfig::init(
            project_root,
            "my-app".to_string(),
            Some(TechStack::FrontendWeb),
        )
        .unwrap();

        assert_eq!(config.project.name, "my-app");
        assert_eq!(config.project.tech_stack, Some("frontend-web".to_string()));
        assert_eq!(config.dev.port, Some(3000));
        assert_eq!(config.quality.min_coverage, Some(80));

        // Verify file was saved
        assert!(ProjectConfig::exists(project_root));
    }

    #[test]
    fn test_apply_stack_defaults() {
        let mut config = ProjectConfig::default();

        config.apply_stack_defaults(&TechStack::BackendApi);
        assert_eq!(config.dev.port, Some(8080));
        assert_eq!(config.quality.min_coverage, Some(90));

        config.apply_stack_defaults(&TechStack::DataScience);
        assert_eq!(config.dev.port, Some(8888));
        assert!(config.paths.custom.contains_key("notebooks"));
    }

    #[test]
    fn test_get_tech_stack() {
        let mut config = ProjectConfig::default();
        assert!(config.get_tech_stack().is_none());

        config.project.tech_stack = Some("rust-cli".to_string());
        assert!(matches!(
            config.get_tech_stack().unwrap(),
            TechStack::RustCli
        ));

        config.project.tech_stack = Some("invalid".to_string());
        assert!(config.get_tech_stack().is_none());
    }

    #[test]
    fn test_load_nonexistent_returns_default() {
        let temp_dir = TempDir::new().unwrap();
        let config = ProjectConfig::load(temp_dir.path()).unwrap();
        assert_eq!(config.project.version, "0.1.0");
    }
}
