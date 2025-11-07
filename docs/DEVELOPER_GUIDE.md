# cldev Developer Guide

**Version**: 1.0.0
**Last Updated**: 2025-11-07

## Table of Contents

- [Introduction](#introduction)
- [Architecture Overview](#architecture-overview)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [Core Systems](#core-systems)
- [Adding Features](#adding-features)
- [Testing](#testing)
- [Documentation](#documentation)
- [Performance](#performance)
- [Security](#security)
- [Release Process](#release-process)

---

## Introduction

### Purpose

This guide provides comprehensive information for developers who want to:
- Contribute to cldev development
- Understand the codebase architecture
- Add new features or commands
- Fix bugs and optimize performance
- Extend functionality

### Prerequisites

**Required:**
- Rust 1.70 or later
- Git
- Understanding of CLI development
- Familiarity with TOML, JSON

**Recommended:**
- Experience with clap for CLI parsing
- Knowledge of serde for serialization
- Understanding of i18n concepts
- Familiarity with Rust best practices

---

## Architecture Overview

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    cldev Binary                         │
├─────────────────────────────────────────────────────────┤
│  CLI Layer (clap)                                       │
│  ├─ Argument Parsing                                    │
│  ├─ Command Routing                                     │
│  └─ Output Handling (i18n)                              │
├─────────────────────────────────────────────────────────┤
│  Command Layer                                          │
│  ├─ Config Commands (init, check, list)                │
│  ├─ Git Commands (planned)                             │
│  └─ Quality Commands (planned)                         │
├─────────────────────────────────────────────────────────┤
│  Core Layer                                             │
│  ├─ Configuration Management (Arc<Config>)             │
│  ├─ i18n System (MessageCatalog, I18n)                 │
│  ├─ Error Handling (CldevError)                        │
│  └─ Security (Permissions, Validation)                 │
├─────────────────────────────────────────────────────────┤
│  Infrastructure                                         │
│  ├─ File System (dirs, std::fs)                        │
│  ├─ Serialization (serde, toml)                        │
│  └─ UI (dialoguer, indicatif)                          │
└─────────────────────────────────────────────────────────┘
```

### Design Principles

1. **Modularity**: Clear separation of concerns
2. **Type Safety**: Leverage Rust's type system
3. **Error Handling**: Comprehensive Result types
4. **Performance**: Zero-cost abstractions
5. **Security**: Secure by default
6. **Testability**: Unit and integration tests
7. **Documentation**: Self-documenting code
8. **i18n**: Multi-language support

### Key Design Decisions

#### Arc<Config> Pattern

**Decision**: Use `Arc<Config>` for configuration sharing
**Rationale**:
- Thread-safe sharing across modules
- Single allocation, cheap cloning
- Immutable after creation
- Strong reference counting

**Example**:
```rust
pub fn load(path: Option<PathBuf>) -> Result<Arc<Self>> {
    let config = Config::default();
    Ok(Arc::new(config))
}
```

#### Zero-Cost i18n

**Decision**: Embed translations in binary with `include_str!()`
**Rationale**:
- No runtime file I/O
- Compile-time validation
- O(1) lookup with HashMap
- ~5KB binary size increase

**Example**:
```rust
const MESSAGES_JSON: &str = include_str!("../i18n/messages.json");
```

#### Interactive UI

**Decision**: Use dialoguer + indicatif for interactive prompts
**Rationale**:
- Excellent UX with minimal code
- Cross-platform compatibility
- Consistent theming
- Progress visualization

---

## Development Setup

### Clone and Build

```bash
# Clone repository
git clone https://github.com/YOUR_USERNAME/cldev
cd cldev

# Build debug version (fast compilation)
cargo build

# Build release version (optimized)
cargo build --release
```

### Development Tools

#### Recommended VS Code Extensions
- rust-analyzer
- CodeLLDB (debugging)
- Better TOML
- Error Lens

#### Recommended Tools
```bash
# Install clippy (linter)
rustup component add clippy

# Install rustfmt (formatter)
rustup component add rustfmt

# Install cargo-watch (auto-rebuild)
cargo install cargo-watch

# Install cargo-tarpaulin (coverage)
cargo install cargo-tarpaulin
```

### Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_name

# With output
cargo test -- --nocapture

# With coverage
cargo tarpaulin --out Html
```

### Development Workflow

```bash
# Watch mode (auto-rebuild on changes)
cargo watch -x build

# Run with arguments
cargo run -- config init --lang ja

# Check without building
cargo check

# Lint
cargo clippy -- -D warnings

# Format
cargo fmt
```

---

## Project Structure

```
cldev/
├── src/
│   ├── lib.rs                  # Library entry point
│   ├── main.rs                 # Binary entry point
│   ├── cli/
│   │   ├── mod.rs              # CLI module exports
│   │   ├── args.rs             # Argument definitions (clap)
│   │   └── output.rs           # Output handling + i18n
│   ├── commands/
│   │   ├── mod.rs              # Command module exports
│   │   └── config/
│   │       ├── mod.rs          # Config command exports
│   │       ├── init.rs         # Interactive initialization
│   │       ├── check.rs        # Configuration validation
│   │       └── list.rs         # Configuration display
│   ├── core/
│   │   ├── mod.rs              # Core module exports
│   │   ├── config.rs           # Configuration management
│   │   ├── i18n.rs             # Internationalization
│   │   ├── error.rs            # Error types
│   │   └── security.rs         # Security utilities
│   └── i18n/
│       └── messages.json       # Translation catalog
├── tests/
│   └── integration_test.rs     # Integration tests
├── examples/
│   └── i18n_demo.rs            # i18n demonstration
├── docs/
│   ├── USER_GUIDE.md           # User documentation
│   ├── DEVELOPER_GUIDE.md      # This file
│   ├── architecture/           # Architecture docs
│   ├── guides/                 # User guides
│   ├── implementation/         # Implementation details
│   └── development/            # Development planning
├── completions/                # Shell completions (generated)
├── Cargo.toml                  # Project manifest
├── Cargo.lock                  # Dependency lock file
├── CONTRIBUTING.md             # Contribution guidelines
├── CHANGELOG.md                # Version history
└── README.md                   # Project overview
```

---

## Core Systems

### Configuration System

**Location**: `src/core/config.rs`

#### Key Components

```rust
/// Main configuration structure
pub struct Config {
    version: String,
    general: GeneralConfig,
    git: GitConfig,
    quality: QualityConfig,
    dev: DevConfig,
    lr: LearningRecordConfig,
    ui: UiConfig,
    performance: PerformanceConfig,
}

/// Version validation
pub struct ConfigVersion;
```

#### Usage

```rust
use cldev::core::Config;
use std::sync::Arc;

// Load configuration
let config: Arc<Config> = Config::load(None)?;

// Access fields
let language = &config.general.language;

// Save configuration
config.save(None)?;
```

#### Adding Configuration Fields

1. Add field to appropriate struct in `config.rs`
2. Update `Default` implementation
3. Add to TOML serialization tests
4. Update documentation

**Example**:
```rust
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeneralConfig {
    pub language: String,
    pub new_field: String,  // Add here
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            language: "ja".to_string(),
            new_field: "default_value".to_string(),  // Add here
        }
    }
}
```

### i18n System

**Location**: `src/core/i18n.rs`, `src/i18n/messages.json`

#### Key Components

```rust
/// Language enumeration
pub enum Language {
    English,
    Japanese,
}

/// Message catalog
pub struct MessageCatalog {
    messages: HashMap<String, HashMap<String, String>>,
}

/// i18n handler
pub struct I18n {
    current_language: Language,
    catalog: MessageCatalog,
}
```

#### Usage

```rust
use cldev::core::{I18n, Language};

// Create i18n handler
let i18n = I18n::new(Language::Japanese);

// Get message
let msg = i18n.t("config.init.success");

// With variable
let msg = i18n.t_format("config.saved", "path", "/path/to/config");

// With multiple variables
let vars = HashMap::from([
    ("user", "Alice"),
    ("count", "5"),
]);
let msg = i18n.t_with_vars("summary.stats", vars);
```

#### Adding Messages

1. Edit `src/i18n/messages.json`
2. Add key with English and Japanese translations
3. Use in code with `i18n.t("your.message.key")`

**Example**:
```json
{
  "en": {
    "your.message.key": "Your message in English"
  },
  "ja": {
    "your.message.key": "日本語でのメッセージ"
  }
}
```

### Error Handling

**Location**: `src/core/error.rs`

#### Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum CldevError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("TOML parsing error: {0}")]
    TomlParse(#[from] toml::de::Error),

    #[error("TOML serialization error: {0}")]
    TomlSerialize(#[from] toml::ser::Error),

    // Add more as needed
}
```

#### Usage

```rust
use cldev::core::CldevError;

fn example() -> Result<(), CldevError> {
    let config = Config::load()?;  // Propagates error
    Ok(())
}
```

### Security System

**Location**: `src/core/security.rs`

#### File Permissions

```rust
#[cfg(unix)]
pub fn check_permissions(path: &Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let metadata = std::fs::metadata(path)?;
    let permissions = metadata.permissions();
    let mode = permissions.mode();

    if mode & 0o777 != 0o600 {
        return Err(CldevError::Security(
            format!("Invalid permissions: {:#o}", mode)
        ));
    }
    Ok(())
}
```

---

## Adding Features

### Adding a New Command

#### 1. Define Command Structure

Create file `src/commands/your_command.rs`:

```rust
use clap::Args;
use crate::core::CldevError;
use std::sync::Arc;
use crate::core::Config;

#[derive(Debug, Args)]
pub struct YourCommand {
    /// Your argument
    #[arg(long)]
    pub your_arg: Option<String>,
}

impl YourCommand {
    pub fn execute(&self, config: Arc<Config>) -> Result<(), CldevError> {
        // Implementation
        Ok(())
    }
}
```

#### 2. Register in Args

Edit `src/cli/args.rs`:

```rust
#[derive(Debug, Subcommand)]
pub enum Commands {
    // ... existing commands
    YourCommand(YourCommand),
}
```

#### 3. Add Route in Main

Edit `src/main.rs`:

```rust
match cli.command {
    Commands::YourCommand(cmd) => cmd.execute(config)?,
    // ... other commands
}
```

#### 4. Add Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_your_command() {
        // Test implementation
    }
}
```

#### 5. Add i18n Messages

Edit `src/i18n/messages.json`:

```json
{
  "en": {
    "your.command.success": "Command executed successfully"
  },
  "ja": {
    "your.command.success": "コマンドが正常に実行されました"
  }
}
```

#### 6. Document

- Add to `docs/USER_GUIDE.md`
- Add examples to `docs/guides/`
- Update `CHANGELOG.md`

### Adding a New Language

#### 1. Add Language Enum

Edit `src/core/i18n.rs`:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    English,
    Japanese,
    French,  // Add here
}
```

#### 2. Add Language Code Mapping

```rust
pub fn from_code(code: &str) -> Self {
    match code {
        "en" | "en_US" | "en_GB" => Language::English,
        "ja" | "ja_JP" => Language::Japanese,
        "fr" | "fr_FR" => Language::French,  // Add here
        _ => Language::English,
    }
}

pub fn to_code(&self) -> &'static str {
    match self {
        Language::English => "en",
        Language::Japanese => "ja",
        Language::French => "fr",  // Add here
    }
}
```

#### 3. Add Translations

Edit `src/i18n/messages.json`:

```json
{
  "en": { /* ... */ },
  "ja": { /* ... */ },
  "fr": {
    "config.init.success": "Configuration initialisée avec succès"
    // ... translate all messages
  }
}
```

#### 4. Update Documentation

- Add to `docs/guides/SUPPORTED_LANGUAGES.md`
- Update `docs/USER_GUIDE.md`

---

## Testing

### Unit Tests

Located in module files with `#[cfg(test)]`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.version, "1.0.0");
    }

    #[test]
    fn test_config_serialization() -> Result<(), CldevError> {
        let config = Config::default();
        let toml_string = toml::to_string(&config)?;
        assert!(toml_string.contains("version"));
        Ok(())
    }
}
```

### Integration Tests

Located in `tests/` directory:

```rust
// tests/integration_test.rs
use cldev::core::Config;

#[test]
fn test_full_workflow() {
    // Test complete workflows
}
```

### Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_config_default

# Show output
cargo test -- --nocapture

# Integration tests only
cargo test --test '*'

# With coverage
cargo tarpaulin --out Html
```

### Test Coverage Goals

- **Unit tests**: 100% of public API
- **Integration tests**: All major workflows
- **Overall coverage**: 80%+ target

---

## Documentation

### Code Documentation

Use rustdoc comments for all public items:

```rust
/// Loads configuration from the specified path.
///
/// # Arguments
///
/// * `path` - Optional path to configuration file. If `None`, uses default.
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
/// On Unix systems, validates file permissions (must be 600).
pub fn load(path: Option<PathBuf>) -> Result<Arc<Self>> {
    // implementation
}
```

### Generate Documentation

```bash
# Generate and open docs
cargo doc --open

# With private items
cargo doc --document-private-items --open
```

### User Documentation

- **User Guide**: `docs/USER_GUIDE.md`
- **Quick Start**: `docs/guides/QUICKSTART.md`
- **Examples**: `docs/guides/`

### Developer Documentation

- **This Guide**: `docs/DEVELOPER_GUIDE.md`
- **Architecture**: `docs/architecture/`
- **Implementation**: `docs/implementation/`

---

## Performance

### Benchmarking

```bash
# Install criterion
cargo install cargo-criterion

# Run benchmarks
cargo criterion
```

### Performance Targets

- **Config Load**: <1ms
- **Message Lookup**: <100ns
- **Command Execution**: <10ms startup

### Optimization Tips

1. **Use Arc<Config>**: Share configuration efficiently
2. **Lazy Evaluation**: Compute values only when needed
3. **Minimize Allocations**: Reuse buffers where possible
4. **Profile First**: Use `cargo flamegraph` for profiling

### Memory Usage

- **Binary Size**: ~2-3 MB (release)
- **Runtime Memory**: ~5-10 MB typical
- **Config Memory**: <1 KB

---

## Security

### Security Principles

1. **Secure by Default**: Safe defaults for all settings
2. **Least Privilege**: Minimal required permissions
3. **Input Validation**: Validate all user input
4. **Path Validation**: Prevent path traversal attacks
5. **File Permissions**: Enforce 600 on sensitive files

### Security Checklist

- [ ] Validate all user inputs
- [ ] Check file permissions (Unix)
- [ ] Sanitize path inputs
- [ ] Use type-safe parsing
- [ ] Handle errors securely
- [ ] No secrets in logs
- [ ] Secure temp files

### Reporting Security Issues

Email security issues to: security@example.com (private)

---

## Release Process

### Version Numbering

Follow Semantic Versioning (SemVer):
- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

### Release Checklist

1. **Update Version**
   - `Cargo.toml`: version = "X.Y.Z"
   - `CHANGELOG.md`: Add release date

2. **Run Tests**
   ```bash
   cargo test
   cargo clippy -- -D warnings
   cargo fmt --check
   ```

3. **Build Release**
   ```bash
   cargo build --release
   ```

4. **Create Tag**
   ```bash
   git tag -a vX.Y.Z -m "Release vX.Y.Z"
   git push origin vX.Y.Z
   ```

5. **Publish** (when ready)
   ```bash
   cargo publish
   ```

### Changelog Entry

```markdown
## [X.Y.Z] - 2025-MM-DD

### Added
- New feature description

### Changed
- Changed feature description

### Fixed
- Bug fix description
```

---

## Additional Resources

### Internal Documentation

- [Implementation History](./implementation/DEVELOPMENT_HISTORY.md)
- [Architecture Docs](./architecture/)
- [User Guide](./USER_GUIDE.md)

### External Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [clap Documentation](https://docs.rs/clap/)
- [serde Documentation](https://docs.rs/serde/)
- [thiserror Documentation](https://docs.rs/thiserror/)

### Community

- GitHub Issues: Bug reports and features
- GitHub Discussions: Questions and ideas
- Contributing: See [CONTRIBUTING.md](../CONTRIBUTING.md)

---

**Last Updated**: 2025-11-07
**Documentation Version**: 1.0.0
**cldev Version**: 1.0.0
