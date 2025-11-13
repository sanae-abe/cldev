# cldev - Claude Development CLI

[![Build Status](https://github.com/sanae-abe/cldev/workflows/CI/badge.svg)](https://github.com/sanae-abe/cldev/actions)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)

**cldev** is a unified CLI tool for managing development workflows with Claude Code. It consolidates development commands into a single, type-safe, blazingly fast Rust binary with full i18n support (English/Japanese/Chinese).

**Current Status**: 35 commands implemented

English | [日本語](README.ja.md) | [简体中文](README.zh.md) | [繁體中文](README.zh-TW.md)

<!-- Screenshot placeholder: Add demo GIF showing cldev config init -->

---

## Table of Contents

- [Overview](#overview)
- [Key Features](#key-features)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Usage Examples](#usage-examples)
- [Supported Languages & Tech Stacks](#supported-languages--tech-stacks)
- [Configuration System](#configuration-system)
- [Command Reference](#command-reference)
- [Development](#development)
- [Contributing](#contributing)
- [License](#license)

---

## Overview

**cldev** replaces fragmented shell scripts with a cohesive development toolkit that:

- **Unifies** 35 commands across 9 categories (config, dev, git, quality, tech, ops, analysis, lr, todo)
- **Accelerates** workflows with faster startup (~21ms hyperfine benchmark, 1.5x faster than gh CLI's 32ms)
- **Simplifies** installation (`cargo install cldev`)
- **Internationalizes** all outputs (English/Japanese/Chinese with extensible i18n)
- **Secures** operations (path traversal prevention, command injection protection)
- **Autodetects** project types (Node.js, Rust, Go, Python, etc.)

### Why cldev?

**Before:**
```bash
# Fragmented commands across 3 separate script collections
~/.claude/scripts/claude validate
uc feature user-auth
~/.claude/learning-analytics/context-search.sh "encryption"
```

**After:**
```bash
# Single unified CLI with intelligent defaults
cldev config check
cldev dev feature user-auth
cldev lr find "encryption"
```

**Improvements:**
- 15% reduction in command count (41 → 35)
- 77% shorter command syntax
- 80% faster installation
- 1.5x faster execution (vs gh CLI)
- Full type safety (Rust)
- i18n support (English/Japanese/Chinese Simplified & Traditional)

---

## Key Features

### 🚀 Performance
- **Fast startup**: ~21ms (hyperfine benchmark, cold start ~346ms)
- **Compact binary**: 3.3MB (93% smaller than gh CLI's 51MB)
- **Optimized release builds**: LTO, strip, codegen-units=1
- **Efficient resource usage**: Minimal memory footprint

### 🌐 Internationalization
- **Current support**: English (en), Japanese (ja), Chinese Simplified (zh), Chinese Traditional (zh-TW)
- **Roadmap**: Korean (ko) - Year 2 Q2, additional languages on request
- **Auto-detection**: Uses `LANG` environment variable
- **Extensible**: JSON-based i18n system (upgradable to fluent-rs)

### 🔒 Security
- **Path traversal prevention**: Secure path canonicalization
- **Command injection protection**: Safe command execution
- **Permission validation**: Config file security checks (600)
- **Input validation**: Comprehensive sanitization

### 🎯 Developer Experience
- **Shell completions**: Bash, Zsh, Fish, PowerShell
- **Interactive setup**: Guided configuration wizard
- **Smart autodetection**: Git remotes, project types, tech stacks
- **Rich output**: Colored, formatted, emoji-enhanced (configurable)
- **Comprehensive help**: Detailed `--help` for all commands

### 🏗️ Architecture
- **Modular design**: Clear separation of concerns
- **3-layer config**: Global → Tech Stack → Project
- **Type-safe**: Rust's compile-time guarantees
- **Extensible**: Plugin-ready command system

### 📚 Learning Record System (AI-Powered)
- **AI-assisted error matching**: Find similar errors using Levenshtein distance and pattern normalization
- **Hotspot detection**: Proactive warnings for files with recurring problems
- **Context-aware search**: Composite scoring (files 40%, errors 30%, tags 20%, recency 10%)
- **Built-in knowledge base**: TF-IDF powered search with full UTF-8 support
- **Problem tracking**: Keep track of unresolved issues with intelligent prioritization
- **Learning analytics**: View statistics and patterns with detailed breakdowns

Unlike typical development CLIs, cldev includes an **AI-powered** searchable learning record system designed specifically for Claude Code integration. While many developers manually maintain TIL (Today I Learned) repositories or engineering logs in separate tools, cldev provides:

**🔍 Intelligent Error Matching**
```bash
# AI automatically finds similar errors by normalizing patterns
cldev lr suggest "thread panicked at overflow in main.rs:42"
# Matches: "thread panicked at overflow in lib.rs:123" (85% similarity)
```

**⚠️ Proactive Problem Prevention**
```bash
# Check file hotspot status before editing
cldev lr check-file src/auth/login.rs
# ⚠️  WARNING: This file is a known hotspot!
# Past Issues: JWT validation error (Resolved), Auth timeout (Unresolved)
```

**🎯 Context-Based Discovery**
```bash
# Find sessions similar to your current context
cldev lr similar session-abc123
# Returns sessions with similar files, errors, tags, and recent activity
```

**Basic Usage**
```bash
# Record a learning session
cldev lr new "JWT authentication implementation"

# Search past solutions
cldev lr find "authentication" --field topic

# View learning statistics
cldev lr stats --period week
```

---

## Installation

### Option 1: Cargo (Rust Package Manager)

```bash
# Install from crates.io (coming soon)
# cargo install cldev

# Build from source
git clone https://github.com/sanae-abe/cldev.git
cd cldev
cargo install --path .
```

> **Note**: Pre-built binaries and Homebrew installation will be available in future releases. See [Roadmap](docs/development/IMPLEMENTATION_PLAN.md) for details.

### Verify Installation

```bash
cldev --version
# Output: cldev 1.0.0
```

**📋 Detailed Verification**: Use the [Runtime Test Plan](docs/development/RUNTIME_TEST_PLAN.md) to verify all features are working correctly.

---

## Quick Start

### 1. Initialize Configuration (5 minutes)

Run the interactive setup wizard:

```bash
cldev config init
```

This will:
- Detect your language preference
- Configure Claude Code directory (`~/.claude`)
- Set up project root
- Detect Git CLI tools (gh/glab)
- Install shell completions
- Create configuration file

**Example session:**
```
cldev - Initial Setup
━━━━━━━━━━━━━━━━━━━━━━━━━━

1. Language / 言語
   > English / 日本語
   [日本語]

2. Claude Code directory
   ✓ Detected: /Users/sanae/.claude

3. Project root
   [~/projects]

4. Git CLI
   ✓ GitHub CLI (gh): detected
   - GitLab CLI (glab): not found

5. Shell completion
   Shell: zsh
   Add to: ~/.zshrc

✓ Configuration saved: ~/.config/cldev/config.toml
✓ Shell completion added: ~/.zshrc

Next: source ~/.zshrc
```

### 2. Verify Configuration

```bash
cldev config check
# ✅ Configuration file is valid
# 💡 Next step: cldev dev feature
```

### 3. Start Your First Feature

```bash
cldev dev feature user-authentication
# Guides you through:
# - Branch creation
# - Implementation planning
# - Test scaffolding
# - Commit preparation
```

---

## Usage Examples

### Configuration Management

```bash
# Check configuration health
cldev config check

# Check with detailed validation
cldev config check --detailed --validate

# Edit configuration in your editor
cldev config edit

# List all available commands
cldev config list

# List commands with detailed info
cldev config list --detailed

# Filter commands by category
cldev config list --filter dev

# Monthly maintenance (backup, cleanup, validation)
cldev config maintain --backup --cleanup
```

### Development Workflows

```bash
# Start new feature development
cldev dev feature payment-integration

# Emergency production issue handling
cldev dev urgent "API authentication failing"

# Fix critical bug
cldev dev fix "memory leak in user service"

# Debug with systematic approach
cldev dev debug "slow database queries"

# Refactor code safely
cldev dev refactor src/auth/

# Optimize performance
cldev dev optimize --focus "database queries"

# Research and document
cldev dev research "JWT best practices"
```

### Git Operations

```bash
# Create conventional commit
cldev git commit "feat: add OAuth2 support"

# Create feature branch (conventional naming)
cldev git branch user-profile --type feature

# Create Pull Request (auto-detects GitHub)
cldev git merge-request --title "Add user authentication"

# Create Merge Request (auto-detects GitLab)
cldev git merge-request --title "Fix memory leak"

# Show git status with recommendations
cldev git status --detailed
```

### Code Quality

```bash
# Run linter (auto-detects project type)
cldev quality lint

# Run linter with auto-fix
cldev quality lint --fix

# Format code (auto-detects: Prettier/rustfmt/gofmt)
cldev quality format

# Check formatting without changes
cldev quality format --check

# Run tests
cldev quality test

# Run specific test pattern
cldev quality test --pattern "auth*"

# Run with coverage report
cldev quality test --coverage

# Watch mode for continuous testing
cldev quality test --watch
```

### Tech Stack Operations

```bash
# Start development server (auto-detects project type)
cldev tech start

# Start specific stack
cldev tech start web --port 3000
cldev tech start api --port 8080
cldev tech start mobile
cldev tech start ds  # Data science notebook

# Detached mode
cldev tech start --detach
```

### Operations

```bash
# Build project (auto-detects build system)
cldev ops build

# Build with bundle analysis
cldev ops build --analyze

# Clean build
cldev ops build --clean

# Deploy to environment
cldev ops deploy production

# Dry-run deployment
cldev ops deploy staging --dry-run

# Deploy with auto-confirmation
cldev ops deploy production --yes
```

### Analysis & Code Review

```bash
# Analyze codebase structure
cldev analysis analyze --target structure

# Analyze performance
cldev analysis analyze --target performance --detailed

# Explain technical concept with examples
cldev analysis explain "OAuth2 flow" --examples

# Review merge request
cldev analysis review-mr 42 --detailed

# Security-focused review
cldev analysis review-mr 42 --security-focus

# Performance-focused review
cldev analysis review-mr 42 --performance-focus

# Run semantic analysis (MCP integration)
cldev analysis serena --mode check
```

### Learning Records

```bash
# Create new learning record
cldev lr new "Understanding Rust lifetimes" --edit

# Search learning records
cldev lr find "encryption"

# Recent records
cldev lr find --recent 10

# Search by specific field
cldev lr find "JWT" --field topic

# Check file hotspot status before editing
cldev lr check-file src/auth/login.rs

# Find similar errors (AI-powered error matching)
cldev lr suggest "thread panicked at overflow" --threshold 0.7

# Find sessions similar to a specific session
cldev lr similar session-abc123 --limit 5

# Generate statistics
cldev lr stats

# Weekly statistics
cldev lr stats --period week --detailed

# Analyze problem patterns
cldev lr problems

# High priority problems
cldev lr problems --priority high --recent 20
```

### Todo Management

**Personal TODO management** with Markdown format (`.cldev/TODO.md`)

```bash
# Add todo item (interactive: priority + tags)
cldev todo manage add "Implement rate limiting"

# List all todos (grouped by priority)
cldev todo manage list

# Complete todo (interactive selection)
cldev todo manage complete

# Sync with git commits (auto-complete matching todos)
cldev todo manage sync

# Interactive mode (menu-driven)
cldev todo manage interactive
```

**Features:**
- ✅ Markdown format (`.cldev/TODO.md`) - editable in any editor
- ✅ Priority levels (🔥 Critical, ⚠️ High, 📌 Medium, 📝 Low)
- ✅ Tags support (`#rust #performance`)
- ✅ Git commit integration (auto-complete todos)
- ✅ Project-local or global storage

**Example TODO.md:**
```markdown
# Personal TODOs

## ⚠️ High
- [ ] Learning Record性能改善 #rust #performance (created: 2025-01-09)

## 📌 Medium
- [ ] TF-IDF検索精度向上 #search (created: 2025-01-09)

## ✅ Completed
- [x] READMEのコマンド数修正 (created: 2025-01-09, completed: 2025-01-09)
```

### Shell Completions

```bash
# Generate completions for Zsh
cldev completions zsh > ~/.zsh/completions/_cldev

# Generate for Bash
cldev completions bash > /usr/local/etc/bash_completion.d/cldev

# Generate for Fish
cldev completions fish > ~/.config/fish/completions/cldev.fish

# Print installation instructions
cldev completions zsh --install
```

---

## Supported Languages & Tech Stacks

### Languages

| Language | Detection | Linting | Formatting | Testing |
|----------|-----------|---------|------------|---------|
| **JavaScript** | ✅ package.json | ESLint | Prettier | Jest/Vitest |
| **TypeScript** | ✅ tsconfig.json | ESLint | Prettier | Jest/Vitest |
| **Rust** | ✅ Cargo.toml | Clippy | rustfmt | cargo test |
| **Go** | ✅ go.mod | golangci-lint | gofmt/goimports | go test |
| **Python** | ✅ requirements.txt | pylint/ruff | black/ruff | pytest |
| **Ruby** | ✅ Gemfile | rubocop | rubocop | rspec |
| **Java** | ✅ pom.xml/build.gradle | checkstyle | google-java-format | JUnit |

### Frameworks

| Framework | Detection | Dev Server | Build | Deploy |
|-----------|-----------|------------|-------|--------|
| **React** | ✅ package.json | ✅ vite/next | ✅ | ✅ |
| **Vue** | ✅ package.json | ✅ vite | ✅ | ✅ |
| **Angular** | ✅ angular.json | ✅ ng serve | ✅ | ✅ |
| **Next.js** | ✅ next.config.js | ✅ next dev | ✅ | ✅ |
| **Express** | ✅ package.json | ✅ node | - | ✅ |
| **FastAPI** | ✅ requirements.txt | ✅ uvicorn | - | ✅ |
| **Rails** | ✅ Gemfile | ✅ rails s | - | ✅ |

### Build Tools

- **Node.js**: npm, yarn, pnpm, bun
- **Rust**: cargo
- **Go**: go build, make
- **Python**: pip, poetry, pipenv
- **Java**: maven, gradle

### Git Platforms

- **GitHub**: Automatic detection via `gh` CLI
- **GitLab**: Automatic detection via `glab` CLI
- **Remote detection**: Auto-detects from `.git/config`

---

## Configuration System

### 3-Layer Hierarchy

```
🌍 Global Config (~/.config/cldev/config.toml)
    │ Base settings applicable to all projects
    ▼
🔧 Tech Stack Config (~/.claude/stacks/*.md)
    │ Technology-specific settings (web/api/mobile/data-science)
    ▼
🎯 Project Config (project/.claude/config.toml)
    │ Project-specific overrides
```

### Configuration File Structure

**Location**:
- macOS: `~/Library/Application Support/cldev/config.toml`
- Linux: `~/.config/cldev/config.toml`
- Windows: `%APPDATA%\cldev\config.toml`

```toml
# cldev configuration file
version = "1.0.0"

[general]
language = "ja"  # en, ja, zh, or zh-TW
claude_dir = "/Users/username/.claude"
projects_dir = "/Users/username/projects"

[git]
github_cli = true
gitlab_cli = false
default_base_branch = "main"
auto_push = true

[quality]
auto_fix = false
run_tests_before_commit = true

[dev]
auto_create_branch = true
branch_prefix = "feature"
session_recording = true

[lr]
sessions_dir = "/Users/username/.claude/learnings"
auto_save = true
default_tags = ["development", "claude-code"]

[ui]
color = true
emoji = true
progress_bar = true

[performance]
parallel_tasks = 4
timeout_seconds = 300
```

### Version Management

cldev uses semantic versioning for configuration files:

- **Major version** (1.x.x): Breaking changes, migration required
- **Minor version** (x.1.x): New features, backward compatible
- **Patch version** (x.x.1): Bug fixes, fully compatible

cldev automatically validates and migrates configurations when needed.

---

## Command Reference

### Command Categories

cldev organizes 35 commands into 9 logical categories:

```
┌─────────────────────────────────────────────────────┐
│                   cldev Commands                    │
├─────────────────────────────────────────────────────┤
│ config (6)     │ Configuration management           │
│ dev (7)        │ Development workflows              │
│ git (4)        │ Git operations                     │
│ quality (3)    │ Code quality & testing             │
│ tech (1)       │ Tech stack operations              │
│ ops (2)        │ Build & deployment                 │
│ analysis (4)   │ Code analysis & review             │
│ lr (7)         │ Learning records                   │
│ todo (1)       │ Task management                    │
│ completions    │ Shell completions                  │
└─────────────────────────────────────────────────────┘
```

### Full Command List

#### Config Commands (6)
```bash
cldev config init          # Interactive setup wizard
cldev config check         # Validate configuration
cldev config edit          # Edit config in editor
cldev config list          # List all commands
cldev config maintain      # Monthly maintenance
cldev config update-docs   # Update documentation
```

#### Dev Commands (7)
```bash
cldev dev feature          # New feature development
cldev dev urgent           # Emergency production issue
cldev dev fix              # Critical bug fix
cldev dev debug            # Systematic debugging
cldev dev refactor         # Safe refactoring
cldev dev optimize         # Performance optimization
cldev dev research         # Technical research
```

#### Git Commands (4)
```bash
cldev git commit           # Conventional commit
cldev git branch           # Create branch
cldev git merge-request    # Create PR/MR (auto-detect)
cldev git status           # Status with recommendations
```

#### Quality Commands (3)
```bash
cldev quality lint         # Run linter
cldev quality format       # Format code
cldev quality test         # Run tests
```

#### Tech Commands (1)
```bash
cldev tech start           # Start dev environment (auto-detect)
```

#### Ops Commands (2)
```bash
cldev ops build            # Build project
cldev ops deploy           # Deploy to environment
```

#### Analysis Commands (4)
```bash
cldev analysis analyze     # Code analysis
cldev analysis explain     # Technical explanation
cldev analysis review-mr   # MR/PR review
cldev analysis serena      # Semantic analysis (MCP)
```

#### Learning Record Commands (7)
```bash
cldev lr new               # Create learning record
cldev lr find              # Search records
cldev lr stats             # Generate statistics
cldev lr problems          # Problem pattern analysis
cldev lr check-file        # Check file hotspot status
cldev lr suggest           # Find similar errors
cldev lr similar           # Find similar sessions
```

#### Todo Commands (1)
```bash
cldev todo manage          # Manage todo items
```

#### Shell Completions
```bash
cldev completions <shell>  # Generate completions
```

### Global Options

All commands support these global flags:

```bash
--verbose, -v      # Detailed output
--quiet, -q        # Suppress non-error output
--no-color         # Disable colored output
--lang <LANG>      # Override language (en/ja/zh/zh-TW)
--help, -h         # Show help
--version, -V      # Show version
```

---

## Development

### Prerequisites

- **Rust 1.70+** (install via [rustup](https://rustup.rs/))
- **Git 2.30+**
- Optional: `gh` (GitHub CLI), `glab` (GitLab CLI)

### Building from Source

```bash
# Clone repository
git clone https://github.com/sanae-abe/cldev.git
cd cldev

# Build in debug mode
cargo build

# Build optimized release binary
cargo build --release

# Install locally
cargo install --path .
```

### Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_config_load

# Run integration tests only
cargo test --test '*'

# Generate coverage report (requires cargo-tarpaulin)
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

### Code Quality

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check

# Run linter
cargo clippy

# Run linter with strict checks
cargo clippy --all-targets --all-features -- -D warnings

# Security audit
cargo audit
```

### Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench config_bench
```

#### Startup Time Comparison (実測値)

Measured on macOS 14.6, Apple M2 Pro using [hyperfine](https://github.com/sharkdp/hyperfine):

| Tool | Mean | Min | Max | vs cldev |
|------|------|-----|-----|----------|
| **cldev** | 21.2ms ± 8.3ms | 11.3ms | 41.0ms | 1.0x (baseline) |
| **gh CLI** | 31.8ms ± 1.5ms | 29.0ms | 34.1ms | 1.5x slower |
| **glab** | 126.3ms ± 13.1ms | 111.5ms | 149.9ms | 6.0x slower |

**Binary Size:**
- cldev: 3.3MB (stripped, LTO optimized)
- gh CLI: 51MB

### Project Structure

```
cldev/
├── Cargo.toml              # Package manifest
├── Cargo.lock              # Dependency lock file
├── README.md               # Project overview (this file)
├── CONTRIBUTING.md         # Contribution guidelines
├── CHANGELOG.md            # Version history (Keep a Changelog format)
├── src/
│   ├── main.rs             # Binary entry point
│   ├── lib.rs              # Library exports
│   ├── cli/                # CLI argument parsing and output
│   │   ├── mod.rs          # CLI module exports
│   │   ├── args.rs         # Command definitions (clap)
│   │   └── output.rs       # Output formatting + i18n integration
│   ├── commands/           # Command implementations
│   │   ├── mod.rs          # Command module exports
│   │   └── config/         # Configuration commands
│   │       ├── mod.rs      # Config command exports
│   │       ├── init.rs     # Interactive initialization (Phase 1-B)
│   │       ├── check.rs    # Configuration validation
│   │       └── list.rs     # Configuration display
│   ├── core/               # Core functionality
│   │   ├── mod.rs          # Core module exports
│   │   ├── config.rs       # Configuration management (Phase 1-A)
│   │   ├── i18n.rs         # Internationalization (Phase 1-B)
│   │   ├── error.rs        # Error types
│   │   └── security.rs     # Security utilities
│   └── i18n/               # i18n resources
│       └── messages.json   # Translation catalog (621 keys, 4 languages)
├── tests/                  # Integration tests
│   └── integration_test.rs # Integration tests
├── examples/               # Usage examples
│   └── i18n_demo.rs        # i18n demonstration
├── completions/            # Shell completion scripts (generated)
│   ├── cldev.bash
│   ├── cldev.zsh
│   ├── cldev.fish
│   └── _cldev.ps1
└── docs/                   # Documentation
    ├── USER_GUIDE.md       # Complete user documentation
    ├── DEVELOPER_GUIDE.md  # Developer and contributor guide
    ├── guides/             # User guides and tutorials
    │   ├── QUICKSTART.md
    │   ├── CONFIG_USAGE_EXAMPLES.md
    │   ├── INTERACTIVE_UI_DEMO.md
    │   ├── i18n_quick_start.md
    │   ├── CORE_MODULES_QUICK_REFERENCE.md
    │   └── SUPPORTED_LANGUAGES.md
    ├── architecture/       # Architecture and design
    │   ├── i18n.md
    │   ├── hierarchical-config-system.md
    │   ├── TECH_STACK_COMPARISON.md
    │   ├── RUST_BEST_PRACTICES_REVIEW.md
    │   ├── SECURITY_IMPLEMENTATION.md
    │   └── COMMAND_OPTIMIZATION_ANALYSIS.md
    ├── implementation/     # Implementation details
    │   ├── DEVELOPMENT_HISTORY.md
    │   ├── COMMANDS_IMPLEMENTED.md
    │   ├── IMPLEMENTATION_SUMMARY.md
    │   └── CORE_MODULES_IMPLEMENTATION.md
    └── development/        # Development planning
        ├── IMPLEMENTATION_PLAN.md
        ├── TODO.md
        └── GTM_BUSINESS_STRATEGY.md
```

---

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Workflow

1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/amazing-feature`)
3. **Commit** using conventional commits (`git commit -m 'feat: add amazing feature'`)
4. **Push** to your fork (`git push origin feature/amazing-feature`)
5. **Open** a Pull Request

### Commit Convention

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: Add new feature
fix: Fix bug
docs: Update documentation
style: Format code
refactor: Refactor code
perf: Improve performance
test: Add tests
chore: Update dependencies
```

### Code Review Process

1. **Automated checks**: CI must pass (tests, lints, formatting)
2. **Security review**: All code reviewed for security implications
3. **Performance review**: No performance regressions
4. **Documentation**: All public APIs documented
5. **Tests**: New features require tests

---

## License

This project is dual-licensed under:

- **MIT License** ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- **Apache License 2.0** ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

You may choose either license for your use.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

---

## Documentation

### Quick Links

- **[User Guide](docs/USER_GUIDE.md)**: Complete user documentation
- **[Developer Guide](docs/DEVELOPER_GUIDE.md)**: Contributing and development guide
- **[Quick Start](docs/guides/QUICKSTART.md)**: Get started in 5 minutes
- **[Contributing](CONTRIBUTING.md)**: How to contribute
- **[Changelog](CHANGELOG.md)**: Version history and changes

### Documentation Structure

```
docs/
├── USER_GUIDE.md              # Complete user documentation
├── DEVELOPER_GUIDE.md         # Developer and contributor guide
├── guides/                    # User guides and tutorials
│   ├── QUICKSTART.md          # Quick start guide
│   ├── CONFIG_USAGE_EXAMPLES.md    # Configuration examples
│   ├── INTERACTIVE_UI_DEMO.md      # Interactive UI walkthrough
│   ├── i18n_quick_start.md         # i18n quick start
│   ├── CORE_MODULES_QUICK_REFERENCE.md  # Core modules reference
│   └── SUPPORTED_LANGUAGES.md      # Language support
├── architecture/              # Architecture and design
│   ├── i18n.md                # Internationalization system
│   ├── hierarchical-config-system.md  # Configuration architecture
│   ├── TECH_STACK_COMPARISON.md       # Technology analysis
│   ├── RUST_BEST_PRACTICES_REVIEW.md  # Rust best practices
│   ├── SECURITY_IMPLEMENTATION.md     # Security design
│   └── COMMAND_OPTIMIZATION_ANALYSIS.md  # Command optimization
├── implementation/            # Implementation details
│   ├── DEVELOPMENT_HISTORY.md        # Complete development history
│   ├── COMMANDS_IMPLEMENTED.md       # Command implementation status
│   ├── IMPLEMENTATION_SUMMARY.md     # Implementation summary
│   └── CORE_MODULES_IMPLEMENTATION.md  # Core modules details
└── development/              # Development planning
    ├── IMPLEMENTATION_PLAN.md  # Implementation roadmap
    ├── TODO.md                 # Task tracking
    └── GTM_BUSINESS_STRATEGY.md  # Go-to-market strategy
```

### By Role

**For Users:**
- Start with [Quick Start Guide](docs/guides/QUICKSTART.md)
- Read [User Guide](docs/USER_GUIDE.md) for complete documentation
- Check [Configuration Examples](docs/guides/CONFIG_USAGE_EXAMPLES.md) for setup

**For Contributors:**
- Read [Contributing Guidelines](CONTRIBUTING.md)
- Study [Developer Guide](docs/DEVELOPER_GUIDE.md)
- Review [Development History](docs/implementation/DEVELOPMENT_HISTORY.md)

**For Architects:**
- Review [Architecture Docs](docs/architecture/)
- Study [Implementation Plan](docs/development/IMPLEMENTATION_PLAN.md)
- Check [Security Implementation](docs/architecture/SECURITY_IMPLEMENTATION.md)

---

## Support

- **Issues**: [GitHub Issues](https://github.com/sanae-abe/cldev/issues)
- **Discussions**: [GitHub Discussions](https://github.com/sanae-abe/cldev/discussions)
- **Documentation**: [docs/](docs/)

---

## Acknowledgments

Built with:
- [clap](https://github.com/clap-rs/clap) - Command-line argument parsing
- [serde](https://github.com/serde-rs/serde) - Serialization framework
- [tokio](https://tokio.rs/) - Async runtime
- [anyhow](https://github.com/dtolnay/anyhow) - Error handling
- All other amazing Rust crates in [Cargo.toml](Cargo.toml)

Inspired by modern CLI tools:
- [ripgrep](https://github.com/BurntSushi/ripgrep)
- [exa](https://github.com/ogham/exa)
- [bat](https://github.com/sharkdp/bat)
- [fd](https://github.com/sharkdp/fd)

---

**Made with ❤️ by the cldev team**

*Empowering developers with unified, intelligent CLI workflows*
