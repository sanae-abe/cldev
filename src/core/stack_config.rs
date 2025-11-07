//! Technical stack configuration management
//!
//! This module manages technology stack-specific settings that are loaded from
//! `~/.config/cldev/stacks/{stack_name}.toml` files. Stack configurations provide
//! specialized commands, tools, and settings for different development environments.
//!
//! # Supported Stacks
//!
//! - `frontend-web`: React/Vue/Angular, TypeScript, Web development
//! - `backend-api`: Node.js/Python/Go/Rust, API development
//! - `mobile-app`: React Native/Flutter, iOS/Android
//! - `data-science`: Python/R, Jupyter, ML/Data analysis
//! - `rust-cli`: Rust CLI development with modern tooling
//!
//! # Configuration Hierarchy
//!
//! Stack configs sit between global and project configs:
//! Global (~/.config/cldev/config.toml) → Stack → Project (.cldev/config.toml)

#![allow(dead_code)]

use crate::core::error::{CldevError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Technical stack identifier
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[allow(dead_code)]
pub enum TechStack {
    /// Frontend web development (React, Vue, Angular, TypeScript)
    FrontendWeb,
    /// Backend API development (Node.js, Python, Go, Rust)
    BackendApi,
    /// Mobile app development (React Native, Flutter, iOS, Android)
    MobileApp,
    /// Data science and ML (Python, R, Jupyter)
    DataScience,
    /// Rust CLI development
    RustCli,
}

impl TechStack {
    /// Get the stack name as a string
    pub fn as_str(&self) -> &'static str {
        match self {
            TechStack::FrontendWeb => "frontend-web",
            TechStack::BackendApi => "backend-api",
            TechStack::MobileApp => "mobile-app",
            TechStack::DataScience => "data-science",
            TechStack::RustCli => "rust-cli",
        }
    }

    /// Parse stack name from string
    pub fn from_str(s: &str) -> Result<Self> {
        match s {
            "frontend-web" => Ok(TechStack::FrontendWeb),
            "backend-api" => Ok(TechStack::BackendApi),
            "mobile-app" => Ok(TechStack::MobileApp),
            "data-science" => Ok(TechStack::DataScience),
            "rust-cli" => Ok(TechStack::RustCli),
            _ => Err(CldevError::validation(format!(
                "Unknown tech stack: {}. Valid options: frontend-web, backend-api, mobile-app, data-science, rust-cli",
                s
            ))),
        }
    }

    /// Get all available tech stacks
    pub fn all() -> Vec<Self> {
        vec![
            TechStack::FrontendWeb,
            TechStack::BackendApi,
            TechStack::MobileApp,
            TechStack::DataScience,
            TechStack::RustCli,
        ]
    }
}

/// Stack-specific configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct StackConfig {
    /// Stack metadata
    #[serde(default)]
    pub stack: StackMetadata,

    /// Command definitions (e.g., build, test, lint)
    #[serde(default)]
    pub commands: HashMap<String, String>,

    /// Development tools configuration
    #[serde(default)]
    pub tools: ToolsConfig,

    /// Quality and linting configuration
    #[serde(default)]
    pub quality: StackQualityConfig,

    /// Testing configuration
    #[serde(default)]
    pub testing: TestingConfig,

    /// Stack-specific environment settings
    #[serde(default)]
    pub environment: EnvironmentConfig,
}

/// Stack metadata information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct StackMetadata {
    /// Stack name (frontend-web, backend-api, etc.)
    pub name: String,

    /// Human-readable description
    #[serde(default)]
    pub description: String,

    /// Primary languages used in this stack
    #[serde(default)]
    pub languages: Vec<String>,

    /// Primary frameworks used in this stack
    #[serde(default)]
    pub frameworks: Vec<String>,

    /// Stack version (for configuration format)
    #[serde(default = "default_stack_version")]
    pub version: String,
}

/// Development tools configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct ToolsConfig {
    /// Package manager (npm, yarn, pnpm, cargo, pip, etc.)
    #[serde(default)]
    pub package_manager: Option<String>,

    /// Code formatter (prettier, black, rustfmt, etc.)
    #[serde(default)]
    pub formatter: Option<String>,

    /// Linter (eslint, pylint, clippy, etc.)
    #[serde(default)]
    pub linter: Option<String>,

    /// Type checker (tsc, mypy, etc.)
    #[serde(default)]
    pub type_checker: Option<String>,

    /// Build tool (webpack, vite, cargo, etc.)
    #[serde(default)]
    pub build_tool: Option<String>,

    /// Test runner (jest, pytest, cargo test, etc.)
    #[serde(default)]
    pub test_runner: Option<String>,
}

