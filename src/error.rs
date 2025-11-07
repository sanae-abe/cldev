use std::io;
use std::path::PathBuf;
use thiserror::Error;

/// Result type alias for cldev operations
pub type Result<T> = std::result::Result<T, CliError>;

/// Top-level error type for the cldev CLI
#[derive(Error, Debug)]
pub enum CliError {
    /// Configuration-related errors
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),

    /// Git-related errors
    #[error("Git error: {0}")]
    Git(#[from] GitError),

    /// File system errors
    #[error("File system error: {0}")]
    Io(#[from] io::Error),

    /// Command execution errors
    #[error("Command execution error: {0}")]
    Command(#[from] CommandError),

    /// Analysis errors
    #[error("Analysis error: {0}")]
    Analysis(#[from] AnalysisError),

    /// Learning record errors
    #[error("Learning record error: {0}")]
    LearningRecord(#[from] LearningRecordError),

    /// JSON parsing errors
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// TOML parsing errors
    #[error("TOML error: {0}")]
    Toml(#[from] toml::de::Error),

    /// Generic error with message
    #[error("{0}")]
    Generic(String),
}

/// Configuration-related errors
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Configuration file not found: {path}")]
    NotFound { path: PathBuf },

    #[error("Invalid configuration: {reason}")]
    Invalid { reason: String },

    #[error("Failed to parse configuration: {reason}")]
    ParseError { reason: String },

    #[error("Configuration directory error: {path}")]
    DirectoryError { path: PathBuf },

    #[error("Failed to initialize configuration: {reason}")]
    InitError { reason: String },

    #[error("Failed to update configuration: {reason}")]
    UpdateError { reason: String },
}

/// Git-related errors
#[derive(Error, Debug)]
pub enum GitError {
    #[error("Not a git repository: {path}")]
    NotRepository { path: PathBuf },

    #[error("Git command failed: {command}")]
    CommandFailed { command: String },

    #[error("Invalid branch name: {name}")]
    InvalidBranch { name: String },

    #[error("Uncommitted changes found")]
    UncommittedChanges,

    #[error("Failed to parse git output: {output}")]
    ParseError { output: String },

    #[error("Merge conflict detected")]
    MergeConflict,

    #[error("Remote repository error: {remote}")]
    RemoteError { remote: String },
}

/// Command execution errors
#[derive(Error, Debug)]
pub enum CommandError {
    #[error("Command not found: {command}")]
    NotFound { command: String },

    #[error("Command failed with exit code {code}: {command}")]
    Failed { command: String, code: i32 },

    #[error("Invalid arguments: {reason}")]
    InvalidArguments { reason: String },

    #[error("Operation cancelled by user")]
    Cancelled,

    #[error("Prerequisite not met: {requirement}")]
    PrerequisiteFailed { requirement: String },
}

/// Analysis-related errors
#[derive(Error, Debug)]
pub enum AnalysisError {
    #[error("Failed to analyze project structure: {reason}")]
    StructureError { reason: String },

    #[error("Failed to parse source file: {path}")]
    ParseError { path: PathBuf },

    #[error("Unsupported file type: {extension}")]
    UnsupportedFileType { extension: String },

    #[error("Analysis timeout: operation took too long")]
    Timeout,

    #[error("Insufficient data for analysis")]
    InsufficientData,
}

/// Learning record errors
#[derive(Error, Debug)]
pub enum LearningRecordError {
    #[error("Learning record not found: {id}")]
    NotFound { id: String },

    #[error("Failed to create learning record: {reason}")]
    CreateError { reason: String },

    #[error("Failed to update learning record: {reason}")]
    UpdateError { reason: String },

    #[error("Invalid learning record format: {reason}")]
    InvalidFormat { reason: String },

    #[error("Learning record database error: {reason}")]
    DatabaseError { reason: String },
}

impl CliError {
    /// Create a generic error from a string
    pub fn msg<S: Into<String>>(msg: S) -> Self {
        CliError::Generic(msg.into())
    }

    /// Check if the error is recoverable
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            CliError::Command(CommandError::Cancelled)
                | CliError::Command(CommandError::InvalidArguments { .. })
                | CliError::Config(ConfigError::Invalid { .. })
        )
    }

    /// Get exit code for the error
    pub fn exit_code(&self) -> i32 {
        match self {
            CliError::Command(CommandError::Cancelled) => 130, // SIGINT
            CliError::Command(CommandError::InvalidArguments { .. }) => 2,
            CliError::Config(_) => 78, // EX_CONFIG
            CliError::Io(_) => 74,     // EX_IOERR
            _ => 1,                    // General error
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = CliError::msg("test error");
        assert_eq!(err.to_string(), "test error");
    }

    #[test]
    fn test_recoverable_errors() {
        let err = CliError::Command(CommandError::Cancelled);
        assert!(err.is_recoverable());

        let err = CliError::Io(io::Error::new(io::ErrorKind::NotFound, "test"));
        assert!(!err.is_recoverable());
    }

    #[test]
    fn test_exit_codes() {
        let err = CliError::Command(CommandError::Cancelled);
        assert_eq!(err.exit_code(), 130);

        let err = CliError::Config(ConfigError::NotFound {
            path: PathBuf::from("/test"),
        });
        assert_eq!(err.exit_code(), 78);
    }
}
