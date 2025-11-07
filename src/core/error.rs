#![allow(dead_code)]

use std::io;
use thiserror::Error;

/// Custom error types for cldev
#[derive(Error, Debug)]
pub enum CldevError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("TOML parsing error: {0}")]
    TomlParse(#[from] toml::de::Error),

    #[error("TOML serialization error: {0}")]
    TomlSerialize(#[from] toml::ser::Error),

    #[error("Editor error: {0}")]
    Editor(String),

    #[error("Environment variable error: {0}")]
    Env(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Command execution error: {0}")]
    Command(String),

    #[error("Git error: {0}")]
    Git(String),

    #[error("Dialog error: {0}")]
    Dialog(String),
}

/// Result type alias for cldev operations
pub type Result<T> = std::result::Result<T, CldevError>;

impl CldevError {
    /// Create a new configuration error
    pub fn config<S: Into<String>>(msg: S) -> Self {
        CldevError::Config(msg.into())
    }

    /// Create a new editor error
    pub fn editor<S: Into<String>>(msg: S) -> Self {
        CldevError::Editor(msg.into())
    }

    /// Create a new validation error
    pub fn validation<S: Into<String>>(msg: S) -> Self {
        CldevError::Validation(msg.into())
    }

    /// Create a new command error
    pub fn command<S: Into<String>>(msg: S) -> Self {
        CldevError::Command(msg.into())
    }

    /// Create a new IO error with custom message
    pub fn io<S: Into<String>>(msg: S) -> Self {
        CldevError::Io(std::io::Error::new(std::io::ErrorKind::Other, msg.into()))
    }

    /// Create a new security error
    pub fn security<S: Into<String>>(msg: S) -> Self {
        CldevError::Config(format!("Security: {}", msg.into()))
    }

    /// Create a new Git error
    pub fn git<S: Into<String>>(msg: S) -> Self {
        CldevError::Git(msg.into())
    }
}

// Implement From for dialoguer::Error
impl From<dialoguer::Error> for CldevError {
    fn from(err: dialoguer::Error) -> Self {
        CldevError::Dialog(err.to_string())
    }
}

// Implement From for serde_json::Error
impl From<serde_json::Error> for CldevError {
    fn from(err: serde_json::Error) -> Self {
        CldevError::Config(format!("JSON error: {}", err))
    }
}

// Implement From for git2::Error
impl From<git2::Error> for CldevError {
    fn from(err: git2::Error) -> Self {
        CldevError::Git(err.to_string())
    }
}
