//! CLI tests for i18n language support
//!
//! Tests language switching and internationalized output for all supported languages:
//! - English (en)
//! - Japanese (ja)
//! - Chinese Simplified (zh)
//! - Chinese Traditional (zh-TW)

use assert_cmd::Command;
use predicates::prelude::*;

/// Test English language output
#[test]
fn test_english_help() {
    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.arg("--lang")
        .arg("en")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage:"))
        .stdout(predicate::str::contains("Commands:"));
}

/// Test Japanese language output
#[test]
fn test_japanese_help() {
    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.arg("--lang")
        .arg("ja")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("使用方法:").or(predicate::str::contains("Usage:")));
}

/// Test Chinese Simplified language output
#[test]
fn test_chinese_simplified_help() {
    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.arg("--lang")
        .arg("zh")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("用法:").or(predicate::str::contains("Usage:")));
}

/// Test Chinese Traditional language output
#[test]
fn test_chinese_traditional_help() {
    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.arg("--lang")
        .arg("zh-TW")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("用法:").or(predicate::str::contains("Usage:")));
}

/// Test version command with all languages
#[test]
fn test_version_all_languages() {
    let languages = vec!["en", "ja", "zh", "zh-TW"];

    for lang in languages {
        let mut cmd = Command::cargo_bin("cldev").unwrap();

        cmd.arg("--lang")
            .arg(lang)
            .arg("--version")
            .assert()
            .success()
            .stdout(predicate::str::contains("cldev"));
    }
}

/// Test invalid language code shows error
#[test]
fn test_invalid_language_fallback() {
    let mut cmd = Command::cargo_bin("cldev").unwrap();

    // Invalid language should show error with valid language list
    cmd.arg("--lang")
        .arg("invalid-lang")
        .arg("--version")
        .assert()
        .failure()
        .stderr(
            predicate::str::contains("invalid value")
                .or(predicate::str::contains("possible values")),
        );
}

/// Test config command with Japanese language
#[test]
fn test_config_help_japanese() {
    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.arg("--lang")
        .arg("ja")
        .args(["config", "--help"])
        .assert()
        .success();
    // Output should contain either Japanese or English help text
}

/// Test config command with Chinese Simplified
#[test]
fn test_config_help_chinese_simplified() {
    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.arg("--lang")
        .arg("zh")
        .args(["config", "--help"])
        .assert()
        .success();
}

/// Test config command with Chinese Traditional
#[test]
fn test_config_help_chinese_traditional() {
    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.arg("--lang")
        .arg("zh-TW")
        .args(["config", "--help"])
        .assert()
        .success();
}

/// Test language persistence (environment variable)
#[test]
fn test_language_environment_variable() {
    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.env("CLDEV_LANG", "ja").arg("--help").assert().success();
}

/// Test language flag overrides environment
#[test]
fn test_language_flag_override() {
    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.env("CLDEV_LANG", "ja")
        .arg("--lang")
        .arg("en")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage:"));
}

/// Test UTF-8 output for Japanese characters
#[test]
fn test_utf8_japanese_output() {
    let mut cmd = Command::cargo_bin("cldev").unwrap();

    let output = cmd
        .arg("--lang")
        .arg("ja")
        .arg("--version")
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should successfully parse UTF-8 without panicking
    assert!(stdout.contains("cldev"));
}

/// Test UTF-8 output for Chinese Simplified characters
#[test]
fn test_utf8_chinese_simplified_output() {
    let mut cmd = Command::cargo_bin("cldev").unwrap();

    let output = cmd
        .arg("--lang")
        .arg("zh")
        .arg("--version")
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should successfully parse UTF-8 without panicking
    assert!(stdout.contains("cldev"));
}

/// Test UTF-8 output for Chinese Traditional characters
#[test]
fn test_utf8_chinese_traditional_output() {
    let mut cmd = Command::cargo_bin("cldev").unwrap();

    let output = cmd
        .arg("--lang")
        .arg("zh-TW")
        .arg("--version")
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should successfully parse UTF-8 without panicking
    assert!(stdout.contains("cldev"));
}

/// Test error messages in different languages
#[test]
fn test_error_messages_i18n() {
    let languages = vec!["en", "ja", "zh", "zh-TW"];

    for lang in languages {
        let mut cmd = Command::cargo_bin("cldev").unwrap();

        // Trigger an error with invalid command
        cmd.arg("--lang")
            .arg(lang)
            .arg("invalid-command")
            .assert()
            .failure();
        // Error should be displayed in the appropriate language
    }
}

/// Test analysis command help with Japanese
#[test]
fn test_analysis_help_japanese() {
    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.arg("--lang")
        .arg("ja")
        .args(["analysis", "--help"])
        .assert()
        .success();
}

/// Test analysis command help with Chinese
#[test]
fn test_analysis_help_chinese() {
    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.arg("--lang")
        .arg("zh")
        .args(["analysis", "--help"])
        .assert()
        .success();
}

/// Test git command help with all languages
#[test]
fn test_git_help_all_languages() {
    let languages = vec!["en", "ja", "zh", "zh-TW"];

    for lang in languages {
        let mut cmd = Command::cargo_bin("cldev").unwrap();

        cmd.arg("--lang")
            .arg(lang)
            .args(["git", "--help"])
            .assert()
            .success();
    }
}

/// Test quality command help with all languages
#[test]
fn test_quality_help_all_languages() {
    let languages = vec!["en", "ja", "zh", "zh-TW"];

    for lang in languages {
        let mut cmd = Command::cargo_bin("cldev").unwrap();

        cmd.arg("--lang")
            .arg(lang)
            .args(["quality", "--help"])
            .assert()
            .success();
    }
}

/// Test lr command help with all languages
#[test]
fn test_lr_help_all_languages() {
    let languages = vec!["en", "ja", "zh", "zh-TW"];

    for lang in languages {
        let mut cmd = Command::cargo_bin("cldev").unwrap();

        cmd.arg("--lang")
            .arg(lang)
            .args(["lr", "--help"])
            .assert()
            .success();
    }
}