/// Stack-specific quality configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct StackQualityConfig {
    /// Required linting rules
    #[serde(default)]
    pub required_rules: Vec<String>,

    /// Code coverage threshold (0-100)
    #[serde(default)]
    pub coverage_threshold: Option<u8>,

    /// Maximum cyclomatic complexity
    #[serde(default)]
    pub max_complexity: Option<u32>,

    /// Enable strict mode (TypeScript strict, Rust deny warnings, etc.)
    #[serde(default = "default_true")]
    pub strict_mode: bool,
}

/// Testing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct TestingConfig {
    /// Test file patterns (e.g., "**/*.test.ts", "tests/**/*.rs")
    #[serde(default)]
    pub test_patterns: Vec<String>,

    /// Test command template
    #[serde(default)]
    pub test_command: Option<String>,

    /// Coverage command template
    #[serde(default)]
    pub coverage_command: Option<String>,

    /// Watch mode command
    #[serde(default)]
    pub watch_command: Option<String>,
}

/// Environment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct EnvironmentConfig {
    /// Required environment variables (names only, not values)
    #[serde(default)]
    pub required_env_vars: Vec<String>,

    /// Development server port
    #[serde(default)]
    pub dev_port: Option<u16>,

    /// Node version requirement (for Node.js stacks)
    #[serde(default)]
    pub node_version: Option<String>,

    /// Python version requirement (for Python stacks)
    #[serde(default)]
    pub python_version: Option<String>,

    /// Rust version requirement (for Rust stacks)
    #[serde(default)]
    pub rust_version: Option<String>,
}

// Default value functions
fn default_stack_version() -> String {
    "1.0.0".to_string()
}

fn default_true() -> bool {
    true
}

impl Default for StackMetadata {
    fn default() -> Self {
        Self {
            name: String::new(),
            description: String::new(),
            languages: Vec::new(),
            frameworks: Vec::new(),
            version: default_stack_version(),
        }
    }
}

impl Default for ToolsConfig {
    fn default() -> Self {
        Self {
            package_manager: None,
            formatter: None,
            linter: None,
            type_checker: None,
            build_tool: None,
            test_runner: None,
        }
    }
}

impl Default for StackQualityConfig {
    fn default() -> Self {
        Self {
            required_rules: Vec::new(),
            coverage_threshold: None,
            max_complexity: None,
            strict_mode: true,
        }
    }
}

impl Default for TestingConfig {
    fn default() -> Self {
        Self {
            test_patterns: Vec::new(),
            test_command: None,
            coverage_command: None,
            watch_command: None,
        }
    }
}

impl Default for EnvironmentConfig {
    fn default() -> Self {
        Self {
            required_env_vars: Vec::new(),
            dev_port: None,
            node_version: None,
            python_version: None,
            rust_version: None,
        }
    }
}

impl Default for StackConfig {
    fn default() -> Self {
        Self {
            stack: StackMetadata::default(),
            commands: HashMap::new(),
            tools: ToolsConfig::default(),
            quality: StackQualityConfig::default(),
            testing: TestingConfig::default(),
            environment: EnvironmentConfig::default(),
        }
    }
}

impl StackConfig {
    /// Get the default stack configuration directory
    ///
    /// Returns: `~/.config/cldev/stacks/`
    pub fn stacks_dir() -> Result<PathBuf> {
        if let Some(config_dir) = dirs::config_dir() {
            Ok(config_dir.join("cldev").join("stacks"))
        } else {
            dirs::home_dir()
                .map(|home| home.join(".cldev").join("stacks"))
                .ok_or_else(|| CldevError::config("Could not determine home directory"))
        }
    }

    /// Get the path for a specific stack configuration file
    ///
    /// # Arguments
    ///
    /// * `stack` - The tech stack identifier
    ///
    /// # Returns
    ///
    /// Path to `~/.config/cldev/stacks/{stack_name}.toml`
    pub fn stack_path(stack: &TechStack) -> Result<PathBuf> {
        Ok(Self::stacks_dir()?.join(format!("{}.toml", stack.as_str())))
    }

