# CLI Testing Guide

## Overview

This guide covers comprehensive CLI testing for the `cldev` project, including unit tests, integration tests, and CI/CD integration.

## Test Organization

### Test Categories

1. **Version Tests** (`tests/cli_tests/version_test.rs`)
   - Version flag output
   - Version format validation
   - Consistency checks

2. **Help Tests** (`tests/cli_tests/help_test.rs`)
   - Help flag output
   - Subcommand help
   - Invalid command handling

3. **Config Tests** (`tests/cli_tests/config_commands_test.rs`)
   - Configuration initialization
   - Configuration validation
   - Configuration listing

4. **Completion Tests** (`tests/cli_tests/completion_test.rs`)
   - Shell completion generation
   - Supported shells

5. **i18n Tests** (`tests/cli_tests/i18n_test.rs`) **[NEW]**
   - English (en)
   - Japanese (ja)
   - Chinese Simplified (zh)
   - Chinese Traditional (zh-TW)
   - Language flag handling
   - UTF-8 output validation

6. **Analysis Tests** (`tests/cli_tests/analysis_test.rs`) **[NEW]**
   - Analyze command (structure, performance, quality, debt, overview)
   - Serena command
   - UTF-8 handling regression tests
   - Binary file handling
   - i18n output for analysis commands

## Running Tests Locally

### Quick Start

```bash
# Run all CLI tests
cargo test --test cli

# Run all tests in the CLI test directory
cargo test --tests

# Run specific test file
cargo test --test version_test
cargo test --test i18n_test
cargo test --test analysis_test
```

### Individual Test Categories

```bash
# Version tests
cargo test --test version_test --verbose

# Help tests
cargo test --test help_test --verbose

# Config tests
cargo test --test config_commands_test --verbose

# Completion tests
cargo test --test completion_test --verbose

# i18n tests (all languages)
cargo test --test i18n_test --verbose

# Analysis tests (analyze, serena)
cargo test --test analysis_test --verbose
```

### Running Specific Tests

```bash
# Run a specific test function
cargo test --test i18n_test test_japanese_help

# Run tests matching a pattern
cargo test --test analysis_test serena

# Run with output displayed
cargo test --test i18n_test -- --nocapture
```

### Language Testing

```bash
# Test all supported languages
cargo test --test i18n_test test_version_all_languages

# Test specific language
cargo test --test i18n_test test_japanese_help
cargo test --test i18n_test test_chinese_simplified_help
cargo test --test i18n_test test_chinese_traditional_help

# Test UTF-8 output
cargo test --test i18n_test test_utf8_japanese_output
```

### Analysis Command Testing

```bash
# Test analyze command
cargo test --test analysis_test test_analyze

# Test serena command
cargo test --test analysis_test test_serena

# Test UTF-8 regression (binary file handling)
cargo test --test analysis_test test_serena_utf8_regression
```

## Test Development

### Test Structure

All CLI tests use the `assert_cmd` crate for command execution:

```rust
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_example() {
    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("cldev"));
}
```

### Testing Multiple Languages

```rust
#[test]
fn test_all_languages() {
    let languages = vec!["en", "ja", "zh", "zh-TW"];

    for lang in languages {
        let mut cmd = Command::cargo_bin("cldev").unwrap();

        cmd.arg("--lang")
            .arg(lang)
            .arg("--help")
            .assert()
            .success();
    }
}
```

### Testing UTF-8 Handling

```rust
#[test]
fn test_utf8_output() {
    let mut cmd = Command::cargo_bin("cldev").unwrap();

    let output = cmd.arg("--lang")
        .arg("ja")
        .arg("--version")
        .output()
        .unwrap();

    // Should successfully parse UTF-8
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("cldev"));
}
```

### Testing Binary File Handling

```rust
#[test]
fn test_binary_file_no_panic() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("binary.bin");

    // Create binary file with non-UTF-8 bytes
    let mut file = fs::File::create(&file_path).unwrap();
    file.write_all(&[0xFF, 0xFE, 0xFD]).unwrap();

    let mut cmd = Command::cargo_bin("cldev").unwrap();

    // Should not panic
    cmd.args(["analysis", "serena", file_path.to_str().unwrap()])
        .assert()
        .code(predicate::function(|code: &i32| *code == 0 || *code == 1));
}
```

## CI/CD Integration

### GitHub Actions Workflow

The CLI tests are integrated into the CI pipeline at `.github/workflows/ci.yml`:

```yaml
cli-tests:
  name: CLI Integration Tests
  strategy:
    fail-fast: false
    matrix:
      os: [ubuntu-latest, macos-latest, windows-latest]
  runs-on: ${{ matrix.os }}
  steps:
    - name: Run CLI version tests
      run: cargo test --test version_test --verbose --locked

    - name: Run CLI i18n tests
      run: cargo test --test i18n_test --verbose --locked

    - name: Run CLI analysis tests
      run: cargo test --test analysis_test --verbose --locked
```

