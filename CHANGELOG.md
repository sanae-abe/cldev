# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

#### Complete Internationalization (i18n) Migration (Nov 14-28, 2025)
- **Full multilingual support** for all 33 commands across 9 categories
- **Languages supported**: English (en), Japanese (ja)
- **Total translation keys**: 1,079 per language
- **Test coverage**: 627 tests, all passing

**Commands Internationalized**:
- **Config (6)**: init, check, list, edit, maintain, update-docs
- **Dev (7)**: feature, fix, debug, urgent, refactor, optimize, research
- **Git (4)**: branch, commit, status, merge-request
- **Quality (3)**: format, lint, test
- **Ops (2)**: build, deploy
- **Tech (1)**: start
- **Analysis (4)**: analyze, explain, review-mr, serena
- **Learning Records (6)**: new, find, stats, problems, similar, suggest
- **Todo (1)**: manage

**Migration Details**:
- Reduced language support from 4 (en/ja/zh/zh-TW) to 2 (en/ja) for maintainability
- All user-facing strings now use `OutputHandler` i18n methods
- Removed 2 obsolete test files for Chinese language support
- Language selection via `--lang` flag or `CLDEV_LANG` environment variable
- Internal error messages remain in English (developer-focused)

**Documentation Updates**:
- Updated `docs/i18n-implementation-progress.md` with complete migration summary
- Added implementation patterns and translation guidelines
- Documented key naming conventions: `{category}-{subcategory}-{detail}`

**Breaking Changes**:
- Removed support for Chinese Simplified (zh) and Chinese Traditional (zh-TW)
- Only `en` and `ja` are valid values for `--lang` flag

### Planned
- Phase 2: High-frequency commands implementation
- Git operations integration
- Learning record management
- Project scaffolding

---

## [1.0.0] - 2025-11-07

### Added

#### Core Configuration System (Phase 1-A)
- TOML-based configuration management system
- Semantic versioning with validation (major.minor.patch)
- Thread-safe `Arc<Config>` ownership model
- Configuration structures:
  - `GeneralConfig` - Language and directory settings
  - `GitConfig` - Git CLI integration settings
  - `QualityConfig` - Code quality settings
  - `DevConfig` - Development workflow settings
  - `LearningRecordConfig` - Learning record settings
  - `UiConfig` - UI preference settings
  - `PerformanceConfig` - Performance tuning settings
- Default configuration values (Japanese language, ~/.claude, ~/projects)
- Configuration file location: `~/.config/cldev/config.toml`
- Version compatibility checking system
- Migration support for future versions

#### Security Features (Phase 1-A)
- Unix file permissions validation (600 - owner read/write only)
- Automatic permission setting on configuration save
- Path traversal prevention
- Secure default configuration
- Path canonicalization for security

#### Interactive UI System (Phase 1-B)
- `dialoguer` integration for interactive prompts:
  - `Select` widget for language selection
  - `Input` widget for directory paths
  - `Confirm` widget for yes/no prompts
  - `ColorfulTheme` for consistent UI styling
- `indicatif` integration for progress visualization:
  - `ProgressBar` with custom styling
  - 5-step configuration generation process
  - Spinner, elapsed time, and progress indicators
- Interactive setup wizard (`cldev config init`):
  - Language selection (English / 日本語)
  - Claude Code directory auto-detection
  - Project root directory configuration
  - Git CLI detection (gh/glab)
  - Shell completion setup (zsh/bash/fish/PowerShell)
  - Alias configuration

#### Internationalization System (Phase 1-B)
- Comprehensive i18n system with zero-cost abstractions
- Language support:
  - English (en)
  - Japanese (ja)
- Automatic language detection from environment (LANG, LC_ALL)
- Message catalog with 63 message keys, 126 translations
- JSON-based translation storage (`src/i18n/messages.json`)
- Message categories:
  - Command execution (2 messages)
  - Configuration management (9 messages)
  - File operations (5 messages)
  - Validation (7 messages)
  - UI elements (26 messages)
  - Progress indicators (9 messages)
  - General utilities (5 messages)
- `I18n` handler with:
  - O(1) HashMap-based message lookup
  - Single variable substitution (`t_format`)
  - Multiple variable substitution (`t_with_vars`)
  - Comprehensive fallback chain (current lang → English → key)
  - Runtime language switching support
- `OutputHandler` integration:
  - `i18n()` - Get i18n handler reference
  - `t(key)` - Get localized message
  - `t_with_vars(key, vars)` - Get message with variables
  - `t_format(key, name, value)` - Get message with single variable

#### Testing Infrastructure
- Comprehensive test suite:
  - Phase 1-A: 9 unit tests (configuration system)
  - Phase 1-B: 12 unit tests (i18n system)
  - Total: 36 tests, all passing
