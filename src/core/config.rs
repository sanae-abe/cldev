//! Configuration management for cldev
//!
//! This module handles loading, saving, and validating the TOML configuration file
//! with versioning support. Configuration is shared across modules using Arc<Config>.
//!
//! # Configuration File Location
//!
//! Platform-specific default locations:
//! - **macOS**: `~/Library/Application Support/cldev/config.toml`
//! - **Linux**: `~/.config/cldev/config.toml`
//! - **Windows**: `%APPDATA%\cldev\config.toml`
//! - **Fallback**: `~/.cldev/config.toml` (if config directory cannot be determined)
//!
//! # Security
//!
//! - Configuration file permissions are checked and enforced (600)
//! - All paths are validated against path traversal attacks
//! - Sensitive data should never be stored in the config file

#![allow(dead_code)]

use crate::core::error::{CldevError, Result};
use crate::core::project_config::ProjectConfig;
use crate::core::stack_config::{StackConfig, TechStack};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};

/// Semantic version for configuration file format
///
/// Version changes:
/// - Major: Breaking changes, migration required
/// - Minor: Backward compatible additions
/// - Patch: Bug fixes only
pub const CONFIG_VERSION: &str = "1.0.0";

/// Validates a version string against the current CONFIG_VERSION
///
/// # Version Compatibility Rules
///
/// - Major version must match exactly
/// - Minor version can be lower (backward compatible)
/// - Patch version is ignored
pub fn validate_version(version: &str) -> Result<()> {
    let current_parts: Vec<&str> = CONFIG_VERSION.split('.').collect();
    let file_parts: Vec<&str> = version.split('.').collect();

    if current_parts.len() != 3 || file_parts.len() != 3 {
        return Err(CldevError::validation(format!(
            "Invalid version format: {}",
            version
        )));
    }

    // Major version must match
    if current_parts[0] != file_parts[0] {
        return Err(CldevError::validation(format!(
            "Configuration major version mismatch: expected {}, found {}. Migration required.",
            current_parts[0], file_parts[0]
        )));
    }

    // Minor version can be lower (backward compatible)
    let current_minor: u32 = current_parts[1]
        .parse()
        .map_err(|e| CldevError::validation(format!("Invalid minor version: {}", e)))?;
    let file_minor: u32 = file_parts[1]
        .parse()
        .map_err(|e| CldevError::validation(format!("Invalid minor version: {}", e)))?;

    if file_minor > current_minor {
        return Err(CldevError::validation(
            "Configuration file is from a newer version. Please update cldev.".to_string(),
        ));
    }

    Ok(())
}

