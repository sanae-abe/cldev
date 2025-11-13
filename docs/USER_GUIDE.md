# cldev User Guide

**Version**: 1.0.0
**Last Updated**: 2025-11-07

## Table of Contents

- [Introduction](#introduction)
- [Installation](#installation)
- [Getting Started](#getting-started)
- [Configuration](#configuration)
- [Using cldev](#using-cldev)
- [Command Reference](#command-reference)
- [Language Support](#language-support)
- [Best Practices](#best-practices)
- [Troubleshooting](#troubleshooting)
- [FAQ](#faq)

---

## Introduction

### What is cldev?

cldev is a command-line interface tool designed to streamline development workflows when working with Claude Code. It provides:

- Interactive configuration management
- Multi-language support (English and Japanese)
- Shell completion generation
- Intelligent development workflows
- Learning session recording
- Git integration

### Key Features

- **Interactive Setup**: User-friendly wizard for initial configuration
- **i18n Support**: Full internationalization with automatic language detection
- **Shell Integration**: Completion support for zsh, bash, fish, and PowerShell
- **Type Safety**: Built with Rust for reliability and performance
- **Security First**: Secure configuration file handling with proper permissions
- **Extensible**: Modular architecture for future enhancements

---

## Installation

### Prerequisites

- Rust 1.70 or later
- Git (optional, for Git integration features)
- GitHub CLI (`gh`) or GitLab CLI (`glab`) (optional)

### From Source

```bash
# Clone the repository
git clone https://github.com/YOUR_USERNAME/cldev
cd cldev

# Build release version
cargo build --release

# Install to system
cargo install --path .
```

### Verify Installation

```bash
# Check version
cldev --version

# View help
cldev --help
```

---

## Getting Started

### Initial Setup

Run the interactive setup wizard:

```bash
cldev config init
```

The wizard will guide you through:

1. **Language Selection**: Choose between English and Japanese
2. **Claude Directory**: Auto-detect or specify your ~/.claude directory
3. **Projects Directory**: Set your default projects location
4. **Git CLI**: Detect installed Git CLI tools (gh/glab)
5. **Shell Completion**: Optionally add shell completions
6. **Aliases**: Optionally create command aliases

### Quick Setup (Non-Interactive)

Use default values for quick setup:

```bash
cldev config init --defaults
```

### Language Selection

Set your preferred language:

```bash
# Japanese
cldev config init --lang ja

# English
cldev config init --lang en
```

Or set via environment variable:

```bash
# Japanese
export LANG=ja_JP.UTF-8
cldev config init

# English
export LANG=en_US.UTF-8
cldev config init
```

---

## Configuration

### Configuration File

Location: `~/.config/cldev/config.toml`

### Configuration Structure

```toml
version = "1.0.0"

[general]
language = "ja"
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

### Managing Configuration

#### View Current Configuration

```bash
cldev config check
```

#### Validate Configuration

```bash
cldev config check --detailed
```

#### List All Settings

```bash
cldev config list
```

#### Reinitialize Configuration

```bash
# Force overwrite existing configuration
cldev config init --force
```

### Configuration Options Explained

#### General Settings

- **language**: Interface language (`ja` or `en`)
- **claude_dir**: Claude Code configuration directory
- **projects_dir**: Default directory for projects

#### Git Settings

- **github_cli**: Enable GitHub CLI (`gh`) integration
- **gitlab_cli**: Enable GitLab CLI (`glab`) integration
- **default_base_branch**: Default branch for operations (`main` or `master`)
- **auto_push**: Automatically push commits to remote

#### Quality Settings

- **auto_fix**: Automatically fix linting issues
- **run_tests_before_commit**: Run tests before creating commits

#### Development Settings

- **auto_create_branch**: Automatically create branches for features/fixes
- **branch_prefix**: Default prefix for branch names
- **session_recording**: Enable learning session recording

#### Learning Record Settings

- **sessions_dir**: Directory for learning session storage
- **auto_save**: Automatically save learning sessions
- **default_tags**: Default tags for learning sessions

#### UI Settings

- **color**: Enable colored output
- **emoji**: Enable emoji in output
- **progress_bar**: Show progress bars for long operations

#### Performance Settings

- **parallel_tasks**: Number of parallel tasks (1-8)
- **timeout_seconds**: Operation timeout in seconds

---

## Using cldev

### Basic Commands

#### Get Help

```bash
# General help
cldev --help

# Command-specific help
cldev config --help
cldev config init --help
```

#### Version Information

```bash
cldev --version
```

### Configuration Commands

#### Initialize Configuration

```bash
# Interactive setup
cldev config init

# With options
cldev config init --lang ja --defaults
cldev config init --force --verbose
```

**Options:**
- `--lang <LANG>`: Set language (ja/en)
- `--defaults`: Use default values (non-interactive)
- `--force`: Overwrite existing configuration
- `--verbose`: Show detailed output
- `--quiet`: Suppress non-error output
- `--no-color`: Disable colored output

#### Check Configuration

```bash
# Basic validation
cldev config check

# Detailed validation
cldev config check --detailed
```

#### List Configuration

```bash
# Show all settings
cldev config list

# Show specific section
cldev config list --section general
cldev config list --section git
```

---

## Command Reference

### cldev config init

Initialize cldev configuration with interactive setup wizard.

**Usage:**
```bash
cldev config init [OPTIONS]
```

**Options:**
- `-d, --defaults` - Skip interactive prompts and use defaults
- `-f, --force` - Force initialization even if config exists
- `-v, --verbose` - Enable verbose output
- `-q, --quiet` - Suppress non-error output
- `--no-color` - Disable colored output
- `--lang <LANG>` - Set language (ja/en) [default: en]

**Examples:**
```bash
# Interactive setup in Japanese
cldev config init --lang ja

# Quick setup with defaults
cldev config init --defaults

# Reinitialize configuration
cldev config init --force
```

### cldev config check

Validate configuration file and settings.

**Usage:**
```bash
cldev config check [OPTIONS]
```

**Options:**
- `--detailed` - Show detailed validation information
- `-v, --verbose` - Enable verbose output

**Examples:**
```bash
# Basic validation
cldev config check

# Detailed validation with verbose output
cldev config check --detailed --verbose
```

### cldev config list

Display configuration settings.

**Usage:**
```bash
cldev config list [OPTIONS]
```

**Options:**
- `--section <SECTION>` - Show specific section only
- `--format <FORMAT>` - Output format (text/json/toml)

**Examples:**
```bash
# Show all settings
cldev config list

# Show Git settings only
cldev config list --section git

# Export as JSON
cldev config list --format json
```

---

## Language Support

### Supported Languages

- **English** (`en`)
- **Japanese** (`ja`)

### Automatic Language Detection

cldev automatically detects your system language from:
1. `LANG` environment variable
2. `LC_ALL` environment variable
3. Falls back to English if not detected

### Changing Language

#### Using Command-Line Flag (Temporary)

```bash
# Use Japanese for a single command
cldev --lang ja config check

# Use English explicitly
cldev --lang en config check
```

#### During Setup

```bash
cldev config init --lang ja
```

#### Via Environment Variable

```bash
# Temporary (current session)
export CLDEV_LANG=ja
cldev config check

# Or use system locale
export LANG=ja_JP.UTF-8
cldev config check

# Permanent (add to ~/.bashrc or ~/.zshrc)
echo 'export CLDEV_LANG=ja' >> ~/.bashrc
```

#### Via Configuration File

Edit `~/.config/cldev/config.toml`:

```toml
[general]
language = "ja"
```

### Available Messages

cldev supports **1,079 localized messages** across all commands, covering:
- Command execution messages
- Configuration management prompts
- File operation notifications
- Validation messages
- UI elements
- Progress indicators
- Error messages
- Interactive workflows
- Status reports

All 33 commands across 9 categories are fully internationalized.

For complete list, see [docs/guides/SUPPORTED_LANGUAGES.md](./guides/SUPPORTED_LANGUAGES.md)

---

## Best Practices

### Configuration Management

1. **Use Interactive Setup**: Run `cldev config init` for initial setup
2. **Validate Regularly**: Run `cldev config check` after manual edits
3. **Backup Configuration**: Keep backups of `~/.config/cldev/config.toml`
4. **Version Control**: Store project-specific settings in version control

### Security

1. **File Permissions**: cldev automatically sets config file to 600 (owner only)
2. **Never Commit Secrets**: Do not store API keys in configuration
3. **Regular Updates**: Keep cldev updated for security patches

### Performance

1. **Adjust Parallel Tasks**: Set `parallel_tasks` based on your system (2-8)
2. **Increase Timeout**: For slow networks, increase `timeout_seconds`
3. **Disable Features**: Disable unused features for faster operation

### Workflow Integration

1. **Shell Completion**: Install completions for faster command entry
2. **Aliases**: Create shortcuts for frequently used commands
3. **Learning Sessions**: Enable session recording for knowledge retention

---

## Troubleshooting

### Common Issues

#### Configuration Not Found

**Problem**: `cldev config check` reports "Configuration file not found"

**Solution**:
```bash
cldev config init
```

#### Permission Denied

**Problem**: Cannot read/write configuration file

**Solution**:
```bash
# Fix permissions
chmod 600 ~/.config/cldev/config.toml

# Or reinitialize
cldev config init --force
```

#### Language Not Detected

**Problem**: Wrong language displayed

**Solution**:
```bash
# Set language explicitly
cldev config init --lang ja

# Or set environment variable
export LANG=ja_JP.UTF-8
```

#### Shell Completion Not Working

**Problem**: Tab completion not functioning

**Solution**:
```bash
# Regenerate completions
cldev completions zsh > ~/.zsh/completions/_cldev

# Reload shell configuration
source ~/.zshrc
```

### Getting Help

If you encounter issues:

1. Check configuration: `cldev config check --detailed`
2. Review logs: `cldev --verbose`
3. Consult documentation: `docs/`
4. Report issue: GitHub Issues

---

## FAQ

### General Questions

**Q: What is cldev?**
A: cldev is a CLI tool for streamlining development workflows with Claude Code.

**Q: Is cldev free?**
A: Yes, cldev is open source (license to be determined).

**Q: Which platforms are supported?**
A: macOS, Linux, and Windows (with cross-platform Rust implementation).

### Installation Questions

**Q: Do I need Rust installed?**
A: Yes, Rust 1.70+ is required to build cldev from source.

**Q: Can I install via package manager?**
A: Not yet. Currently, installation is via cargo only.

### Configuration Questions

**Q: Where is the configuration file stored?**
A: `~/.config/cldev/config.toml` on Unix systems.

**Q: Can I have multiple configurations?**
A: Currently, one global configuration is supported. Project-specific configs are planned.

**Q: How do I reset to defaults?**
A: Run `cldev config init --defaults --force`

### Feature Questions

**Q: Does cldev support my shell?**
A: Yes, zsh, bash, fish, and PowerShell are supported.

**Q: Can I add custom commands?**
A: Not yet. Plugin system is planned for future releases.

**Q: Does cldev work offline?**
A: Yes, core features work offline. Some features may require internet.

### Troubleshooting Questions

**Q: Configuration validation fails. What should I do?**
A: Run `cldev config check --detailed` for specific error messages.

**Q: How do I report a bug?**
A: Open an issue on GitHub with details and error messages.

**Q: Where can I find more help?**
A: Check `docs/` directory or GitHub Discussions.

---

## Additional Resources

### Documentation

- [Quick Start Guide](./guides/QUICKSTART.md) - Fast introduction
- [Configuration Examples](./guides/CONFIG_USAGE_EXAMPLES.md) - Example configurations
- [Interactive UI Demo](./guides/INTERACTIVE_UI_DEMO.md) - UI walkthrough
- [Developer Guide](./DEVELOPER_GUIDE.md) - For contributors

### Architecture

- [i18n System](./architecture/i18n.md) - Internationalization design
- [Config System](./architecture/hierarchical-config-system.md) - Configuration architecture
- [Security](./architecture/SECURITY_IMPLEMENTATION.md) - Security implementation

### Development

- [Contributing](../CONTRIBUTING.md) - Contribution guidelines
- [Changelog](../CHANGELOG.md) - Version history
- [Implementation Plan](./development/IMPLEMENTATION_PLAN.md) - Roadmap

---

## Feedback and Contributions

We welcome feedback and contributions!

- **Bug Reports**: Open an issue on GitHub
- **Feature Requests**: Open an issue with [Feature Request] tag
- **Contributions**: See [CONTRIBUTING.md](../CONTRIBUTING.md)
- **Questions**: GitHub Discussions

---

**Last Updated**: 2025-11-07
**Documentation Version**: 1.0.0
**cldev Version**: 1.0.0
