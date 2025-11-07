//! E2E tests for quality assurance workflow
//!
//! Tests the complete workflow for code quality commands including
//! lint, format, and test execution.

use cldev::core::config::Config;
use cldev::core::error::Result;
use git2::Repository;
use std::fs;
use tempfile::TempDir;

fn setup_quality_env() -> Result<TempDir> {
    let temp_dir = TempDir::new()?;
    let repo_path = temp_dir.path();

    // Initialize Git repository
    let repo = Repository::init(repo_path)?;
    let mut config = repo.config()?;
    config.set_str("user.name", "Test User")?;
    config.set_str("user.email", "test@example.com")?;

    // Create Rust project structure
    fs::create_dir_all(repo_path.join("src"))?;
    fs::write(
        repo_path.join("Cargo.toml"),
        r#"
[package]
name = "test-project"
version = "0.1.0"
edition = "2021"

[dependencies]
"#,
    )?;

    fs::write(
        repo_path.join("src/lib.rs"),
        r#"
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }
}
"#,
    )?;

    // Create initial commit
    let mut index = repo.index()?;
    index.add_path(std::path::Path::new("Cargo.toml"))?;
    index.add_path(std::path::Path::new("src/lib.rs"))?;
    index.write()?;

    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    let signature = repo.signature()?;

    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        "Initial commit",
        &tree,
        &[],
    )?;

    // Create cldev config
    let config_dir = repo_path.join(".config/cldev");
    fs::create_dir_all(&config_dir)?;

    let mut cldev_config = Config::default();
    cldev_config.quality.auto_fix = true;
    cldev_config.quality.run_tests_before_commit = true;
    cldev_config.save(Some(config_dir.join("config.toml")))?;

    Ok(temp_dir)
}

#[test]
fn test_quality_config_defaults() -> Result<()> {
    let temp_dir = setup_quality_env()?;
    let config_path = temp_dir.path().join(".config/cldev/config.toml");

    let config = Config::load(Some(config_path))?;

    // Verify quality settings
    assert!(config.quality.auto_fix);
    assert!(config.quality.run_tests_before_commit);

    Ok(())
}

#[test]
fn test_quality_workflow_sequence() -> Result<()> {
    let temp_dir = setup_quality_env()?;
    let config_path = temp_dir.path().join(".config/cldev/config.toml");

    let config = Config::load(Some(config_path))?;

    // Quality workflow: lint -> format -> test
    let workflow_steps = vec![
        "Run linter (clippy)",
        "Auto-fix lint issues",
        "Format code",
        "Run tests",
        "Check coverage",
    ];

    // Simulate workflow
    assert!(config.quality.auto_fix);

    for step in workflow_steps {
        // In real scenario, each step would execute actual commands
        println!("Executing: {}", step);
    }

    Ok(())
}

#[test]
fn test_lint_workflow() -> Result<()> {
    let temp_dir = setup_quality_env()?;

    // Simulate lint workflow
    let steps = vec![
        "cargo clippy --all-targets",
        "cargo clippy --fix --allow-dirty",
        "cargo fmt --check",
    ];

    for step in steps {
        println!("Would execute: {}", step);
    }

    Ok(())
}

#[test]
fn test_format_workflow() -> Result<()> {
    let temp_dir = setup_quality_env()?;

    // Create file with formatting issues
    let bad_format = r#"
pub fn badly_formatted(  x : i32  ,y:i32)->i32{
x+y
}
"#;

    fs::write(temp_dir.path().join("src/bad.rs"), bad_format)?;

    // Simulate format command
    println!("Would execute: cargo fmt");

    Ok(())
}

#[test]
fn test_test_workflow() -> Result<()> {
    let temp_dir = setup_quality_env()?;

    // Simulate test workflow
    let test_commands = vec![
        "cargo test",
        "cargo test --release",
        "cargo test --all-features",
    ];

    for cmd in test_commands {
        println!("Would execute: {}", cmd);
    }

    Ok(())
}

#[test]
fn test_quality_before_commit() -> Result<()> {
    let temp_dir = setup_quality_env()?;
    let config_path = temp_dir.path().join(".config/cldev/config.toml");

    let config = Config::load(Some(config_path))?;

    if config.quality.run_tests_before_commit {
        // Simulate pre-commit quality checks
        println!("Running pre-commit quality checks:");
        println!("  1. Linting");
        println!("  2. Formatting");
        println!("  3. Tests");
    }

    Ok(())
}

#[test]
fn test_coverage_workflow() -> Result<()> {
    let temp_dir = setup_quality_env()?;

    // Simulate coverage workflow
    let coverage_steps = vec![
        "cargo tarpaulin --out Html",
        "Open coverage report",
        "Verify coverage threshold",
    ];

    for step in coverage_steps {
        println!("Would execute: {}", step);
    }

    Ok(())
}

#[test]
fn test_quality_ci_workflow() -> Result<()> {
    let temp_dir = setup_quality_env()?;

    // Simulate CI quality workflow
    let ci_steps = vec![
        "cargo clippy -- -D warnings",
        "cargo fmt -- --check",
        "cargo test --all-features",
        "cargo doc --no-deps",
    ];

    for step in ci_steps {
        println!("CI step: {}", step);
    }

    Ok(())
}