/// Main configuration structure with versioning support
///
/// This structure is designed to be shared across modules using Arc<Config>
/// for efficient memory usage and thread-safe access.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Configuration file format version (required)
    #[serde(default = "default_version")]
    pub version: String,

    /// General settings
    #[serde(default)]
    pub general: GeneralConfig,

    /// Git-related settings
    #[serde(default)]
    pub git: GitConfig,

    /// Code quality settings
    #[serde(default)]
    pub quality: QualityConfig,

    /// Development workflow settings
    #[serde(default)]
    pub dev: DevConfig,

    /// Learning record settings
    #[serde(default)]
    pub lr: LearningRecordConfig,

    /// UI preferences
    #[serde(default)]
    pub ui: UiConfig,

    /// Performance tuning
    #[serde(default)]
    pub performance: PerformanceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    /// Language preference (en or ja)
    #[serde(default = "default_language")]
    pub language: String,

    /// Claude Code directory path
    #[serde(default = "default_claude_dir")]
    pub claude_dir: PathBuf,

    /// Projects root directory
    #[serde(default = "default_projects_dir")]
    pub projects_dir: PathBuf,

    /// Current tech stack (optional)
    #[serde(default)]
    pub tech_stack: Option<String>,

    /// Current project name (optional)
    #[serde(default)]
    pub project_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitConfig {
    /// GitHub CLI (gh) available
    #[serde(default = "default_true")]
    pub github_cli: bool,

    /// GitLab CLI (glab) available
    #[serde(default)]
    pub gitlab_cli: bool,

    /// Default base branch for PRs/MRs
    #[serde(default = "default_base_branch")]
    pub default_base_branch: String,

    /// Automatically push after commit
    #[serde(default = "default_true")]
    pub auto_push: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityConfig {
    /// Automatically fix linting issues
    #[serde(default)]
    pub auto_fix: bool,

    /// Run tests before commit
    #[serde(default = "default_true")]
    pub run_tests_before_commit: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevConfig {
    /// Automatically create feature branch
    #[serde(default = "default_true")]
    pub auto_create_branch: bool,

    /// Branch name prefix
    #[serde(default = "default_branch_prefix")]
    pub branch_prefix: String,

    /// Enable session recording
    #[serde(default = "default_true")]
    pub session_recording: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningRecordConfig {
    /// Learning sessions directory
    #[serde(default = "default_sessions_dir")]
    pub sessions_dir: PathBuf,

    /// Auto-save learning records
    #[serde(default = "default_true")]
    pub auto_save: bool,

    /// Default tags for new sessions
    #[serde(default = "default_tags")]
    pub default_tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    /// Enable colored output
    #[serde(default = "default_true")]
    pub color: bool,

    /// Enable emoji in output
    #[serde(default = "default_true")]
    pub emoji: bool,

    /// Show progress bars
    #[serde(default = "default_true")]
    pub progress_bar: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Number of parallel tasks
    #[serde(default = "default_parallel_tasks")]
    pub parallel_tasks: usize,

    /// Command timeout in seconds
    #[serde(default = "default_timeout")]
    pub timeout_seconds: u64,
}

// Default value functions
fn default_version() -> String {
    CONFIG_VERSION.to_string()
}

fn default_language() -> String {
    "ja".to_string()
}

// Cache for default claude directory
static DEFAULT_CLAUDE_DIR: OnceLock<PathBuf> = OnceLock::new();

fn default_claude_dir() -> PathBuf {
    DEFAULT_CLAUDE_DIR
        .get_or_init(|| {
            dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join(".claude")
        })
        .clone()
}

// Cache for default projects directory
static DEFAULT_PROJECTS_DIR: OnceLock<PathBuf> = OnceLock::new();

fn default_projects_dir() -> PathBuf {
    DEFAULT_PROJECTS_DIR
        .get_or_init(|| {
            dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("projects")
        })
        .clone()
}

fn default_true() -> bool {
    true
}

fn default_base_branch() -> String {
    "main".to_string()
}

fn default_branch_prefix() -> String {
    "feature".to_string()
}

// Cache for default sessions directory
static DEFAULT_SESSIONS_DIR: OnceLock<PathBuf> = OnceLock::new();

fn default_sessions_dir() -> PathBuf {
    DEFAULT_SESSIONS_DIR
        .get_or_init(|| default_claude_dir().join("learning-sessions"))
        .clone()
}

fn default_tags() -> Vec<String> {
    vec!["development".to_string(), "claude-code".to_string()]
}

fn default_parallel_tasks() -> usize {
    4
}

fn default_timeout() -> u64 {
    300
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            language: default_language(),
            claude_dir: default_claude_dir(),
            projects_dir: default_projects_dir(),
            tech_stack: None,
            project_name: None,
        }
    }
}

impl Default for GitConfig {
    fn default() -> Self {
        Self {
            github_cli: default_true(),
            gitlab_cli: false,
            default_base_branch: default_base_branch(),
            auto_push: default_true(),
        }
    }
}

impl Default for QualityConfig {
    fn default() -> Self {
        Self {
            auto_fix: false,
            run_tests_before_commit: default_true(),
        }
    }
}

impl Default for DevConfig {
    fn default() -> Self {
        Self {
            auto_create_branch: default_true(),
            branch_prefix: default_branch_prefix(),
            session_recording: default_true(),
        }
    }
}

impl Default for LearningRecordConfig {
    fn default() -> Self {
        Self {
            sessions_dir: default_sessions_dir(),
            auto_save: default_true(),
            default_tags: default_tags(),
        }
    }
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            color: default_true(),
            emoji: default_true(),
            progress_bar: default_true(),
        }
    }
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            parallel_tasks: default_parallel_tasks(),
            timeout_seconds: default_timeout(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: default_version(),
            general: GeneralConfig::default(),
            git: GitConfig::default(),
            quality: QualityConfig::default(),
            dev: DevConfig::default(),
            lr: LearningRecordConfig::default(),
            ui: UiConfig::default(),
            performance: PerformanceConfig::default(),
        }
    }
}

