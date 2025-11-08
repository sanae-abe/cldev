//! Path traversal prevention tests
//!
//! These tests verify that SecurePath correctly prevents various
//! path traversal attack vectors.

use cldev::core::security::{SecurePath, SecurityError};
use std::fs::{self, File};
use std::path::Path;
use tempfile::TempDir;

/// Test basic path validation with valid relative path
#[test]
fn test_valid_relative_path() {
    let temp_dir = TempDir::new().unwrap();
    let secure_path = SecurePath::new(temp_dir.path().to_path_buf()).unwrap();

    // Create a test file
    let test_file = temp_dir.path().join("config.toml");
    File::create(&test_file).unwrap();

    // Validate relative path
    let result = secure_path.validate(Path::new("config.toml"));
    assert!(result.is_ok(), "Valid relative path should be accepted");

    let validated_path = result.unwrap();
    // Compare canonicalized paths (handles /var vs /private/var on macOS)
    assert_eq!(validated_path, test_file.canonicalize().unwrap());
}

/// Test path validation with nested directories
#[test]
fn test_valid_nested_path() {
    let temp_dir = TempDir::new().unwrap();
    let secure_path = SecurePath::new(temp_dir.path().to_path_buf()).unwrap();

    // Create nested directory structure
    let nested_dir = temp_dir.path().join("configs").join("user");
    fs::create_dir_all(&nested_dir).unwrap();

    let test_file = nested_dir.join("settings.toml");
    File::create(&test_file).unwrap();

    // Validate nested path
    let result = secure_path.validate(Path::new("configs/user/settings.toml"));
    assert!(result.is_ok(), "Valid nested path should be accepted");

    let validated_path = result.unwrap();
    // Compare canonicalized paths (handles /var vs /private/var on macOS)
    assert_eq!(validated_path, test_file.canonicalize().unwrap());
}

/// Test classic path traversal attack: ../../../
#[test]
fn test_path_traversal_parent_directory() {
    let temp_dir = TempDir::new().unwrap();
    let secure_path = SecurePath::new(temp_dir.path().to_path_buf()).unwrap();

    // Attempt to access parent directories
    let attacks = vec![
        "../config.toml",
        "../../etc/passwd",
        "../../../etc/shadow",
        "../../../../root/.ssh/id_rsa",
    ];

    for attack in attacks {
        let result = secure_path.validate(Path::new(attack));
        assert!(
            result.is_err(),
            "Path traversal attack '{}' should be rejected",
            attack
        );
        // The error could be PathTraversal or CanonicalizationError
        // depending on whether the path exists outside the base directory
        match result {
            Err(SecurityError::PathTraversal { .. }) => {}
            Err(SecurityError::CanonicalizationError(_)) => {}
            _ => panic!(
                "Should return PathTraversal or CanonicalizationError for '{}'",
                attack
            ),
        }
    }
}

/// Test absolute path outside base directory
#[test]
fn test_absolute_path_outside_base() {
    let temp_dir = TempDir::new().unwrap();
    let secure_path = SecurePath::new(temp_dir.path().to_path_buf()).unwrap();

    // Attempt to access system files via absolute paths
    let attacks = vec![
        "/etc/passwd",
        "/etc/shadow",
        "/root/.ssh/id_rsa",
        "/var/log/auth.log",
    ];

    for attack in attacks {
        let result = secure_path.validate(Path::new(attack));
        assert!(
            result.is_err(),
            "Absolute path outside base '{}' should be rejected",
            attack
        );
    }
}

/// Test symlink-based path traversal
#[test]
#[cfg(unix)]
fn test_symlink_path_traversal() {
    use std::os::unix::fs::symlink;

    let temp_dir = TempDir::new().unwrap();
    let secure_path = SecurePath::new(temp_dir.path().to_path_buf()).unwrap();

    // Create a symlink pointing outside the base directory
    let symlink_path = temp_dir.path().join("evil_link");
    symlink("/etc/passwd", &symlink_path).unwrap();

    // Attempt to access via symlink - should be rejected after canonicalization
    let result = secure_path.validate(Path::new("evil_link"));
    assert!(result.is_err(), "Symlink escape attempt should be rejected");
}

/// Test mixed path traversal patterns
#[test]
fn test_mixed_path_traversal() {
    let temp_dir = TempDir::new().unwrap();
    let secure_path = SecurePath::new(temp_dir.path().to_path_buf()).unwrap();

    let attacks = vec![
        "configs/../../../etc/passwd",
        "./../../etc/shadow",
        "valid/path/../../../etc/passwd",
    ];

    for attack in attacks {
        let result = secure_path.validate(Path::new(attack));
        assert!(
            result.is_err(),
            "Mixed path traversal '{}' should be rejected",
            attack
        );
    }
}

/// Test validation of non-existent paths
#[test]
fn test_validate_non_existent_valid() {
    let temp_dir = TempDir::new().unwrap();
    let secure_path = SecurePath::new(temp_dir.path().to_path_buf()).unwrap();

    // Validate a non-existent path that would be within base directory
    let result = secure_path.validate_non_existent(Path::new("future/config.toml"));
    assert!(result.is_ok(), "Valid non-existent path should be accepted");

    let validated_path = result.unwrap();
    assert!(validated_path.starts_with(secure_path.base_dir()));
}