- Test coverage:
  - Configuration serialization/deserialization
  - Version validation
  - File permissions (Unix)
  - Arc reference counting
  - Language detection and switching
  - Message retrieval and variable substitution
  - Fallback mechanisms
- Integration tests:
  - Full workflow coverage
  - Demo programs (examples/i18n_demo.rs)

#### Documentation
- User guides:
  - `docs/guides/QUICKSTART.md` - Quick start guide
  - `docs/guides/CONFIG_USAGE_EXAMPLES.md` - Configuration examples
  - `docs/guides/INTERACTIVE_UI_DEMO.md` - Interactive UI walkthrough
  - `docs/guides/i18n_quick_start.md` - i18n quick start
  - `docs/guides/CORE_MODULES_QUICK_REFERENCE.md` - Core modules reference
  - `docs/guides/SUPPORTED_LANGUAGES.md` - Language support documentation
- Architecture documentation:
  - `docs/architecture/i18n.md` - i18n system design (361 lines)
  - `docs/architecture/hierarchical-config-system.md` - Configuration architecture
  - `docs/architecture/TECH_STACK_COMPARISON.md` - Technology stack analysis
  - `docs/architecture/RUST_BEST_PRACTICES_REVIEW.md` - Rust best practices
  - `docs/architecture/SECURITY_IMPLEMENTATION.md` - Security design
  - `docs/architecture/COMMAND_OPTIMIZATION_ANALYSIS.md` - Command optimization
- Implementation documentation:
  - `docs/implementation/DEVELOPMENT_HISTORY.md` - Complete development history
  - `docs/implementation/COMMANDS_IMPLEMENTED.md` - Command implementation status
  - `docs/implementation/IMPLEMENTATION_SUMMARY.md` - Implementation summary
  - `docs/implementation/CORE_MODULES_IMPLEMENTATION.md` - Core modules details
- Development documentation:
  - `docs/development/IMPLEMENTATION_PLAN.md` - Implementation roadmap
  - `docs/development/TODO.md` - Task tracking
  - `docs/development/GTM_BUSINESS_STRATEGY.md` - Go-to-market strategy
- Project documentation:
  - `CONTRIBUTING.md` - Contribution guidelines
  - `CHANGELOG.md` - This file
  - `README.md` - Project overview

#### Dependencies
- Core dependencies:
  - `serde = "1.0"` with `derive` feature - Serialization
  - `toml = "0.8"` - TOML parsing
  - `dirs = "5.0"` - Cross-platform directory resolution
  - `clap = "4.5"` with `derive` and `cargo` features - CLI parsing
  - `thiserror = "1.0"` - Error type derivation
  - `dialoguer = "0.11"` - Interactive prompts
  - `indicatif = "0.17"` - Progress indicators
- Development dependencies:
  - `tempfile = "3.10"` - Temporary file testing

#### Commands Implemented
- `cldev config init` - Interactive configuration initialization
- `cldev config check` - Configuration validation
- `cldev --version` - Version information
- `cldev --help` - Help information

### Performance
- Zero-cost message embedding (compile-time inclusion)
- O(1) message lookup with HashMap
- Arc<Config> for efficient configuration sharing
- Minimal allocations in hot paths
- Fast TOML parsing with serde
- Lazy evaluation of default paths
- Binary size impact: +5.4 KB (embedded translations)
- Build time: ~6s (full), ~0.2s (incremental)

### Quality Metrics
- Code coverage: 100% of public API
- All clippy lints passing
- rustfmt applied consistently
- No compiler warnings (except development-specific)
- Comprehensive error handling
- No panics in production code
- Type-safe throughout
- Platform-specific code properly isolated

---

## Release Statistics

### v1.0.0 - Phase 1 Completion

**Lines of Code:**
- Implementation: ~2,000+ lines
- Tests: 36 tests (embedded in modules)
- Documentation: ~945+ lines
- Total: ~3,000+ lines

**Features:**
- Commands: 4 implemented
- Message keys: 63
- Translations: 126 (2 languages)
- Test coverage: 100% of public API

**Performance:**
- Language detection: ~1μs
- Message retrieval: ~50ns
- Variable substitution: ~200ns per variable
- Configuration load: <1ms

---

## Migration Guide

### From 0.x to 1.0

Version 1.0.0 is the initial release. No migration required.

### Configuration Version Compatibility

The configuration system supports semantic versioning:
- **Major version**: Must match exactly (breaking changes)
- **Minor version**: Backward compatible (new features)
- **Patch version**: Backward compatible (bug fixes)

Configuration files with version `1.x.x` are compatible with cldev 1.0.0.
Configuration files with version `2.x.x` will require migration (future).

---

## Links

- [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
- [Semantic Versioning](https://semver.org/spec/v2.0.0.html)
- [Repository](https://github.com/YOUR_USERNAME/cldev)

---

**Note**: This changelog follows the "Keep a Changelog" format and semantic versioning principles. All notable changes will be documented here for each release.
