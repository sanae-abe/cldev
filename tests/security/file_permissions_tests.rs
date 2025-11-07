//! File permissions validation tests
//!
//! These tests verify that configuration file permissions are
//! properly validated and enforced (600 / rw-------).

use cldev::core::security::{check_file_permissions, set_secure_permissions, SecurityError};
use std::fs::{self, File};
use std::io::Write;
use tempfile::TempDir;

/// Test that correctly permissioned files pass validation (Unix only)
#[test]
#[cfg(unix)]
fn test_correct_permissions() {
    use std::os::unix::fs::PermissionsExt;

    let temp_dir = TempDir::new().unwrap();
    let config_file = temp_dir.path().join("config.toml");

    // Create file with correct permissions (0o600)
    let mut file = File::create(&config_file).unwrap();
    writeln!(file, "language = \"en\"").unwrap();

    let permissions = fs::Permissions::from_mode(0o600);
    fs::set_permissions(&config_file, permissions).unwrap();

    // Validation should pass
    let result = check_file_permissions(&config_file);
    assert!(
        result.is_ok(),
        "File with correct permissions (600) should pass"
    );
}

/// Test that incorrectly permissioned files fail validation (Unix only)
#[test]
#[cfg(unix)]
fn test_incorrect_permissions_644() {
    use std::os::unix::fs::PermissionsExt;

    let temp_dir = TempDir::new().unwrap();
    let config_file = temp_dir.path().join("config.toml");

    // Create file with world-readable permissions (0o644)
    let mut file = File::create(&config_file).unwrap();
    writeln!(file, "language = \"en\"").unwrap();

    let permissions = fs::Permissions::from_mode(0o644);
    fs::set_permissions(&config_file, permissions).unwrap();

    // Validation should fail
    let result = check_file_permissions(&config_file);
    assert!(
        result.is_err(),
        "File with incorrect permissions (644) should fail"
    );
    assert!(
        matches!(result, Err(SecurityError::InvalidPermissions { .. })),
        "Should return InvalidPermissions error"
    );
}

/// Test various insecure permission combinations (Unix only)
#[test]
#[cfg(unix)]
fn test_various_insecure_permissions() {
    use std::os::unix::fs::PermissionsExt;

    let temp_dir = TempDir::new().unwrap();

    let insecure_permissions = vec![
        (0o644, "rw-r--r-- (world-readable)"),
        (0o664, "rw-rw-r-- (group-writable, world-readable)"),
        (0o666, "rw-rw-rw- (world-writable)"),
        (0o640, "rw-r----- (group-readable)"),
        (0o660, "rw-rw---- (group-writable)"),
        (0o700, "rwx------ (owner-executable)"),
        (0o755, "rwxr-xr-x (world-executable)"),
        (0o777, "rwxrwxrwx (all permissions)"),
    ];

    for (mode, description) in insecure_permissions {
        let config_file = temp_dir.path().join(format!("config_{:o}.toml", mode));

        // Create file with the specified permissions
        let mut file = File::create(&config_file).unwrap();
        writeln!(file, "test = true").unwrap();

        let permissions = fs::Permissions::from_mode(mode);
        fs::set_permissions(&config_file, permissions).unwrap();

        // Validation should fail
        let result = check_file_permissions(&config_file);
        assert!(
            result.is_err(),
            "Permissions {} should be rejected",
            description
        );

        if let Err(SecurityError::InvalidPermissions { expected, actual }) = result {
            assert_eq!(expected, "600", "Expected permissions should be 600");
            assert_eq!(
                actual,
                format!("{:o}", mode),
                "Actual permissions should match mode"
            );
        } else {
            panic!("Expected InvalidPermissions error");
        }
    }
}