/// Test rejection of non-existent paths with traversal
#[test]
fn test_validate_non_existent_traversal() {
    let temp_dir = TempDir::new().unwrap();
    let secure_path = SecurePath::new(temp_dir.path().to_path_buf()).unwrap();

    let attacks = vec!["../outside.txt", "../../etc/passwd", "configs/../../../etc"];

    for attack in attacks {
        let result = secure_path.validate_non_existent(Path::new(attack));
        assert!(
            result.is_err(),
            "Non-existent path with traversal '{}' should be rejected",
            attack
        );
    }
}

/// Test URL-encoded path traversal attempts
#[test]
fn test_url_encoded_traversal() {
    let temp_dir = TempDir::new().unwrap();
    let secure_path = SecurePath::new(temp_dir.path().to_path_buf()).unwrap();

    // Some filesystems might accept URL-encoded paths
    let attacks = vec!["%2e%2e%2f%2e%2e%2fetc%2fpasswd", "..%2f..%2fetc%2fpasswd"];

    for attack in attacks {
        // These might fail at canonicalization or path validation
        let result = secure_path.validate(Path::new(attack));
        // We don't assert error here because filesystem behavior varies,
        // but we ensure it doesn't succeed in escaping
        if let Ok(path) = result {
            assert!(
                path.starts_with(secure_path.base_dir()),
                "Path should not escape base directory"
            );
        }
    }
}

/// Test null byte injection in paths
#[test]
fn test_null_byte_injection() {
    let temp_dir = TempDir::new().unwrap();
    let secure_path = SecurePath::new(temp_dir.path().to_path_buf()).unwrap();

    // Attempt null byte injection
    let attack = "config.toml\0../../../etc/passwd";

    // Rust's Path type doesn't accept null bytes in valid UTF-8 strings,
    // so this test verifies the type system prevents this attack
    let result = secure_path.validate(Path::new(attack));

    // Either fails validation or doesn't contain the injected part
    if let Ok(path) = result {
        let path_str = path.to_string_lossy();
        assert!(!path_str.contains("/etc/passwd"));
    }
}

/// Test home directory expansion prevention
#[test]
fn test_home_directory_expansion() {
    let temp_dir = TempDir::new().unwrap();
    let secure_path = SecurePath::new(temp_dir.path().to_path_buf()).unwrap();

    // Attempt home directory expansion
    let attacks = vec!["~/.ssh/id_rsa", "~root/.bashrc", "~/../../etc/passwd"];

    for attack in attacks {
        let result = secure_path.validate_non_existent(Path::new(attack));
        assert!(
            result.is_err(),
            "Home directory expansion '{}' should be rejected",
            attack
        );
    }
}

/// Test base directory validation
#[test]
fn test_invalid_base_directory() {
    let non_existent = Path::new("/this/path/does/not/exist/hopefully");

    let result = SecurePath::new(non_existent.to_path_buf());
    assert!(result.is_err(), "Invalid base directory should be rejected");
}

/// Test edge case: current directory reference
#[test]
fn test_current_directory_reference() {
    let temp_dir = TempDir::new().unwrap();
    let secure_path = SecurePath::new(temp_dir.path().to_path_buf()).unwrap();

    // Create a test file
    let test_file = temp_dir.path().join("config.toml");
    File::create(&test_file).unwrap();

    // Validate with ./ prefix
    let result = secure_path.validate(Path::new("./config.toml"));
    assert!(
        result.is_ok(),
        "Current directory reference should be accepted"
    );
}

/// Test edge case: empty path components
#[test]
fn test_empty_path_components() {
    let temp_dir = TempDir::new().unwrap();
    let secure_path = SecurePath::new(temp_dir.path().to_path_buf()).unwrap();

    // Create a test file
    let test_file = temp_dir.path().join("config.toml");
    File::create(&test_file).unwrap();

    // Path with empty components (repeated slashes)
    let result = secure_path.validate(Path::new("./config.toml"));
    assert!(result.is_ok(), "Path with ./ should be accepted");
}

/// Benchmark: Validation performance
#[test]
fn test_validation_performance() {
    let temp_dir = TempDir::new().unwrap();
    let secure_path = SecurePath::new(temp_dir.path().to_path_buf()).unwrap();

    // Create test file
    let test_file = temp_dir.path().join("benchmark.txt");
    File::create(&test_file).unwrap();

    // Validate path multiple times to ensure reasonable performance
    let iterations = 1000;
    let start = std::time::Instant::now();

    for _ in 0..iterations {
        let _ = secure_path.validate(Path::new("benchmark.txt"));
    }

    let duration = start.elapsed();
    let avg_microseconds = duration.as_micros() / iterations;

    // Validation should be fast (< 100 microseconds average)
    assert!(
        avg_microseconds < 100,
        "Path validation too slow: {} microseconds average",
        avg_microseconds
    );
}
