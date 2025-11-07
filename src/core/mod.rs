//! Core functionality modules for cldev CLI
//!
//! This module contains the foundational components for configuration management,
//! security, and shared utilities.

#![allow(unused_imports)]

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
pub use git_utils::GitUtils;
pub use project_config::ProjectConfig;
pub use project_detector::{ProjectDetector, ProjectType};
pub use session_recorder::{LearningSession, LearningSessionBuilder};
pub use stack_config::{StackConfig, TechStack};
