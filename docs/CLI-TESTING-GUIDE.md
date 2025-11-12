# CLI Testing Guide

## Overview

This guide explains the CLI testing infrastructure for `cldev`, including how to run tests locally and how the CI/CD pipeline validates CLI functionality.

## Test Categories

### 1. CLI Integration Tests (`tests/cli_tests/`)

**Purpose**: Test the compiled binary's command-line interface

**Test Files**:
- `version_test.rs` - Version flag testing
- `help_test.rs` - Help command testing
- `config_commands_test.rs` - Config subcommand testing
- `completion_test.rs` - Shell completion testing
- `analysis_test.rs` - Analysis commands (analyze, serena)
- `i18n_test.rs` - i18n language switching

**Running Locally**:
```bash
# Run all CLI tests
cargo test --test cli

# Run specific test file
cargo test --test version_test
cargo test --test config_commands_test

# Run with output
cargo test --test cli -- --nocapture
```

### 2. i18n Verification Tests

**Purpose**: Ensure all 4 languages (en, ja, zh, zh-TW) work correctly

**What's Tested**:
- Key consistency across all languages
- Japanese output for `analyze` and `serena` commands
- Chinese (Simplified and Traditional) output
- Language flag (`--lang`) functionality

**Running Locally**:
```bash
# Check i18n key consistency
python3 -c "
import json
data = json.load(open('src/i18n/messages.json'))
langs = ['en', 'ja', 'zh', 'zh-TW']
for lang in langs:
    print(f'{lang}: {len(data[lang])} keys')
"

# Test Japanese output
./target/release/cldev --lang ja --help
./target/release/cldev analysis analyze --lang ja overview
./target/release/cldev analysis serena --lang ja

# Test Chinese output
./target/release/cldev --lang zh --help
./target/release/cldev analysis analyze --lang zh overview
```

### 3. Regression Tests

**Purpose**: Verify previously fixed bugs don't reoccur

**Bug Fixes Covered**:
1. **Serena UTF-8 Error** (Commit bced535)
   - Test: Serena handles binary/non-UTF-8 files gracefully
   - Should not crash with "stream did not contain valid UTF-8"

2. **i18n Japanese Display** (Commit 74e9047)
   - Test: `--lang ja` produces Japanese output for analyze command
   - Should not display all English when Japanese is requested

3. **i18n Serena Display** (Commit 17b5f8a)
   - Test: `--lang ja` produces Japanese output for serena command
   - Should contain "セマンティック" or "分析"

**Running Locally**:
```bash
# Test Serena UTF-8 handling
mkdir -p /tmp/cldev-test
echo "binary content" > /tmp/cldev-test/test.bin
echo "fn main() {}" > /tmp/cldev-test/test.rs
./target/release/cldev analysis serena /tmp/cldev-test

# Test i18n (should show Japanese)
./target/release/cldev analysis analyze --lang ja overview
./target/release/cldev analysis serena --lang ja
```

### 4. Performance Tests

**Purpose**: Ensure CLI remains fast and lightweight

**Metrics**:
- Startup time: < 100ms target
- Binary size: < 5MB target

**Running Locally**:
```bash
# Measure startup time (run multiple times for average)
for i in {1..10}; do
  /usr/bin/time -p ./target/release/cldev --version 2>&1 | grep real
done

# Check binary size
ls -lh target/release/cldev
du -h target/release/cldev
```

## CI/CD Pipeline

### Workflow: `cli-testing.yml`

**Triggered by**:
- Push to `main` branch (with changes to `src/` or `tests/cli_tests/`)
- Pull requests to `main`
- Manual trigger via `gh workflow run cli-testing.yml`

**Jobs**:

1. **cli-tests** (Ubuntu, macOS, Windows)
   - Build release binary
   - Run all CLI integration tests
   - Test version and help commands
   - Test all 4 language flags
   - Test config commands

2. **i18n-tests** (Ubuntu)
   - Verify i18n key consistency
   - Test Japanese output (analyze & serena)
   - Test Chinese output (analyze)