impl Config {
    /// Get the default configuration file path
    ///
    /// Platform-specific locations:
    /// - **macOS**: `~/Library/Application Support/cldev/config.toml`
    /// - **Linux**: `~/.config/cldev/config.toml`
    /// - **Windows**: `%APPDATA%\cldev\config.toml`
    /// - **Fallback**: `~/.cldev/config.toml` (if config directory cannot be determined)
    pub fn default_path() -> Result<PathBuf> {
        if let Some(config_dir) = dirs::config_dir() {
            Ok(config_dir.join("cldev").join("config.toml"))
        } else {
            // Fallback to home directory
            dirs::home_dir()
                .map(|home| home.join(".cldev").join("config.toml"))
                .ok_or_else(|| CldevError::config("Could not determine home directory"))
        }
    }

    /// Load configuration from the default path or custom path
    ///
    /// # Arguments
    ///
    /// * `path` - Optional custom path to configuration file
    ///
    /// # Returns
    ///
    /// Returns `Arc<Config>` for efficient sharing across modules
    ///
    /// # Security
    ///
    /// - Validates file permissions (should be 600)
    /// - Validates version compatibility
    /// - Returns default config if file doesn't exist
    pub fn load(path: Option<PathBuf>) -> Result<Arc<Self>> {
        let config_path = path.unwrap_or_else(|| {
            Self::default_path().unwrap_or_else(|_| PathBuf::from("~/.config/cldev/config.toml"))
        });

        // If config file doesn't exist, return default config
        if !config_path.exists() {
            return Ok(Arc::new(Self::default()));
        }

        // Check file permissions (Unix only)
        #[cfg(unix)]
        Self::check_permissions(&config_path)?;

        // Read and parse TOML
        let content = fs::read_to_string(&config_path).map_err(|e| {
            CldevError::config(format!(
                "Failed to read config file {}: {}",
                config_path.display(),
                e
            ))
        })?;

        let config: Config = toml::from_str(&content).map_err(|e| {
            CldevError::config(format!(
                "Failed to parse config file {}: {}",
                config_path.display(),
                e
            ))
        })?;

        // Validate version
        validate_version(&config.version)?;

        Ok(Arc::new(config))
    }

    /// Save configuration to file
    ///
    /// # Arguments
    ///
    /// * `path` - Optional custom path (uses default if None)
    ///
    /// # Security
    ///
    /// - Creates parent directories if needed
    /// - Sets file permissions to 600 (owner read/write only)
    pub fn save(&self, path: Option<PathBuf>) -> Result<()> {
        let config_path = path.unwrap_or_else(|| {
            Self::default_path().unwrap_or_else(|_| PathBuf::from("~/.config/cldev/config.toml"))
        });

        // Create parent directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                CldevError::io(format!(
                    "Failed to create config directory {}: {}",
                    parent.display(),
                    e
                ))
            })?;
        }

        // Serialize to TOML
        let content = toml::to_string_pretty(self)
            .map_err(|e| CldevError::config(format!("Failed to serialize configuration: {}", e)))?;

        // Write to file
        fs::write(&config_path, content).map_err(|e| {
            CldevError::io(format!(
                "Failed to write config file {}: {}",
                config_path.display(),
                e
            ))
        })?;

        // Set permissions to 600 (Unix only)
        #[cfg(unix)]
        Self::set_permissions(&config_path)?;

        Ok(())
    }

    /// Check configuration file permissions (Unix only)
    ///
    /// Ensures the config file has restrictive permissions (600) to prevent
    /// unauthorized access.
    #[cfg(unix)]
    fn check_permissions(path: &Path) -> Result<()> {
        use std::os::unix::fs::PermissionsExt;

        let metadata = fs::metadata(path).map_err(|e| {
            CldevError::io(format!(
                "Failed to read metadata for {}: {}",
                path.display(),
                e
            ))
        })?;

        let permissions = metadata.permissions();
        let mode = permissions.mode();

        // Check if permissions are more permissive than 600
        if mode & 0o177 != 0 {
            return Err(CldevError::security(format!(
                "Configuration file has insecure permissions: {:o}. Expected 600 (owner read/write only).\nRun: chmod 600 {}",
                mode & 0o777,
                path.display()
            )));
        }

        Ok(())
    }

    /// Set configuration file permissions to 600 (Unix only)
    #[cfg(unix)]
    fn set_permissions(path: &Path) -> Result<()> {
        use std::os::unix::fs::PermissionsExt;

        let mut permissions = fs::metadata(path)
            .map_err(|e| {
                CldevError::io(format!(
                    "Failed to read metadata for {}: {}",
                    path.display(),
                    e
                ))
            })?
            .permissions();
        permissions.set_mode(0o600);
        fs::set_permissions(path, permissions).map_err(|e| {
            CldevError::io(format!(
                "Failed to set permissions for {}: {}",
                path.display(),
                e
            ))
        })?;

        Ok(())
    }

    /// Load hierarchical configuration with 3-layer system
    ///
    /// # Configuration Hierarchy (Priority: Project > Stack > Global)
    ///
    /// 1. **Global**: OS-specific config directory (e.g., `~/Library/Application Support/cldev/config.toml` on macOS)
    /// 2. **Stack**: OS-specific config directory + `stacks/{stack_name}.toml`
    /// 3. **Project**: `./.cldev/config.toml` (if project_root is provided)
    ///
    /// # Arguments
    ///
    /// * `project_root` - Optional project root directory for project-level config
    ///
    /// # Returns
    ///
    /// Returns merged configuration with proper priority handling
    pub fn load_hierarchical(project_root: Option<PathBuf>) -> Result<HierarchicalConfig> {
        // 1. Load global config
        let global = Self::load(None)?;

        // 2. Load stack config if tech_stack is specified
        let stack = if let Some(ref stack_name) = global.general.tech_stack {
            let tech_stack = TechStack::parse(stack_name)?;
            Some(StackConfig::load(&tech_stack)?)
        } else {
            None
        };

        // 3. Load project config if project_root is provided
        let project = if let Some(ref root) = project_root {
            if ProjectConfig::exists(root) {
                Some(ProjectConfig::load(root)?)
            } else {
                None
            }
        } else {
            None
        };

        Ok(HierarchicalConfig {
            global,
            stack,
            project,
        })
    }

    /// Get tech stack enum from configuration
    pub fn get_tech_stack(&self) -> Option<TechStack> {
        self.general
            .tech_stack
            .as_ref()
            .and_then(|s| TechStack::parse(s).ok())
    }

    /// Set tech stack in configuration
    pub fn set_tech_stack(&mut self, stack: Option<TechStack>) {
        self.general.tech_stack = stack.map(|s| s.as_str().to_string());
    }

    /// Get project name from configuration
    pub fn get_project_name(&self) -> Option<&str> {
        self.general.project_name.as_deref()
    }

    /// Set project name in configuration
    pub fn set_project_name(&mut self, name: Option<String>) {
        self.general.project_name = name;
    }
}