/// Test set_secure_permissions function (Unix only)
#[test]
#[cfg(unix)]
fn test_set_secure_permissions() {
    use std::os::unix::fs::PermissionsExt;

    let temp_dir = TempDir::new().unwrap();
    let config_file = temp_dir.path().join("config.toml");

    // Create file with insecure permissions
    let mut file = File::create(&config_file).unwrap();
    writeln!(file, "language = \"en\"").unwrap();

    let permissions = fs::Permissions::from_mode(0o644);
    fs::set_permissions(&config_file, permissions).unwrap();

    // Verify file has incorrect permissions
    let result = check_file_permissions(&config_file);
    assert!(result.is_err(), "File should have incorrect permissions");

    // Fix permissions
    let result = set_secure_permissions(&config_file);
    assert!(result.is_ok(), "Setting permissions should succeed");

    // Verify permissions were corrected
    let result = check_file_permissions(&config_file);
    assert!(result.is_ok(), "File should now have correct permissions");

    // Double-check the actual file mode
    let metadata = fs::metadata(&config_file).unwrap();
    let mode = metadata.permissions().mode() & 0o777;
    assert_eq!(mode, 0o600, "File permissions should be exactly 600");
}

/// Test permission check on non-existent file
#[test]
fn test_nonexistent_file() {
    let temp_dir = TempDir::new().unwrap();
    let nonexistent = temp_dir.path().join("does_not_exist.toml");

    let result = check_file_permissions(&nonexistent);
    assert!(result.is_err(), "Non-existent file should return error");
    assert!(
        matches!(result, Err(SecurityError::IoError(_))),
        "Should return IoError for non-existent file"
    );
}

/// Test permission check on directory
#[test]
#[cfg(unix)]
fn test_directory_permissions() {
    use std::os::unix::fs::PermissionsExt;

    let temp_dir = TempDir::new().unwrap();
    let sub_dir = temp_dir.path().join("subdir");
    fs::create_dir(&sub_dir).unwrap();

    // Set directory permissions to 700
    let permissions = fs::Permissions::from_mode(0o700);
    fs::set_permissions(&sub_dir, permissions).unwrap();

    // Check permissions on directory
    let result = check_file_permissions(&sub_dir);
    // Should fail because directory permissions (700) != file permissions (600)
    assert!(
        result.is_err(),
        "Directory with 700 permissions should fail file permission check"
    );
}

/// Test that the error message contains useful information
#[test]
#[cfg(unix)]
fn test_error_message_format() {
    use std::os::unix::fs::PermissionsExt;

    let temp_dir = TempDir::new().unwrap();
    let config_file = temp_dir.path().join("config.toml");

    // Create file with 644 permissions
    let mut file = File::create(&config_file).unwrap();
    writeln!(file, "test = true").unwrap();

    let permissions = fs::Permissions::from_mode(0o644);
    fs::set_permissions(&config_file, permissions).unwrap();

    // Check error message
    let result = check_file_permissions(&config_file);
    assert!(result.is_err());

    let error = result.unwrap_err();
    let error_msg = error.to_string();

    assert!(
        error_msg.contains("600"),
        "Error message should contain expected permissions"
    );
    assert!(
        error_msg.contains("644"),
        "Error message should contain actual permissions"
    );
}

/// Test set_secure_permissions on non-existent file
#[test]
fn test_set_permissions_nonexistent() {
    let temp_dir = TempDir::new().unwrap();
    let nonexistent = temp_dir.path().join("does_not_exist.toml");

    let result = set_secure_permissions(&nonexistent);
    assert!(
        result.is_err(),
        "Setting permissions on non-existent file should fail"
    );
}