3. **regression-tests** (Ubuntu)
   - Test Serena UTF-8 handling
   - Test i18n implementation (analyze)
   - Test i18n implementation (serena)

4. **performance-tests** (Ubuntu)
   - Measure startup time
   - Check binary size

5. **cli-success**
   - Final status check
   - Ensures all jobs passed

## Adding New Tests

### Example: Adding a new CLI test

```rust
// tests/cli_tests/my_feature_test.rs
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_my_feature() {
    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.arg("my-command")
        .arg("--my-flag")
        .assert()
        .success()
        .stdout(predicate::str::contains("expected output"));
}

#[test]
fn test_my_feature_japanese() {
    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.arg("--lang").arg("ja")
        .arg("my-command")
        .assert()
        .success()
        .stdout(predicate::str::contains("日本語出力"));
}
```

### Checklist for New Tests:
- [ ] Test both success and failure cases
- [ ] Test all supported languages if user-facing
- [ ] Use `Command::cargo_bin("cldev")` for binary invocation
- [ ] Add descriptive test names (e.g., `test_feature_with_flag`)
- [ ] Ensure tests are deterministic (no flaky tests)
- [ ] Keep test execution fast (< 1s per test ideally)

## Debugging Test Failures

### Local Debugging:
```bash
# Run with verbose output
cargo test --test cli -- --nocapture --test-threads=1

# Run specific failing test
cargo test --test cli test_name -- --nocapture

# Check what the binary actually outputs
./target/release/cldev <command> 2>&1 | tee /tmp/output.txt
```

### CI Debugging:
1. Check CI logs for specific failure messages
2. Download test artifacts (if test failed):
   - Go to Actions tab → Failed workflow run
   - Download `cli-test-results-<os>` artifact
3. Reproduce locally on same OS if platform-specific

## Best Practices

1. **Write Tests Before Fixing Bugs**
   - Add regression test that fails
   - Fix the bug
   - Verify test passes

2. **Test User-Facing Output**
   - Always test i18n for new commands
   - Test error messages in all languages
   - Verify help text is translated

3. **Keep Tests Fast**
   - Avoid sleep/delays in tests
   - Use `--release` builds in CI for speed
   - Parallelize independent tests

4. **Make Tests Readable**
   - Clear test names describe what's being tested
   - Use `assert!` with descriptive messages
   - Group related tests in same file

## Test Coverage

Current coverage (as of latest commit):
- **CLI Integration**: 47 tests across 4 test files
- **i18n Verification**: 4 languages × 3 key commands = 12 test scenarios
- **Regression**: 3 major bug fixes covered
- **Performance**: 2 metrics tracked

Target: 80%+ coverage of CLI functionality

## Troubleshooting

### Common Issues:

**Issue**: Test fails with "binary not found"
```bash
# Solution: Build the binary first
cargo build --release --bin cldev
```

**Issue**: i18n test fails - "Japanese output not found"
```bash
# Solution: Check messages.json has all keys
python3 -c "import json; data=json.load(open('src/i18n/messages.json')); print(f\"ja: {len(data['ja'])} keys\")"
```

**Issue**: Test passes locally but fails in CI
```bash
# Solution: Check platform-specific behavior
# - Line endings (Windows vs Unix)
# - File paths (separator differences)
# - Temporary directory locations
```

## Manual Testing Checklist

Before releasing a new version:
- [ ] Run `cargo test --all-features`
- [ ] Run `cargo test --test cli`
- [ ] Test all 4 languages manually:
  - [ ] `--lang en`
  - [ ] `--lang ja`
  - [ ] `--lang zh`
  - [ ] `--lang zh-TW`
- [ ] Test on all platforms (if possible):
  - [ ] Linux
  - [ ] macOS
  - [ ] Windows
- [ ] Check binary size: `ls -lh target/release/cldev`
- [ ] Measure startup time: `time ./target/release/cldev --version`

## References

- [assert_cmd documentation](https://docs.rs/assert_cmd/)
- [predicates documentation](https://docs.rs/predicates/)
- [GitHub Actions workflow syntax](https://docs.github.com/en/actions/using-workflows/workflow-syntax-for-github-actions)