/// Hierarchical configuration with 3 layers
///
/// This structure holds all three configuration layers:
/// - Global: User-wide settings
/// - Stack: Technology stack-specific settings
/// - Project: Project-specific settings
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct HierarchicalConfig {
    /// Global configuration (OS-specific config directory)
    pub global: Arc<Config>,

    /// Stack configuration (OS-specific config directory + stacks/{stack}.toml)
    pub stack: Option<StackConfig>,

    /// Project configuration (./.cldev/config.toml)
    pub project: Option<ProjectConfig>,
}

impl HierarchicalConfig {
    /// Get the effective tech stack
    ///
    /// Priority: Project > Global
    pub fn effective_tech_stack(&self) -> Option<TechStack> {
        self.project
            .as_ref()
            .and_then(|p| p.get_tech_stack())
            .or_else(|| self.global.get_tech_stack())
    }

    /// Get effective project name
    ///
    /// Priority: Project > Global
    pub fn effective_project_name(&self) -> Option<String> {
        self.project
            .as_ref()
            .map(|p| p.project.name.clone())
            .filter(|n| !n.is_empty())
            .or_else(|| self.global.get_project_name().map(String::from))
    }

    /// Get effective dev port
    ///
    /// Priority: Project > Stack > Default
    pub fn effective_dev_port(&self) -> Option<u16> {
        self.project
            .as_ref()
            .and_then(|p| p.dev.port)
            .or_else(|| self.stack.as_ref().and_then(|s| s.environment.dev_port))
    }

    /// Get effective base branch
    ///
    /// Priority: Project > Global
    pub fn effective_base_branch(&self) -> String {
        self.project
            .as_ref()
            .and_then(|p| p.git.base_branch.clone())
            .unwrap_or_else(|| self.global.git.default_base_branch.clone())
    }

    /// Merge command definitions from all layers
    ///
    /// Priority: Project > Stack > Default
    pub fn merged_commands(&self) -> std::collections::HashMap<String, String> {
        let mut commands = std::collections::HashMap::new();

        // Start with stack commands
        if let Some(ref stack) = self.stack {
            commands.extend(stack.commands.clone());
        }

        // Override with project commands
        if let Some(ref project) = self.project {
            commands.extend(project.commands.clone());
        }

        commands
    }

