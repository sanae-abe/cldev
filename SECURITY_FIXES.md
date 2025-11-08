# Security Vulnerability Fixes

This document describes the security vulnerabilities that have been fixed in cldev.

## Summary

Two critical/medium security vulnerabilities have been addressed:
- **VULN-02 (HIGH)**: Deploy script execution without validation
- **VULN-06 (MEDIUM)**: Learning record path traversal

## VULN-02: Deploy Script Execution Without Validation

### Severity: HIGH

### Description
The deploy command executed `deploy.sh` scripts without proper validation, allowing potential execution of malicious scripts with unsafe permissions.

### Location
- File: `src/commands/ops/deploy.rs`
- Lines: 284-291, 316-321 (original)

### Fix Implemented

Added `validate_deploy_script()` function that performs:

1. **Permission Checks (Unix/Linux/macOS)**:
   - Rejects world-writable scripts (mode & 0o002)
   - Warns about group-writable scripts (mode & 0o020)
   - Error message: "deploy.sh is world-writable. Fix with: chmod 644 deploy.sh"

2. **User Confirmation**:
   - Requires explicit user confirmation before execution
   - Shows script path for review
   - Default answer is "No" for safety
   - Allows user to cancel execution

### Code Changes

```rust
/// Validate deploy script before execution
fn validate_deploy_script(script_path: &std::path::Path) -> Result<()> {
    // Check file permissions on Unix systems
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        use std::fs;

        let metadata = fs::metadata(script_path)?;
        let mode = metadata.permissions().mode();

        // Check if world-writable (dangerous)
        if mode & 0o002 != 0 {
            return Err(CldevError::security(
                "deploy.sh is world-writable. Fix with: chmod 644 deploy.sh"
            ));
        }

        // Check if group-writable (warning but allow)
        if mode & 0o020 != 0 {
            println!(
                "{}",
                "⚠️  Warning: deploy.sh is group-writable. Consider: chmod 644 deploy.sh"
                    .yellow()
            );
        }
    }

    // Require explicit user confirmation before executing script
    let confirmed = Confirm::new()
        .with_prompt(format!(
            "⚠️  Execute deploy.sh? Review script before confirming.\nPath: {}",
            script_path.display()
        ))
        .default(false)
        .interact()?;

    if !confirmed {
        return Err(CldevError::config("Deploy script execution cancelled by user"));
    }

    Ok(())
}
```

### Applied In
- `deploy_web_app()` - Line 331
- `deploy_rust_app()` - Line 367

### Impact
- Prevents execution of world-writable malicious scripts
- Requires user awareness before script execution
- Maintains security on multi-user systems

---

## VULN-06: Learning Record Path Traversal

### Severity: MEDIUM

### Description
Learning session topic names were not validated, allowing potential path traversal attacks via malicious topic names containing `..`, `/`, `\`, or shell metacharacters.

### Location
- File: `src/core/session_recorder.rs`
- Functions: `LearningSession::new()`, `LearningSession::save()`

### Fix Implemented

Added two security functions:

1. **`validate_session_topic()`**: Comprehensive input validation
   - Length limit: 200 characters
   - Empty check
   - Path traversal prevention: rejects `..`, `/`, `\`
   - Shell injection prevention: rejects dangerous chars `$`, `` ` ``, `|`, `&`, `;`, `\n`, `\0`, `<`, `>`, `*`, `?`, `"`, `'`

2. **`sanitize_filename()`**: Safe filename generation
   - Allows: alphanumeric, `-`, `_`, space
   - Replaces unsafe characters with `_`
   - Trims whitespace

### Code Changes

```rust
/// Validate session topic/description to prevent path traversal attacks
fn validate_session_topic(topic: &str) -> Result<()> {
    // Length check
    if topic.len() > 200 {
        return Err(CldevError::validation(
            "Topic name too long (max 200 characters)",
        ));
    }

    if topic.is_empty() {
        return Err(CldevError::validation("Topic name cannot be empty"));
    }

    // Path traversal checks
    if topic.contains("..") || topic.contains('/') || topic.contains('\\') {
        return Err(CldevError::validation(
            "Topic name cannot contain path separators or '..'",
        ));
    }

    // Dangerous character checks
    let dangerous = ['$', '`', '|', '&', ';', '\n', '\0', '<', '>', '*', '?', '"', '\''];
    if topic.chars().any(|c| dangerous.contains(&c)) {
        return Err(CldevError::validation(
            "Topic name contains invalid characters",
        ));
    }

    Ok(())
}

/// Sanitize filename to ensure safe file system operations
fn sanitize_filename(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' || c == ' ' {
                c
            } else {
                '_'
            }
        })
        .collect::<String>()
        .trim()
        .to_string()
}
```

### Applied In
- `LearningSession::save()` - Lines 262-263 (validation)
- `LearningSession::save()` - Lines 271-279 (sanitization)
- `LearningSession::save()` - Lines 285-305 (canonical path verification)

### Additional Safety Measures

Canonical path verification prevents symlink-based path traversal:
```rust
// Verify the final path is within sessions_dir
let canonical_sessions = sessions_dir.canonicalize()?;
// ... verification logic
if filepath.exists() && !canonical_filepath.starts_with(&canonical_sessions) {
    return Err(CldevError::security(
        "Attempted path traversal in session save",
    ));
}
```

### Impact
- Prevents directory traversal via malicious topic names
- Prevents shell injection via special characters
- Ensures all learning records stay within designated directory
- Protects against symlink-based attacks

---

## Testing

Security validation tests have been added in `tests/security_vuln_fixes_test.rs`.

To verify the fixes:
```bash
# Check that code compiles
cargo check

# Run security tests
cargo test security_tests

# Verify deploy script validation (manual test)
# 1. Create a world-writable deploy.sh
chmod 666 deploy.sh
cldev deploy --env development
# Expected: Error about world-writable script

# 2. Fix permissions and try again
chmod 644 deploy.sh
cldev deploy --env development
# Expected: Confirmation prompt before execution
```

## Recommendations

### For Users
1. **Deploy Scripts**: Ensure `deploy.sh` has restrictive permissions (644 or 600)
2. **Learning Records**: Use descriptive but simple topic names without special characters
3. **Multi-user Systems**: Be extra cautious with file permissions

### For Developers
1. Always validate user-provided input before file system operations
2. Use canonical paths to prevent symlink attacks
3. Implement defense-in-depth with multiple validation layers
4. Follow principle of least privilege for file permissions
5. Default to secure settings (e.g., confirmation dialogs default to "No")

## References
- OWASP Path Traversal: https://owasp.org/www-community/attacks/Path_Traversal
- CWE-22: Improper Limitation of a Pathname to a Restricted Directory
- CWE-78: OS Command Injection

## Version
- Fixed in: Current development branch
- Applies to: cldev v0.1.x and later
- Date: 2025-11-08
