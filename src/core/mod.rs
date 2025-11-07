//! Core functionality modules for cldev CLI
//!
//! This module contains the foundational components for configuration management,
//! security, and shared utilities.

pub mod config;
pub mod error;
pub mod git_utils;
pub mod i18n;
pub mod project_config;
pub mod project_detector;
pub mod security;
pub mod session_recorder;
pub mod stack_config;

pub use config::{Config, ConfigVersion, HierarchicalConfig};
pub use error::{CldevError, Result};
pub use git_utils::{check_cli_for_remote, check_gh_cli, check_glab_cli, GitUtils, RemoteType};
pub use i18n::{I18n, Language, MessageCatalog};
pub use project_config::ProjectConfig;
pub use project_detector::{ProjectDetector, ProjectType};
pub use security::{SecurePath, SecurityError, SecurityResult};
pub use session_recorder::{LearningSession, LearningSessionBuilder};
pub use stack_config::{StackConfig, TechStack};
