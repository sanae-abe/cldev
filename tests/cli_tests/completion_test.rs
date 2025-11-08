//! CLI tests for shell completion generation
//!
//! Tests completion generation for various shells.

use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;

#[test]
fn test_completion_bash() {
    let mut cmd = cargo_bin_cmd!();
    cmd.env("LANG", "en");

    cmd.args(["completions", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("_cldev").or(predicate::str::contains("complete")));
}

#[test]
fn test_completion_zsh() {
    let mut cmd = cargo_bin_cmd!();
    cmd.env("LANG", "en");

    cmd.args(["completions", "zsh"])
        .assert()
        .success()
        .stdout(predicate::str::contains("#compdef").or(predicate::str::contains("_cldev")));
}

#[test]
fn test_completion_fish() {
    let mut cmd = cargo_bin_cmd!();
    cmd.env("LANG", "en");

    cmd.args(["completions", "fish"])
        .assert()
        .success()
        .stdout(predicate::str::contains("complete").or(predicate::str::contains("cldev")));
}

#[test]
fn test_completion_powershell() {
    let mut cmd = cargo_bin_cmd!();
    cmd.env("LANG", "en");

    cmd.args(["completions", "powershell"])
        .assert()
        .success()
        .stdout(
            predicate::str::contains("Register-ArgumentCompleter")
                .or(predicate::str::contains("cldev")),
        );
}

#[test]
fn test_completion_elvish() {
    let mut cmd = cargo_bin_cmd!();
    cmd.env("LANG", "en");

    cmd.args(["completions", "elvish"])
        .assert()
        .success()
        .stdout(predicate::str::contains("edit:completion").or(predicate::str::contains("cldev")));
}

#[test]
fn test_completion_invalid_shell() {
    let mut cmd = cargo_bin_cmd!();
    cmd.env("LANG", "en");

    cmd.args(["completions", "invalid-shell"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid").or(predicate::str::contains("value")));
}

#[test]
fn test_completion_help() {
    let mut cmd = cargo_bin_cmd!();
    cmd.env("LANG", "en");

    cmd.args(["completions", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("shell completion"))
        .stdout(predicate::str::contains("bash"))
        .stdout(predicate::str::contains("zsh"));
}

#[test]
fn test_completion_output_not_empty() {
    let shells = vec!["bash", "zsh", "fish", "powershell", "elvish"];

    for shell in shells {
        let mut cmd = cargo_bin_cmd!();
    cmd.env("LANG", "en");

        let output = cmd.args(["completions", shell]).output().unwrap();

        assert!(
            !output.stdout.is_empty(),
            "Completion for {} should not be empty",
            shell
        );
    }
}

#[test]
fn test_completion_bash_contains_commands() {
    let mut cmd = cargo_bin_cmd!();
    cmd.env("LANG", "en");

    cmd.args(["completions", "bash"]).assert().success().stdout(
        predicate::str::contains("config")
            .or(predicate::str::contains("git"))
            .or(predicate::str::contains("quality")),
    );
}

#[test]
fn test_completion_installation_instructions() {
    let mut cmd = cargo_bin_cmd!();
    cmd.env("LANG", "en");

    cmd.args(["completions", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Install").or(predicate::str::contains("installation")));
}

#[test]
fn test_completion_bash_syntax_valid() {
    let mut cmd = cargo_bin_cmd!();
    cmd.env("LANG", "en");

    let output = cmd.args(["completions", "bash"]).output().unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();

    // Basic bash syntax check - should contain function definitions
    assert!(
        stdout.contains("function") || stdout.contains("complete"),
        "Bash completion should contain valid bash syntax"
    );
}

#[test]
fn test_completion_zsh_syntax_valid() {
    let mut cmd = cargo_bin_cmd!();
    cmd.env("LANG", "en");

    let output = cmd.args(["completions", "zsh"]).output().unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();

    // Zsh completions typically start with #compdef
    assert!(
        stdout.contains("#compdef") || stdout.contains("_cldev"),
        "Zsh completion should contain valid zsh syntax"
    );
}

#[test]
fn test_completion_fish_syntax_valid() {
    let mut cmd = cargo_bin_cmd!();
    cmd.env("LANG", "en");

    let output = cmd.args(["completions", "fish"]).output().unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();

    // Fish completions use 'complete' command
    assert!(
        stdout.contains("complete -c cldev"),
        "Fish completion should contain valid fish syntax"
    );
}

#[test]
fn test_completion_stdout_only() {
    let shells = vec!["bash", "zsh", "fish"];

    for shell in shells {
        let mut cmd = cargo_bin_cmd!();
    cmd.env("LANG", "en");

        let output = cmd.args(["completions", shell]).output().unwrap();

        // Completion should only output to stdout, not stderr
        assert!(
            output.stderr.is_empty(),
            "Completion for {} should not output to stderr",
            shell
        );
    }
}
