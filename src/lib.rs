//! cldev - Unified CLI for Claude Code development environment
//!
//! This library provides the core functionality for the cldev CLI tool,
//! including configuration management, security features, and shared utilities.

pub mod cli;
pub mod core;

// Re-export commonly used types for convenience
pub use core::{
    config::{Config, ConfigVersion},
    error::{CldevError, Result},
    i18n::{I18n, Language, MessageCatalog},
    security::{SecurePath, SecurityError, SecurityResult},
};
