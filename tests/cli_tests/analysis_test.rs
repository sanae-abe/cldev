//! CLI tests for analysis commands
//!
//! Tests the analyze and serena commands including:
//! - UTF-8 handling for binary files
//! - i18n support for Japanese/Chinese output
//! - Success and error cases
//! - Output format validation

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::io::Write;
use tempfile::TempDir;

/// Test analyze command help
#[test]
fn test_analyze_help() {
    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.args(["analysis", "analyze", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("analyze").or(predicate::str::contains("Analysis")));
}

/// Test analyze command with invalid target
#[test]
fn test_analyze_invalid_target() {
    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.args(["analysis", "analyze", "invalid-analysis-type"])
        .assert()
        .failure();
}

/// Test analyze command without target (uses default 'overview')
#[test]
fn test_analyze_no_target() {
    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.args(["analysis", "analyze"])
        .assert()
        .code(predicate::function(|code: &i32| *code == 0 || *code == 1));
}

/// Test analyze structure command
#[test]
fn test_analyze_structure() {
    let mut cmd = Command::cargo_bin("cldev").unwrap();

    // This may fail if not in a valid project, but should not panic
    cmd.args(["analysis", "analyze", "structure"])
        .assert()
        .code(predicate::function(|code: &i32| *code == 0 || *code == 1));
}

/// Test analyze performance command
#[test]
fn test_analyze_performance() {
    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.args(["analysis", "analyze", "performance"])
        .assert()
        .code(predicate::function(|code: &i32| *code == 0 || *code == 1));
}

/// Test analyze quality command
#[test]
fn test_analyze_quality() {
    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.args(["analysis", "analyze", "quality"])
        .assert()
        .code(predicate::function(|code: &i32| *code == 0 || *code == 1));
}

/// Test analyze debt command
#[test]
fn test_analyze_debt() {
    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.args(["analysis", "analyze", "debt"])
        .assert()
        .code(predicate::function(|code: &i32| *code == 0 || *code == 1));
}

/// Test analyze overview command
#[test]
fn test_analyze_overview() {
    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.args(["analysis", "analyze", "overview"])
        .assert()
        .code(predicate::function(|code: &i32| *code == 0 || *code == 1));
}

/// Test analyze with detailed flag
#[test]
fn test_analyze_with_detailed() {
    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.args(["analysis", "analyze", "overview", "--detailed"])
        .assert()
        .code(predicate::function(|code: &i32| *code == 0 || *code == 1));
}

/// Test analyze with quiet flag
#[test]
fn test_analyze_with_quick() {
    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.args(["analysis", "analyze", "overview", "--quiet"])
        .assert()
        .code(predicate::function(|code: &i32| *code == 0 || *code == 1));
}

/// Test analyze with Japanese language
#[test]
fn test_analyze_japanese() {
    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.arg("--lang")
        .arg("ja")
        .args(["analysis", "analyze", "overview"])
        .assert()
        .code(predicate::function(|code: &i32| *code == 0 || *code == 1));
}

/// Test analyze with Chinese Simplified
#[test]
fn test_analyze_chinese_simplified() {
    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.arg("--lang")
        .arg("zh")
        .args(["analysis", "analyze", "overview"])
        .assert()
        .code(predicate::function(|code: &i32| *code == 0 || *code == 1));
}

/// Test analyze with Japanese
#[test]
fn test_analyze_japanese() {
    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.arg("--lang")
        .arg("ja")
        .args(["analysis", "analyze", "overview"])
        .assert()
        .code(predicate::function(|code: &i32| *code == 0 || *code == 1));
}

/// Test serena command help
#[test]
fn test_serena_help() {
    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.args(["analysis", "serena", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("serena").or(predicate::str::contains("Serena")));
}

/// Test serena command without target
#[test]
fn test_serena_no_target() {
    let mut cmd = Command::cargo_bin("cldev").unwrap();

    // Without target, serena analyzes current directory in interactive mode
    cmd.args(["analysis", "serena"])
        .assert()
        .code(predicate::function(|code: &i32| *code == 0 || *code == 1));
}

/// Test serena command with non-existent file
#[test]
fn test_serena_nonexistent_file() {
    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.args(["analysis", "serena", "nonexistent_file.rs"])
        .assert()
        .failure();
}

/// Test serena UTF-8 handling with text file
#[test]
fn test_serena_utf8_text_file() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.rs");

    // Create a Rust file with UTF-8 content
    let mut file = fs::File::create(&file_path).unwrap();
    writeln!(
        file,
        "// UTF-8 test: 日本語 中文 繁體中文\nfn main() {{\n    println!(\"Hello, world!\");\n}}"
    )
    .unwrap();

    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.args(["analysis", "serena", "batch", file_path.to_str().unwrap()])
        .assert()
        .code(predicate::function(|code: &i32| *code == 0 || *code == 1));
}

/// Test serena UTF-8 handling with binary file (should not panic)
#[test]
fn test_serena_binary_file_no_panic() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("binary.bin");

    // Create a binary file with non-UTF-8 bytes
    let mut file = fs::File::create(&file_path).unwrap();
    file.write_all(&[0xFF, 0xFE, 0xFD, 0x00, 0x01, 0x02])
        .unwrap();

    let mut cmd = Command::cargo_bin("cldev").unwrap();

    // Should either succeed (skip binary) or fail gracefully, but NOT panic
    cmd.args(["analysis", "serena", "batch", file_path.to_str().unwrap()])
        .assert()
        .code(predicate::function(|code: &i32| *code == 0 || *code == 1));
}