/// Test permission preservation after file write
#[test]
#[cfg(unix)]
fn test_permissions_after_write() {
    use std::os::unix::fs::PermissionsExt;

    let temp_dir = TempDir::new().unwrap();
    let config_file = temp_dir.path().join("config.toml");

    // Create file and set secure permissions
    let mut file = File::create(&config_file).unwrap();
    writeln!(file, "version = 1").unwrap();
    drop(file);

    set_secure_permissions(&config_file).unwrap();

    // Verify permissions
    check_file_permissions(&config_file).unwrap();

    // Write more data to the file
    let mut file = fs::OpenOptions::new()
        .append(true)
        .open(&config_file)
        .unwrap();
    writeln!(file, "language = \"en\"").unwrap();
    drop(file);

    // Permissions should still be correct
    let result = check_file_permissions(&config_file);
    assert!(
        result.is_ok(),
        "Permissions should be preserved after write"
    );

    let metadata = fs::metadata(&config_file).unwrap();
    let mode = metadata.permissions().mode() & 0o777;
    assert_eq!(mode, 0o600, "Permissions should remain 600 after write");
}

/// Test umask doesn't affect set_secure_permissions
#[test]
#[cfg(unix)]
fn test_set_permissions_ignores_umask() {
    use std::os::unix::fs::PermissionsExt;

    let temp_dir = TempDir::new().unwrap();
    let config_file = temp_dir.path().join("config.toml");

    // Create file (umask might affect initial permissions)
    File::create(&config_file).unwrap();

    // Set secure permissions explicitly
    set_secure_permissions(&config_file).unwrap();

    // Verify permissions are exactly 600 regardless of umask
    let metadata = fs::metadata(&config_file).unwrap();
    let mode = metadata.permissions().mode() & 0o777;
    assert_eq!(
        mode, 0o600,
        "Permissions should be exactly 600 regardless of umask"
    );
}

/// Test permissions on symlink (Unix only)
#[test]
#[cfg(unix)]
fn test_symlink_permissions() {
    use std::os::unix::fs::{symlink, PermissionsExt};

    let temp_dir = TempDir::new().unwrap();

    // Create target file with correct permissions
    let target_file = temp_dir.path().join("target.toml");
    let mut file = File::create(&target_file).unwrap();
    writeln!(file, "test = true").unwrap();
    drop(file);

    let permissions = fs::Permissions::from_mode(0o600);
    fs::set_permissions(&target_file, permissions).unwrap();

    // Create symlink
    let symlink_path = temp_dir.path().join("link.toml");
    symlink(&target_file, &symlink_path).unwrap();

    // Check permissions through symlink (should follow symlink to target)
    let result = check_file_permissions(&symlink_path);
    assert!(
        result.is_ok(),
        "Symlink should resolve to target with correct permissions"
    );
}

/// Windows-specific test: permissions always pass
#[test]
#[cfg(windows)]
fn test_windows_permissions_always_pass() {
    let temp_dir = TempDir::new().unwrap();
    let config_file = temp_dir.path().join("config.toml");

    // Create file
    let mut file = File::create(&config_file).unwrap();
    writeln!(file, "language = \"en\"").unwrap();

    // On Windows, permission checks should always pass
    let result = check_file_permissions(&config_file);
    assert!(
        result.is_ok(),
        "Windows permission checks should always pass"
    );
}

/// Benchmark: Permission check performance
#[test]
#[cfg(unix)]
fn test_permission_check_performance() {
    use std::os::unix::fs::PermissionsExt;

    let temp_dir = TempDir::new().unwrap();
    let config_file = temp_dir.path().join("config.toml");

    // Create file with correct permissions
    File::create(&config_file).unwrap();
    let permissions = fs::Permissions::from_mode(0o600);
    fs::set_permissions(&config_file, permissions).unwrap();

    // Benchmark permission checks
    let iterations = 1000;
    let start = std::time::Instant::now();

    for _ in 0..iterations {
        let _ = check_file_permissions(&config_file);
    }

    let duration = start.elapsed();
    let avg_microseconds = duration.as_micros() / iterations;

    // Permission check should be fast (< 100 microseconds average)
    assert!(
        avg_microseconds < 100,
        "Permission check too slow: {} microseconds average",
        avg_microseconds
    );
}