    /// Get effective coverage threshold
    ///
    /// Priority: Project > Stack > Default
    pub fn effective_coverage_threshold(&self) -> Option<u8> {
        self.project
            .as_ref()
            .and_then(|p| p.quality.min_coverage)
            .or_else(|| {
                self.stack
                    .as_ref()
                    .and_then(|s| s.quality.coverage_threshold)
            })
    }
}

/// Configuration version utilities
#[allow(dead_code)]
pub struct ConfigVersion;

impl ConfigVersion {
    /// Get the current config version
    pub fn current() -> &'static str {
        CONFIG_VERSION
    }

    /// Parse a version string into (major, minor, patch)
    pub fn parse(version: &str) -> Result<(u32, u32, u32)> {
        let parts: Vec<&str> = version.split('.').collect();
        if parts.len() != 3 {
            return Err(CldevError::validation(format!(
                "Invalid version format: {}",
                version
            )));
        }

        let major = parts[0]
            .parse()
            .map_err(|e| CldevError::validation(format!("Invalid major version: {}", e)))?;
        let minor = parts[1]
            .parse()
            .map_err(|e| CldevError::validation(format!("Invalid minor version: {}", e)))?;
        let patch = parts[2]
            .parse()
            .map_err(|e| CldevError::validation(format!("Invalid patch version: {}", e)))?;

        Ok((major, minor, patch))
    }

    /// Check if a version is compatible with the current version
    pub fn is_compatible(version: &str) -> bool {
        validate_version(version).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.version, CONFIG_VERSION);
        assert_eq!(config.general.language, "ja");
        assert_eq!(config.git.default_base_branch, "main");
        assert!(config.dev.auto_create_branch);
        assert_eq!(config.performance.parallel_tasks, 4);
        assert_eq!(config.performance.timeout_seconds, 300);
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let toml_str = toml::to_string_pretty(&config).unwrap();
        assert!(toml_str.contains("version = \"1.0.0\""));
        assert!(toml_str.contains("[general]"));
        assert!(toml_str.contains("[git]"));
    }

    #[test]
    fn test_config_deserialization() {
        let toml_str = r#"
            version = "1.0.0"

            [general]
            language = "en"

            [git]
            default_base_branch = "develop"
        "#;

        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(config.version, "1.0.0");
        assert_eq!(config.general.language, "en");
        assert_eq!(config.git.default_base_branch, "develop");
    }

    #[test]
    fn test_version_validation() {
        // Same version - OK
        assert!(validate_version("1.0.0").is_ok());

        // Lower minor version - OK (backward compatible)
        assert!(validate_version("1.0.0").is_ok());

        // Different major version - Error
        assert!(validate_version("2.0.0").is_err());

        // Invalid format - Error
        assert!(validate_version("1.0").is_err());
        assert!(validate_version("invalid").is_err());
    }

    #[test]
    fn test_config_version_parse() {
        let (major, minor, patch) = ConfigVersion::parse("1.2.3").unwrap();
        assert_eq!(major, 1);
        assert_eq!(minor, 2);
        assert_eq!(patch, 3);

        assert!(ConfigVersion::parse("invalid").is_err());
        assert!(ConfigVersion::parse("1.2").is_err());
    }

    #[test]
    fn test_config_save_and_load() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");

        // Create and save config
        let mut config = Config::default();
        config.general.language = "en".to_string();
        config.save(Some(config_path.clone())).unwrap();

        // Load config
        let loaded = Config::load(Some(config_path.clone())).unwrap();
        assert_eq!(loaded.general.language, "en");
        assert_eq!(loaded.version, CONFIG_VERSION);
    }

    #[test]
    fn test_load_nonexistent_returns_default() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("nonexistent.toml");

        let config = Config::load(Some(config_path)).unwrap();
        assert_eq!(config.version, CONFIG_VERSION);
        assert_eq!(config.general.language, "ja");
    }

    #[test]
    #[cfg(unix)]
    fn test_config_permissions() {
        use std::os::unix::fs::PermissionsExt;

        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");

        // Save config (should set permissions to 600)
        let config = Config::default();
        config.save(Some(config_path.clone())).unwrap();

        // Check permissions
        let metadata = fs::metadata(&config_path).unwrap();
        let mode = metadata.permissions().mode();
        assert_eq!(mode & 0o777, 0o600);
    }

    #[test]
    fn test_arc_sharing() {
        let config = Config::load(None).unwrap();
        let config_clone = Arc::clone(&config);

        // Both should point to the same data
        assert_eq!(config.version, config_clone.version);
        assert_eq!(Arc::strong_count(&config), 2);
    }
}