### Test Execution Order

1. **Build binary** - Release binary is built first
2. **Version tests** - Basic functionality
3. **Help tests** - Help output validation
4. **Config tests** - Configuration management
5. **Completion tests** - Shell completion
6. **i18n tests** - Language support
7. **Analysis tests** - Analysis commands
8. **Startup time tests** - Performance validation

### CI Test Matrix

Tests run on:
- **OS**: Ubuntu, macOS, Windows
- **Rust versions**: Stable, MSRV (1.70.0)

## Performance Benchmarks

### Target Metrics

- **Startup time**: ≤ 25ms (currently 21ms)
- **Test suite**: ≤ 10s for CLI tests
- **Binary size**: ≤ 2MB (currently 1.5MB)

### Measuring Startup Time

```bash
# Unix/macOS
time ./target/release/cldev --version

# Windows (PowerShell)
Measure-Command { ./target/release/cldev.exe --version }
```

## Test Coverage

### Current Coverage

- **Version tests**: 5 tests
- **Help tests**: 15 tests
- **Config tests**: 10+ tests
- **Completion tests**: 5+ tests
- **i18n tests**: 25 tests (NEW)
- **Analysis tests**: 35+ tests (NEW)

**Total CLI tests**: 95+ tests

### Coverage Goals

- ✅ All 4 languages tested (en, ja, zh, zh-TW)
- ✅ UTF-8 handling validated
- ✅ Binary file handling tested
- ✅ Regression tests for known bugs
- ✅ Success and error cases covered
- ✅ Output format validation

## Troubleshooting

### Common Issues

#### Test Failures on Windows

```bash
# Use PowerShell for better Unicode support
cargo test --test i18n_test
```

#### UTF-8 Encoding Issues

```bash
# Set environment variable
export LANG=en_US.UTF-8

# Run tests
cargo test --test i18n_test
```

#### Temporary File Cleanup

```bash
# Clean up test artifacts
cargo clean
rm -rf target/debug/deps/
```

### Debug Output

```bash
# Show test output
cargo test --test analysis_test -- --nocapture

# Show test execution
cargo test --test i18n_test -- --show-output

# Verbose mode
cargo test --test analysis_test --verbose
```

## Best Practices

### Test Writing Guidelines

1. **Use descriptive test names**
   ```rust
   #[test]
   fn test_serena_utf8_regression() { ... }
   ```

2. **Test both success and failure cases**
   ```rust
   #[test]
   fn test_analyze_invalid_target() { ... }
   ```

3. **Use temporary directories for file tests**
   ```rust
   let temp_dir = TempDir::new().unwrap();
   ```

4. **Clean up resources**
   ```rust
   // TempDir automatically cleans up on drop
   ```

5. **Test all supported languages**
   ```rust
   let languages = vec!["en", "ja", "zh", "zh-TW"];
   ```

### Performance Considerations

- Keep tests fast (< 1s per test)
- Use `--release` for binary performance tests
- Parallel test execution enabled by default

### Regression Testing

Always add tests for fixed bugs:

```rust
/// Test serena regression: UTF-8 panic on binary files
/// This test ensures the bug fix for Serena UTF-8 handling is working
#[test]
fn test_serena_utf8_regression() {
    // Test implementation
}
```

## Integration with Development Workflow

### Pre-commit Checks

```bash
# Before committing
cargo fmt --check
cargo clippy --all-targets --all-features
cargo test --test cli
```

### Pre-push Checks

```bash
# Full test suite
cargo test --all-features
```

### CI Requirements

All tests must pass before merge:
- ✅ Unit tests
- ✅ Integration tests
- ✅ CLI tests
- ✅ Clippy
- ✅ Formatting

## Future Enhancements

### Planned Tests

- [ ] End-to-end workflow tests
- [ ] Performance regression tests
- [ ] Network-dependent command tests
- [ ] Interactive command tests
- [ ] Error recovery tests

### Test Infrastructure

- [ ] Test data fixtures
- [ ] Mock external dependencies
- [ ] Test result dashboards
- [ ] Automated performance tracking

## References

- [assert_cmd documentation](https://docs.rs/assert_cmd/)
- [predicates documentation](https://docs.rs/predicates/)
- [Cargo test documentation](https://doc.rust-lang.org/cargo/commands/cargo-test.html)
- [GitHub Actions workflow](.github/workflows/ci.yml)

## Summary

### Test Statistics

- **Total CLI tests**: 95+
- **Test categories**: 6
- **Languages tested**: 4 (en, ja, zh, zh-TW)
- **Platforms tested**: 3 (Linux, macOS, Windows)
- **Execution time**: < 10s

### Key Features

- ✅ Comprehensive language support testing
- ✅ UTF-8 handling validation
- ✅ Binary file safety tests
- ✅ Regression test coverage
- ✅ CI/CD integration
- ✅ Multi-platform validation
- ✅ Performance benchmarking