    /// Load stack configuration from file
    ///
    /// # Arguments
    ///
    /// * `stack` - The tech stack to load
    ///
    /// # Returns
    ///
    /// Returns the loaded StackConfig, or default if file doesn't exist
    pub fn load(stack: &TechStack) -> Result<Self> {
        let stack_path = Self::stack_path(stack)?;

        // Return default if file doesn't exist
        if !stack_path.exists() {
            return Ok(Self::default_for_stack(stack));
        }

        // Read and parse TOML
        let content = fs::read_to_string(&stack_path).map_err(|e| {
            CldevError::config(format!(
                "Failed to read stack config {}: {}",
                stack_path.display(),
                e
            ))
        })?;

        let config: StackConfig = toml::from_str(&content).map_err(|e| {
            CldevError::config(format!(
                "Failed to parse stack config {}: {}",
                stack_path.display(),
                e
            ))
        })?;

        Ok(config)
    }

    /// Save stack configuration to file
    ///
    /// # Arguments
    ///
    /// * `stack` - The tech stack identifier
    pub fn save(&self, stack: &TechStack) -> Result<()> {
        let stack_path = Self::stack_path(stack)?;

        // Create parent directory if it doesn't exist
        if let Some(parent) = stack_path.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                CldevError::io(format!(
                    "Failed to create stack config directory {}: {}",
                    parent.display(),
                    e
                ))
            })?;
        }

        // Serialize to TOML
        let content = toml::to_string_pretty(self)
            .map_err(|e| CldevError::config(format!("Failed to serialize stack config: {}", e)))?;

        // Write to file
        fs::write(&stack_path, content).map_err(|e| {
            CldevError::io(format!(
                "Failed to write stack config {}: {}",
                stack_path.display(),
                e
            ))
        })?;

        Ok(())
    }

    /// Get default configuration for a specific tech stack
    ///
    /// This provides sensible defaults when no stack config file exists
    pub fn default_for_stack(stack: &TechStack) -> Self {
        match stack {
            TechStack::FrontendWeb => Self::default_frontend_web(),
            TechStack::BackendApi => Self::default_backend_api(),
            TechStack::MobileApp => Self::default_mobile_app(),
            TechStack::DataScience => Self::default_data_science(),
            TechStack::RustCli => Self::default_rust_cli(),
        }
    }

    fn default_frontend_web() -> Self {
        let mut commands = HashMap::new();
        commands.insert("dev".to_string(), "npm run dev".to_string());
        commands.insert("build".to_string(), "npm run build".to_string());
        commands.insert("test".to_string(), "npm test".to_string());
        commands.insert("lint".to_string(), "npm run lint".to_string());
        commands.insert("format".to_string(), "npm run format".to_string());

        Self {
            stack: StackMetadata {
                name: "frontend-web".to_string(),
                description: "Frontend web development with React/Vue/Angular".to_string(),
                languages: vec!["TypeScript".to_string(), "JavaScript".to_string()],
                frameworks: vec![
                    "React".to_string(),
                    "Vue".to_string(),
                    "Angular".to_string(),
                ],
                version: default_stack_version(),
            },
            commands,
            tools: ToolsConfig {
                package_manager: Some("npm".to_string()),
                formatter: Some("prettier".to_string()),
                linter: Some("eslint".to_string()),
                type_checker: Some("tsc".to_string()),
                build_tool: Some("vite".to_string()),
                test_runner: Some("vitest".to_string()),
            },
            quality: StackQualityConfig {
                required_rules: vec!["no-console".to_string(), "no-debugger".to_string()],
                coverage_threshold: Some(80),
                max_complexity: Some(10),
                strict_mode: true,
            },
            testing: TestingConfig {
                test_patterns: vec!["**/*.test.ts".to_string(), "**/*.spec.ts".to_string()],
                test_command: Some("npm test".to_string()),
                coverage_command: Some("npm run test:coverage".to_string()),
                watch_command: Some("npm run test:watch".to_string()),
            },
            environment: EnvironmentConfig {
                required_env_vars: Vec::new(),
                dev_port: Some(3000),
                node_version: Some(">=18.0.0".to_string()),
                python_version: None,
                rust_version: None,
            },
        }
    }

    fn default_backend_api() -> Self {
        let mut commands = HashMap::new();
        commands.insert("dev".to_string(), "npm run dev".to_string());
        commands.insert("build".to_string(), "npm run build".to_string());
        commands.insert("test".to_string(), "npm test".to_string());
        commands.insert("lint".to_string(), "npm run lint".to_string());

        Self {
            stack: StackMetadata {
                name: "backend-api".to_string(),
                description: "Backend API development with Node.js/Python/Go/Rust".to_string(),
                languages: vec![
                    "TypeScript".to_string(),
                    "Python".to_string(),
                    "Go".to_string(),
                ],
                frameworks: vec!["Express".to_string(), "FastAPI".to_string()],
                version: default_stack_version(),
            },
            commands,
            tools: ToolsConfig {
                package_manager: Some("npm".to_string()),
                formatter: Some("prettier".to_string()),
                linter: Some("eslint".to_string()),
                type_checker: Some("tsc".to_string()),
                build_tool: None,
                test_runner: Some("jest".to_string()),
            },
            quality: StackQualityConfig {
                required_rules: vec!["security/detect-object-injection".to_string()],
                coverage_threshold: Some(90),
                max_complexity: Some(15),
                strict_mode: true,
            },
            testing: TestingConfig {
                test_patterns: vec!["**/*.test.ts".to_string()],
                test_command: Some("npm test".to_string()),
                coverage_command: Some("npm run test:coverage".to_string()),
                watch_command: None,
            },
            environment: EnvironmentConfig {
                required_env_vars: vec!["DATABASE_URL".to_string(), "API_KEY".to_string()],
                dev_port: Some(8080),
                node_version: Some(">=18.0.0".to_string()),
                python_version: None,
                rust_version: None,
            },
        }
    }

    fn default_mobile_app() -> Self {
        let mut commands = HashMap::new();
        commands.insert("dev".to_string(), "npm run start".to_string());
        commands.insert("build".to_string(), "npm run build".to_string());
        commands.insert("test".to_string(), "npm test".to_string());
        commands.insert("ios".to_string(), "npm run ios".to_string());
        commands.insert("android".to_string(), "npm run android".to_string());

        Self {
            stack: StackMetadata {
                name: "mobile-app".to_string(),
                description: "Mobile app development with React Native/Flutter".to_string(),
                languages: vec!["TypeScript".to_string(), "Dart".to_string()],
                frameworks: vec!["React Native".to_string(), "Flutter".to_string()],
                version: default_stack_version(),
            },
            commands,
            tools: ToolsConfig {
                package_manager: Some("npm".to_string()),
                formatter: Some("prettier".to_string()),
                linter: Some("eslint".to_string()),
                type_checker: Some("tsc".to_string()),
                build_tool: Some("metro".to_string()),
                test_runner: Some("jest".to_string()),
            },
            quality: StackQualityConfig {
                required_rules: Vec::new(),
                coverage_threshold: Some(70),
                max_complexity: Some(12),
                strict_mode: true,
            },
            testing: TestingConfig {
                test_patterns: vec!["**/*.test.tsx".to_string()],
                test_command: Some("npm test".to_string()),
                coverage_command: Some("npm run test:coverage".to_string()),
                watch_command: Some("npm run test:watch".to_string()),
            },
            environment: EnvironmentConfig {
                required_env_vars: Vec::new(),
                dev_port: Some(8081),
                node_version: Some(">=18.0.0".to_string()),
                python_version: None,
                rust_version: None,
            },
        }
    }

    fn default_data_science() -> Self {
        let mut commands = HashMap::new();
        commands.insert("notebook".to_string(), "jupyter notebook".to_string());
        commands.insert("lab".to_string(), "jupyter lab".to_string());
        commands.insert("test".to_string(), "pytest".to_string());
        commands.insert("lint".to_string(), "pylint **/*.py".to_string());
        commands.insert("format".to_string(), "black .".to_string());

        Self {
            stack: StackMetadata {
                name: "data-science".to_string(),
                description: "Data science and ML with Python/R".to_string(),
                languages: vec!["Python".to_string(), "R".to_string()],
                frameworks: vec![
                    "Jupyter".to_string(),
                    "pandas".to_string(),
                    "scikit-learn".to_string(),
                ],
                version: default_stack_version(),
            },
            commands,
            tools: ToolsConfig {
                package_manager: Some("pip".to_string()),
                formatter: Some("black".to_string()),
                linter: Some("pylint".to_string()),
                type_checker: Some("mypy".to_string()),
                build_tool: None,
                test_runner: Some("pytest".to_string()),
            },
            quality: StackQualityConfig {
                required_rules: Vec::new(),
                coverage_threshold: Some(75),
                max_complexity: Some(15),
                strict_mode: false,
            },
            testing: TestingConfig {
                test_patterns: vec!["tests/**/*.py".to_string(), "test_*.py".to_string()],
                test_command: Some("pytest".to_string()),
                coverage_command: Some("pytest --cov".to_string()),
                watch_command: None,
            },
            environment: EnvironmentConfig {
                required_env_vars: Vec::new(),
                dev_port: Some(8888),
                node_version: None,
                python_version: Some(">=3.9".to_string()),
                rust_version: None,
            },
        }
    }

    fn default_rust_cli() -> Self {
        let mut commands = HashMap::new();
        commands.insert("build".to_string(), "cargo build".to_string());
        commands.insert("test".to_string(), "cargo test".to_string());
        commands.insert("lint".to_string(), "cargo clippy".to_string());
        commands.insert("format".to_string(), "cargo fmt".to_string());
        commands.insert("run".to_string(), "cargo run".to_string());

        Self {
            stack: StackMetadata {
                name: "rust-cli".to_string(),
                description: "Rust CLI development with modern tooling".to_string(),
                languages: vec!["Rust".to_string()],
                frameworks: vec!["clap".to_string(), "tokio".to_string()],
                version: default_stack_version(),
            },
            commands,
            tools: ToolsConfig {
                package_manager: Some("cargo".to_string()),
                formatter: Some("rustfmt".to_string()),
                linter: Some("clippy".to_string()),
                type_checker: None,
                build_tool: Some("cargo".to_string()),
                test_runner: Some("cargo".to_string()),
            },
            quality: StackQualityConfig {
                required_rules: vec!["clippy::all".to_string(), "clippy::pedantic".to_string()],
                coverage_threshold: Some(80),
                max_complexity: Some(10),
                strict_mode: true,
            },
            testing: TestingConfig {
                test_patterns: vec!["tests/**/*.rs".to_string()],
                test_command: Some("cargo test".to_string()),
                coverage_command: Some("cargo tarpaulin".to_string()),
                watch_command: Some("cargo watch -x test".to_string()),
            },
            environment: EnvironmentConfig {
                required_env_vars: Vec::new(),
                dev_port: None,
                node_version: None,
                python_version: None,
                rust_version: Some(">=1.70".to_string()),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tech_stack_as_str() {
        assert_eq!(TechStack::FrontendWeb.as_str(), "frontend-web");
        assert_eq!(TechStack::BackendApi.as_str(), "backend-api");
        assert_eq!(TechStack::MobileApp.as_str(), "mobile-app");
        assert_eq!(TechStack::DataScience.as_str(), "data-science");
        assert_eq!(TechStack::RustCli.as_str(), "rust-cli");
    }

    #[test]
    fn test_tech_stack_from_str() {
        assert!(matches!(
            TechStack::from_str("frontend-web").unwrap(),
            TechStack::FrontendWeb
        ));
        assert!(matches!(
            TechStack::from_str("backend-api").unwrap(),
            TechStack::BackendApi
        ));
        assert!(TechStack::from_str("invalid").is_err());
    }

    #[test]
    fn test_default_configs() {
        let frontend = StackConfig::default_for_stack(&TechStack::FrontendWeb);
        assert_eq!(frontend.stack.name, "frontend-web");
        assert!(frontend.commands.contains_key("dev"));
        assert!(frontend.commands.contains_key("test"));
        assert_eq!(frontend.tools.package_manager, Some("npm".to_string()));

        let rust = StackConfig::default_for_stack(&TechStack::RustCli);
        assert_eq!(rust.stack.name, "rust-cli");
        assert!(rust.commands.contains_key("build"));
        assert_eq!(rust.tools.package_manager, Some("cargo".to_string()));
    }

    #[test]
    fn test_stack_config_serialization() {
        let config = StackConfig::default_for_stack(&TechStack::RustCli);
        let toml_str = toml::to_string_pretty(&config).unwrap();
        assert!(toml_str.contains("[stack]"));
        assert!(toml_str.contains("[commands]"));
        assert!(toml_str.contains("[tools]"));
    }

    #[test]
    fn test_tech_stack_all() {
        let stacks = TechStack::all();
        assert_eq!(stacks.len(), 5);
    }
}
