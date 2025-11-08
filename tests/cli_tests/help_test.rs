//! CLI tests for help command
//!
//! Tests the --help option for all commands and subcommands.

use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;

#[test]
fn test_help_flag() {
    let mut cmd = cargo_bin_cmd!();
    cmd.env("LANG", "en");

    cmd.env("LANG", "en").arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("cldev"))
        .stdout(predicate::str::contains("Usage:"));
}

#[test]
fn test_help_command() {
    let mut cmd = cargo_bin_cmd!();
    cmd.env("LANG", "en");

    cmd.env("LANG", "en").arg("help")
        .assert()
        .success()
        .stdout(predicate::str::contains("cldev"));
}

#[test]
fn test_config_help() {
    let mut cmd = cargo_bin_cmd!();
    cmd.env("LANG", "en");

    cmd.env("LANG", "en").args(["config", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Manage cldev configuration"))
        .stdout(predicate::str::contains("init"))
        .stdout(predicate::str::contains("check"))
        .stdout(predicate::str::contains("list"));
}

#[test]
fn test_config_init_help() {
    let mut cmd = cargo_bin_cmd!();
    cmd.env("LANG", "en");

    cmd.args(["config", "init", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Initialize cldev configuration"));
}

#[test]
fn test_config_check_help() {
    let mut cmd = cargo_bin_cmd!();
    cmd.env("LANG", "en");

    cmd.args(["config", "check", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Validate configuration health"));
}

#[test]
fn test_config_list_help() {
    let mut cmd = cargo_bin_cmd!();
    cmd.env("LANG", "en");

    cmd.args(["config", "list", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("List all configurations"));
}

#[test]
fn test_git_help() {
    let mut cmd = cargo_bin_cmd!();
    cmd.env("LANG", "en");

    cmd.args(["git", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Git operations"))
        .stdout(predicate::str::contains("status"))
        .stdout(predicate::str::contains("commit"))
        .stdout(predicate::str::contains("branch"));
}

#[test]
fn test_quality_help() {
    let mut cmd = cargo_bin_cmd!();
    cmd.env("LANG", "en");

    cmd.args(["quality", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Code quality"))
        .stdout(predicate::str::contains("lint"))
        .stdout(predicate::str::contains("format"))
        .stdout(predicate::str::contains("test"));
}

#[test]
fn test_dev_help() {
    let mut cmd = cargo_bin_cmd!();
    cmd.env("LANG", "en");

    cmd.args(["dev", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Core development workflows"))
        .stdout(predicate::str::contains("urgent"))
        .stdout(predicate::str::contains("fix"))
        .stdout(predicate::str::contains("debug"));
}

#[test]
fn test_lr_help() {
    let mut cmd = cargo_bin_cmd!();
    cmd.env("LANG", "en");

    cmd.args(["lr", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Learning record"))
        .stdout(predicate::str::contains("new"))
        .stdout(predicate::str::contains("find"))
        .stdout(predicate::str::contains("stats"));
}

#[test]
fn test_analysis_help() {
    let mut cmd = cargo_bin_cmd!();
    cmd.env("LANG", "en");

    cmd.args(["analysis", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Code analysis"))
        .stdout(predicate::str::contains("analyze"))
        .stdout(predicate::str::contains("explain"));
}

#[test]
fn test_no_args_shows_help() {
    let mut cmd = cargo_bin_cmd!();
    cmd.env("LANG", "en");

    // Running without args should show help or usage
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage:").or(predicate::str::contains("cldev")));
}

#[test]
fn test_invalid_subcommand() {
    let mut cmd = cargo_bin_cmd!();
    cmd.env("LANG", "en");

    cmd.arg("invalid-subcommand")
        .assert()
        .failure()
        .stderr(predicate::str::contains("unrecognized").or(predicate::str::contains("invalid")));
}

#[test]
fn test_help_output_formatting() {
    let mut cmd = cargo_bin_cmd!();
    cmd.env("LANG", "en");

    cmd.env("LANG", "en").arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Commands:"))
        .stdout(predicate::str::contains("Options:"));
}

#[test]
fn test_subcommand_help_consistency() {
    let subcommands = vec!["config", "git", "quality", "dev", "lr", "analysis"];

    for subcommand in subcommands {
        let mut cmd = cargo_bin_cmd!();
    cmd.env("LANG", "en");

        cmd.args([subcommand, "--help"])
            .assert()
            .success()
            .stdout(predicate::str::contains("Usage:"));
    }
}
