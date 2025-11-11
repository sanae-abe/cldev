//! cldev - Unified CLI for Claude Code development environment
//!
//! This library provides the core functionality for the cldev CLI tool,
//! including configuration management, security features, and shared utilities.

pub mod cli;
pub mod core;

// Re-export commonly used types for convenience
// Use `crate::core` instead of `core` to avoid ambiguity with Rust's built-in `core` crate (MSRV 1.70)
pub use crate::core::{
    config::{Config, ConfigVersion},
    error::{CldevError, Result},
    i18n::{I18n, Language, MessageCatalog},
    security::{SecurePath, SecurityError, SecurityResult},
};