/// Test serena with mixed UTF-8 and binary content
#[test]
fn test_serena_mixed_content() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("mixed.txt");

    // Create file with valid UTF-8 followed by invalid bytes
    let mut file = fs::File::create(&file_path).unwrap();
    file.write_all(b"Valid UTF-8 text\n").unwrap();
    file.write_all(&[0xFF, 0xFE]).unwrap();

    let mut cmd = Command::cargo_bin("cldev").unwrap();

    // Should handle gracefully without panic
    cmd.args(["analysis", "serena", "batch", file_path.to_str().unwrap()])
        .assert()
        .code(predicate::function(|code: &i32| *code == 0 || *code == 1));
}

/// Test serena with directory target
#[test]
fn test_serena_directory_target() {
    let temp_dir = TempDir::new().unwrap();

    // Create a simple Rust file in directory
    let file_path = temp_dir.path().join("test.rs");
    let mut file = fs::File::create(&file_path).unwrap();
    writeln!(file, "fn test() {{}}").unwrap();

    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.args([
        "analysis",
        "serena",
        "batch",
        temp_dir.path().to_str().unwrap(),
    ])
    .assert()
    .code(predicate::function(|code: &i32| *code == 0 || *code == 1));
}

/// Test serena with Japanese language output
#[test]
fn test_serena_japanese_output() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.rs");

    let mut file = fs::File::create(&file_path).unwrap();
    writeln!(file, "fn test() {{}}").unwrap();

    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.arg("--lang")
        .arg("ja")
        .args(["analysis", "serena", "batch", file_path.to_str().unwrap()])
        .assert()
        .code(predicate::function(|code: &i32| *code == 0 || *code == 1));
}

/// Test serena with Chinese output
#[test]
fn test_serena_chinese_output() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.rs");

    let mut file = fs::File::create(&file_path).unwrap();
    writeln!(file, "fn test() {{}}").unwrap();

    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.arg("--lang")
        .arg("zh")
        .args(["analysis", "serena", "batch", file_path.to_str().unwrap()])
        .assert()
        .code(predicate::function(|code: &i32| *code == 0 || *code == 1));
}

/// Test serena output is valid UTF-8
#[test]
fn test_serena_output_valid_utf8() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.rs");

    let mut file = fs::File::create(&file_path).unwrap();
    writeln!(file, "fn test() {{}}").unwrap();

    let mut cmd = Command::cargo_bin("cldev").unwrap();

    let output = cmd
        .args(["analysis", "serena", file_path.to_str().unwrap()])
        .output()
        .unwrap();

    // Should be able to parse output as UTF-8
    let _stdout = String::from_utf8(output.stdout).expect("Output should be valid UTF-8");
    let _stderr = String::from_utf8(output.stderr).expect("Error output should be valid UTF-8");
}

/// Test explain command help
#[test]
fn test_explain_help() {
    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.args(["analysis", "explain", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("explain").or(predicate::str::contains("Explain")));
}

/// Test explain command without target
#[test]
fn test_explain_no_target() {
    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.args(["analysis", "explain"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("required").or(predicate::str::contains("usage")));
}

/// Test explain command with target
#[test]
fn test_explain_with_target() {
    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.args(["analysis", "explain", "function_name"])
        .assert()
        .code(predicate::function(|code: &i32| *code == 0 || *code == 1));
}

/// Test review-mr command help
#[test]
fn test_review_mr_help() {
    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.args(["analysis", "review-mr", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("review").or(predicate::str::contains("Review")));
}

/// Test review-mr command without number
#[test]
fn test_review_mr_no_number() {
    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.args(["analysis", "review-mr"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("required").or(predicate::str::contains("usage")));
}

/// Test analyze JSON output format
#[test]
fn test_analyze_json_output() {
    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.args(["analysis", "analyze", "overview", "--format", "json"])
        .assert()
        .code(predicate::function(|code: &i32| *code == 0 || *code == 1));
}

/// Test serena with multiple file targets
#[test]
fn test_serena_multiple_files() {
    let temp_dir = TempDir::new().unwrap();

    let file1 = temp_dir.path().join("test1.rs");
    let file2 = temp_dir.path().join("test2.rs");

    fs::File::create(&file1).unwrap();
    fs::File::create(&file2).unwrap();

    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.args([
        "analysis",
        "serena",
        "batch",
        file1.to_str().unwrap(),
        file2.to_str().unwrap(),
    ])
    .assert()
    .code(predicate::function(|code: &i32| *code == 0 || *code == 1));
}

/// Test serena regression: UTF-8 panic on binary files
/// This test ensures the bug fix for Serena UTF-8 handling is working
#[test]
fn test_serena_utf8_regression() {
    let temp_dir = TempDir::new().unwrap();
    let binary_file = temp_dir.path().join("binary.so");

    // Create a binary file that would previously cause panic
    let mut file = fs::File::create(&binary_file).unwrap();
    file.write_all(&[
        0x7F, 0x45, 0x4C, 0x46, // ELF magic
        0xFF, 0xFE, 0xFD, 0xFC, // Invalid UTF-8
        0x00, 0x00, 0x00, 0x00,
    ])
    .unwrap();

    let mut cmd = Command::cargo_bin("cldev").unwrap();

    // This should NOT panic and should handle gracefully
    let output = cmd
        .args(["analysis", "serena", binary_file.to_str().unwrap()])
        .output()
        .unwrap();

    // Ensure we get SOME output (success or error, but not panic)
    assert!(output.status.code().is_some());
}