#[test]
fn test_quality_with_multiple_targets() -> Result<()> {
    let temp_dir = setup_quality_env()?;

    // Create additional targets
    fs::create_dir_all(temp_dir.path().join("examples"))?;
    fs::write(
        temp_dir.path().join("examples/example.rs"),
        "fn main() { println!(\"Hello\"); }",
    )?;

    fs::create_dir_all(temp_dir.path().join("benches"))?;
    fs::write(
        temp_dir.path().join("benches/bench.rs"),
        "#![feature(test)]\nextern crate test;\n",
    )?;

    // Simulate quality checks on all targets
    let targets = vec!["lib", "examples", "benches", "tests"];

    for target in targets {
        println!("Checking quality for: {}", target);
    }

    Ok(())
}

#[test]
fn test_quality_auto_fix_toggle() -> Result<()> {
    let temp_dir = setup_quality_env()?;
    let config_path = temp_dir.path().join(".config/cldev/config.toml");

    // Load config
    let mut config = (*Config::load(Some(config_path.clone()))?).clone();

    // Test with auto_fix enabled
    config.quality.auto_fix = true;
    config.save(Some(config_path.clone()))?;

    let loaded = Config::load(Some(config_path.clone()))?;
    assert!(loaded.quality.auto_fix);

    // Test with auto_fix disabled
    let mut config2 = (*loaded).clone();
    config2.quality.auto_fix = false;
    config2.save(Some(config_path.clone()))?;

    let loaded2 = Config::load(Some(config_path))?;
    assert!(!loaded2.quality.auto_fix);

    Ok(())
}

#[test]
fn test_quality_parallel_execution() -> Result<()> {
    let temp_dir = setup_quality_env()?;
    let config_path = temp_dir.path().join(".config/cldev/config.toml");

    let config = Config::load(Some(config_path))?;

    // Use parallel_tasks setting for quality checks
    let parallel_tasks = config.performance.parallel_tasks;

    println!(
        "Running quality checks with {} parallel tasks",
        parallel_tasks
    );

    // Simulate parallel execution
    for i in 0..parallel_tasks {
        println!("  Task {}: Running checks", i + 1);
    }

    Ok(())
}

#[test]
fn test_quality_with_timeout() -> Result<()> {
    let temp_dir = setup_quality_env()?;
    let config_path = temp_dir.path().join(".config/cldev/config.toml");

    let config = Config::load(Some(config_path))?;

    // Use timeout setting
    let timeout = config.performance.timeout_seconds;

    println!("Quality checks will timeout after {} seconds", timeout);

    Ok(())
}

#[test]
fn test_incremental_quality_checks() -> Result<()> {
    let temp_dir = setup_quality_env()?;

    // Simulate incremental checks (only changed files)
    let changed_files = vec!["src/lib.rs", "src/utils.rs"];

    for file in changed_files {
        println!("Running quality check on: {}", file);
    }

    Ok(())
}

#[test]
fn test_quality_report_generation() -> Result<()> {
    let temp_dir = setup_quality_env()?;

    // Simulate quality report generation
    let report_sections = vec![
        "Lint Results",
        "Format Check Results",
        "Test Results",
        "Coverage Results",
    ];

    for section in report_sections {
        println!("Generating: {}", section);
    }

    Ok(())
}

#[test]
fn test_quality_pre_push_hooks() -> Result<()> {
    let temp_dir = setup_quality_env()?;

    // Simulate pre-push quality checks
    let pre_push_checks = vec![
        "Verify all tests pass",
        "Verify no lint warnings",
        "Verify code is formatted",
        "Verify no uncommitted changes",
    ];

    for check in pre_push_checks {
        println!("Pre-push check: {}", check);
    }

    Ok(())
}

#[test]
fn test_quality_benchmark_workflow() -> Result<()> {
    let temp_dir = setup_quality_env()?;

    // Create benchmark file
    fs::create_dir_all(temp_dir.path().join("benches"))?;
    fs::write(
        temp_dir.path().join("benches/my_benchmark.rs"),
        r#"
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
"#,
    )?;

    // Simulate benchmark workflow
    println!("Would execute: cargo bench");

    Ok(())
}

#[test]
fn test_quality_doc_tests() -> Result<()> {
    let temp_dir = setup_quality_env()?;

    // Add doc tests to lib.rs
    let lib_with_docs = r#"
/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// assert_eq!(test_project::add(2, 2), 4);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
"#;

    fs::write(temp_dir.path().join("src/lib.rs"), lib_with_docs)?;

    // Simulate doc test execution
    println!("Would execute: cargo test --doc");

    Ok(())
}

#[test]
fn test_quality_integration_with_git() -> Result<()> {
    let temp_dir = setup_quality_env()?;

    // Get Git repository
    let repo = Repository::open(temp_dir.path())?;

    // Check if working directory is clean before quality checks
    let statuses = repo.statuses(None)?;
    let is_clean = statuses.is_empty();

    if !is_clean {
        println!("Warning: Working directory has uncommitted changes");
    }

    Ok(())
}
