//! CLI tests for version command
//!
//! Tests the --version option and version output format.

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_version_flag() {
    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("cldev"))
        .stdout(predicate::str::is_match(r"\d+\.\d+\.\d+").unwrap());
}

#[test]
fn test_short_version_flag() {
    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.arg("-V")
        .assert()
        .success()
        .stdout(predicate::str::contains("cldev"));
}

#[test]
fn test_version_format() {
    let mut cmd = Command::cargo_bin("cldev").unwrap();

    // Version should follow semantic versioning format
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::is_match(r"cldev \d+\.\d+\.\d+").unwrap());
}

#[test]
fn test_version_no_extra_output() {
    let mut cmd = Command::cargo_bin("cldev").unwrap();

    let output = cmd.arg("--version").output().unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();

    // Version output should be a single line
    assert_eq!(stdout.lines().count(), 1);
}

#[test]
fn test_version_consistency() {
    let mut cmd1 = Command::cargo_bin("cldev").unwrap();
    let mut cmd2 = Command::cargo_bin("cldev").unwrap();

    let output1 = cmd1.arg("--version").output().unwrap();
    let output2 = cmd2.arg("-V").output().unwrap();

    // Both flags should produce the same output
    assert_eq!(output1.stdout, output2.stdout);
}
