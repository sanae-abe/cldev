# Contributing to cldev

Thank you for your interest in contributing to cldev! This document provides guidelines and instructions for contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Development Workflow](#development-workflow)
- [Coding Standards](#coding-standards)
- [Testing Guidelines](#testing-guidelines)
- [Documentation](#documentation)
- [Submitting Changes](#submitting-changes)
- [Release Process](#release-process)

---

## Code of Conduct

This project follows the Rust Code of Conduct. Please be respectful and constructive in all interactions.

### Our Standards

- Be welcoming and inclusive
- Respect differing viewpoints and experiences
- Accept constructive criticism gracefully
- Focus on what is best for the community
- Show empathy towards other community members

---

## Getting Started

### Prerequisites

- Rust 1.70 or later
- Git
- A text editor or IDE (VS Code with rust-analyzer recommended)

### Initial Setup

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/cldev.git
   cd cldev
   ```
3. Add the upstream repository:
   ```bash
   git remote add upstream https://github.com/ORIGINAL_OWNER/cldev.git
   ```

---

## Development Setup

### Building the Project

```bash
# Debug build (fast compilation)
cargo build

# Release build (optimized)
cargo build --release
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run tests with coverage (requires cargo-tarpaulin)
cargo tarpaulin --out Html
```

### Running the CLI

```bash
# Debug build
cargo run -- [ARGUMENTS]

# Release build
./target/release/cldev [ARGUMENTS]
```

---

## Development Workflow

### 1. Create a Branch

```bash
# Feature branch
git checkout -b feature/your-feature-name

# Bug fix branch
git checkout -b fix/bug-description

# Documentation branch
git checkout -b docs/topic
```

### 2. Make Changes

- Write clean, well-documented code
- Follow Rust best practices
- Add tests for new functionality
- Update documentation as needed

### 3. Test Your Changes

```bash
# Run tests
cargo test

# Check formatting
cargo fmt --check

# Run clippy
cargo clippy -- -D warnings

# Build release
cargo build --release
```

### 4. Commit Your Changes

We follow Conventional Commits specification:

```bash
# Feature
git commit -m "feat: add new command for code analysis"

# Bug fix
git commit -m "fix: resolve panic in config loading"

# Documentation
git commit -m "docs: update installation instructions"

# Refactoring
git commit -m "refactor: simplify error handling in core module"

# Tests
git commit -m "test: add unit tests for i18n system"

# Performance
git commit -m "perf: optimize message lookup in i18n"
```

**Commit Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `perf`: Performance improvements
- `chore`: Build process or auxiliary tool changes
- `ci`: CI/CD changes

### 5. Push Changes

```bash
git push origin your-branch-name
```

### 6. Create Pull Request

- Go to GitHub and create a pull request
- Fill out the PR template completely
- Link related issues
- Request review from maintainers

---

## Coding Standards

### Rust Style Guide

Follow the official Rust style guide and use `rustfmt`:

```bash
# Format all code
cargo fmt

# Check formatting
cargo fmt --check
```

### Code Quality

```bash
# Run clippy for linting
cargo clippy -- -D warnings

# Check for common mistakes
cargo clippy --all-targets --all-features
```

### Key Principles

1. **Type Safety**: Leverage Rust's type system for correctness
2. **Error Handling**: Use `Result<T, E>` and proper error types
3. **Documentation**: Document all public APIs with rustdoc
4. **Testing**: Maintain high test coverage (target: 80%+)
5. **Security**: Follow secure coding practices
6. **Performance**: Optimize hot paths, but prioritize clarity

### Code Organization

```
src/
├── lib.rs           # Library entry point
├── main.rs          # Binary entry point
├── cli/             # CLI-specific code
│   ├── args.rs      # Command-line argument parsing
│   └── output.rs    # Output formatting and i18n
├── commands/        # Command implementations
│   ├── config/      # Config management commands
│   ├── git/         # Git operation commands
│   └── quality/     # Code quality commands
├── core/            # Core functionality
│   ├── config.rs    # Configuration management
│   ├── i18n.rs      # Internationalization
│   ├── error.rs     # Error types
│   └── security.rs  # Security utilities
└── utils/           # Utility functions
```

### Naming Conventions

- **Functions**: `snake_case`
- **Types**: `PascalCase`
- **Constants**: `SCREAMING_SNAKE_CASE`
- **Modules**: `snake_case`
- **Lifetimes**: Single lowercase letter (`'a`, `'b`)

### Error Handling

```rust
// Use custom error types with thiserror
#[derive(Debug, thiserror::Error)]
pub enum CldevError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

// Propagate errors with ?
fn example() -> Result<(), CldevError> {
    let config = Config::load()?;
    // ...
    Ok(())
}
```

---

## Testing Guidelines

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_name() {
        // Arrange
        let input = "test data";

        // Act
        let result = function_under_test(input);

        // Assert
        assert_eq!(result, expected_value);
    }
}
```

### Integration Tests

Place integration tests in `tests/` directory:

```rust
// tests/integration_test.rs
use cldev::core::Config;

#[test]
fn test_full_workflow() {
    // Test complete workflows
}
```

### Test Coverage

- Aim for 80%+ code coverage
- Test happy paths and error cases
- Test edge cases and boundary conditions
- Use property-based testing for complex logic (quickcheck, proptest)

### Running Tests

```bash
# All tests
cargo test

# Specific module
cargo test core::config

# Integration tests only
cargo test --test '*'

# With coverage
cargo tarpaulin --out Html
```

---

## Documentation

### Code Documentation

Use rustdoc comments for all public APIs:

```rust
/// Loads configuration from the specified path.
///
/// # Arguments
///
/// * `path` - Optional path to configuration file. If `None`, uses default path.
///
/// # Returns
///
/// Returns `Arc<Config>` on success, or `CldevError` on failure.
///
/// # Examples
///
/// ```
/// use cldev::core::Config;
///
/// let config = Config::load(None)?;
/// ```
///
/// # Security
///
/// On Unix systems, this function validates that the configuration file
/// has permissions set to 600 (owner read/write only).
pub fn load(path: Option<PathBuf>) -> Result<Arc<Self>> {
    // implementation
}
```

### User Documentation

- Update `docs/guides/` for user-facing documentation
- Update `docs/architecture/` for design decisions
- Update `CHANGELOG.md` for all changes
- Update `README.md` if adding major features

### Generating Documentation

```bash
# Generate and open documentation
cargo doc --open

# Generate with private items
cargo doc --document-private-items --open
```

---

## Submitting Changes

### Pull Request Process

1. **Update Documentation**: Ensure all documentation is up to date
2. **Add Tests**: Include tests for new functionality
3. **Update CHANGELOG**: Add entry under "Unreleased" section
4. **Run CI Checks**: Ensure all CI checks pass
5. **Request Review**: Request review from maintainers
6. **Address Feedback**: Respond to review comments promptly

### Pull Request Template

```markdown
## Description
Brief description of changes

## Related Issues
Fixes #123

## Changes Made
- Item 1
- Item 2

## Testing
- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] Manual testing completed

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] Tests passing
- [ ] CHANGELOG.md updated
```

### Review Process

1. Maintainer review (1-2 business days)
2. Address review feedback
3. Approval from at least one maintainer
4. Merge to main branch

---

## Release Process

### Versioning

We follow Semantic Versioning (SemVer):

- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

### Release Checklist

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md` with release date
3. Create git tag: `git tag -a v1.2.3 -m "Release v1.2.3"`
4. Push tag: `git push origin v1.2.3`
5. GitHub Actions will automatically create release

### CHANGELOG Format

Follow "Keep a Changelog" format:

```markdown
## [1.2.3] - 2025-11-07

### Added
- New feature description

### Changed
- Changed feature description

### Deprecated
- Deprecated feature description

### Removed
- Removed feature description

### Fixed
- Bug fix description

### Security
- Security fix description
```

---

## Getting Help

### Resources

- Documentation: `docs/`
- GitHub Issues: Report bugs or request features
- Discussions: Ask questions and share ideas

### Contact

- GitHub Issues: For bug reports and feature requests
- GitHub Discussions: For questions and general discussion

---

## License

By contributing to cldev, you agree that your contributions will be licensed under the project's license (to be determined).

---

## Recognition

Contributors will be recognized in:
- `CONTRIBUTORS.md` file
- Release notes for significant contributions
- GitHub contributors page

Thank you for contributing to cldev!
