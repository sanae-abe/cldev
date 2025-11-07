# cldev Quick Start Guide

## Installation

### From Source
```bash
git clone https://github.com/sanae-abe/cldev
cd cldev
cargo build --release
cargo install --path .
```

### Shell Completions (Optional but Recommended)

```bash
# Bash
cldev completions bash > ~/.local/share/bash-completion/completions/cldev
source ~/.bashrc

# Zsh
mkdir -p ~/.zsh/completions
cldev completions zsh > ~/.zsh/completions/_cldev
# Add to ~/.zshrc: fpath=(~/.zsh/completions $fpath)

# Fish
cldev completions fish > ~/.config/fish/completions/cldev.fish
```

## First Steps

### 1. Initialize Configuration
```bash
# Interactive setup
cldev config init

# Quick setup with defaults
cldev config init --defaults

# Force overwrite existing config
cldev config init --force
```

### 2. Verify Installation
```bash
# Check all configurations
cldev config check

# Detailed validation
cldev config check --detailed

# View all available commands
cldev config list
```

## Common Workflows

### Emergency Production Issue
```bash
# Start urgent response workflow
cldev dev urgent "API authentication failing"

# With verbose output
cldev dev urgent "Database connection timeout" --verbose
```

**What it does**:
1. Guides through impact assessment
2. Helps classify severity (P0-P3)
3. Provides immediate action checklist
4. Suggests investigation steps
5. Offers mitigation strategies
6. Generates rollback guide
7. Documents incident details
8. Records learning session

### Fix a Critical Bug
```bash
# Start bug fix workflow
cldev dev fix "User profile update fails"

# Create fix branch automatically
cldev dev fix "Payment processing error" --branch
```

**What it does**:
1. Classifies bug type
2. Assesses reproducibility
3. Captures reproduction steps
4. Guides root cause analysis
5. Suggests fix patterns
6. Plans implementation
7. Creates testing checklist
8. Generates commit message
9. Records learning session

### Debug an Issue
```bash
# Start debugging workflow
cldev dev debug "React component not rendering"

# With verbose debugging
cldev dev debug "Memory leak in Node.js app" --verbose
```

**What it does**:
1. Classifies issue type
2. Identifies environment
3. Provides investigation framework
4. Guides log analysis
5. Captures reproduction steps
6. Suggests debugging techniques
7. Provides tool-specific commands
8. Tracks hypothesis and evidence
9. Records debugging session

### Git Operations

#### Create Conventional Commit
```bash
# Interactive commit message
cldev git commit

# With message
cldev git commit "feat: add user authentication"

# Amend previous commit
cldev git commit --amend
```

#### Create Conventional Branch
```bash
# Interactive branch creation
cldev git branch

# With branch type
cldev git branch --branch-type feature

# Named branch
cldev git branch "user-authentication"
```

#### Create Pull Request / Merge Request
```bash
# Auto-generate PR/MR
cldev git merge-request

# To specific branch
cldev git merge-request --target develop

# Detailed mode
cldev git merge-request --detailed
```

#### Enhanced Git Status
```bash
# Standard status
cldev git status

# Detailed status
cldev git status --detailed
```

### Code Quality

#### Lint Code
```bash
# Lint entire project
cldev quality lint

# Auto-fix issues
cldev quality lint --fix

# Lint specific files
cldev quality lint src/components/
```

#### Format Code
```bash
# Format entire project
cldev quality format

# Check formatting only
cldev quality format --check

# Format specific files
cldev quality format src/
```

#### Run Tests
```bash
# Run all tests
cldev quality test

# With coverage
cldev quality test --coverage

# Watch mode
cldev quality test --watch

# Specific pattern
cldev quality test "user.*test"
```

### Configuration Management

#### List All Commands
```bash
# All commands
cldev config list

# Detailed view
cldev config list --detailed

# Filter by type
cldev config list --filter project
```

#### Edit Configuration
```bash
# Edit global config
cldev config edit global

# Edit project config
cldev config edit project

# Edit stack config
cldev config edit stack
```

#### Maintain Configurations
```bash
# Backup and cleanup
cldev config maintain --backup --cleanup

# Just backup
cldev config maintain --backup
```

## Global Options

### Verbose Output
```bash
# See detailed debugging information
cldev --verbose dev debug "issue"
cldev -v quality lint
```

### Quiet Mode
```bash
# Suppress non-error output
cldev --quiet quality lint
cldev -q git status
```

### Disable Colors
```bash
# For piping or non-TTY environments
cldev --no-color git status
cldev --no-color quality test > test-results.txt
```

### Language Selection
```bash
# Use Japanese
cldev --lang ja config init

# Use English (default)
cldev --lang en dev urgent "問題"
```

## Pro Tips

### 1. Combine with Shell Aliases
```bash
# Add to ~/.bashrc or ~/.zshrc
alias cdu='cldev dev urgent'
alias cdf='cldev dev fix'
alias cdd='cldev dev debug'
alias cgc='cldev git commit'
alias cgb='cldev git branch'
alias cqs='cldev quality test'
alias cql='cldev quality lint --fix'
alias cqf='cldev quality format'
```

### 2. Use in CI/CD Pipeline
```bash
# .gitlab-ci.yml or .github/workflows/ci.yml
lint:
  script:
    - cldev quality lint

test:
  script:
    - cldev quality test --coverage

format-check:
  script:
    - cldev quality format --check
```

### 3. Pre-commit Hook
```bash
# .git/hooks/pre-commit
#!/bin/bash
cldev quality lint --fix
cldev quality format
cldev quality test
```

### 4. Daily Workflow
```bash
# Morning routine
cldev config check
cldev git status --detailed

# Before commit
cldev quality lint --fix
cldev quality format
cldev quality test
cldev git commit

# End of day
cldev git merge-request
```

## Configuration Examples

### Global Configuration (~/.config/cldev/config.toml)
```toml
version = "1.0.0"
language = "en"

[git]
default_branch = "main"
commit_emoji = true
auto_push = false

[quality]
auto_fix = true
coverage_threshold = 80

[session]
auto_record = true
record_directory = "~/.cldev/sessions"
```

### Project Configuration (.cldev/config.toml)
```toml
version = "1.0.0"

[project]
type = "frontend-web"
tech_stack = "react-typescript"

[git]
default_branch = "develop"

[quality]
lint_command = "npm run lint"
format_command = "npm run format"
test_command = "npm test"
```

## Troubleshooting

### Command Not Found
```bash
# Ensure cldev is in PATH
which cldev

# Or use full path
/path/to/cldev --help

# Reinstall if needed
cargo install --path . --force
```

### Configuration Issues
```bash
# Check configuration health
cldev config check --detailed

# Reset to defaults
cldev config init --defaults --force

# View current config
cldev config list --detailed
```

### Git Operations Failing
```bash
# Check git status
git status

# Ensure in git repository
git rev-parse --is-inside-work-tree

# Check git config
git config --list
```

### Linter/Formatter Not Found
```bash
# cldev auto-detects project type
# Install required tools:

# For JavaScript/TypeScript
npm install -D eslint prettier

# For Rust
rustup component add clippy rustfmt

# For Python
pip install pylint black

# For Go
go install github.com/golangci/golangci-lint/cmd/golangci-lint@latest
```

## Learning Sessions

All dev commands (urgent, fix, debug) automatically save learning sessions:

### View Session Location
Sessions are saved to `~/.cldev/sessions/` organized by date.

### Session Format
```json
{
  "id": "uuid",
  "session_type": "urgent",
  "topic": "API authentication failing",
  "created_at": "2025-11-07T14:00:00Z",
  "tags": ["production", "incident", "P0-Critical"],
  "files_affected": ["src/auth/index.ts"],
  "steps": ["..."],
  "root_cause": "JWT key rotation issue",
  "solution": "Rolled back to previous key",
  "resolved": true,
  "duration_minutes": 15,
  "metadata": {}
}
```

## Getting Help

### Command Help
```bash
# General help
cldev --help

# Category help
cldev dev --help
cldev git --help
cldev quality --help
cldev config --help

# Specific command help
cldev dev urgent --help
cldev git commit --help
cldev quality lint --help
```

### Documentation
- Implementation Summary: `IMPLEMENTATION_SUMMARY.md`
- This Quick Start: `QUICKSTART.md`
- Full documentation: `docs/`

### Support
- GitHub Issues: https://github.com/sanae-abe/cldev/issues
- Discussions: https://github.com/sanae-abe/cldev/discussions

## Next Steps

1. **Explore Commands**: Try all available commands
2. **Configure**: Customize configuration for your workflow
3. **Practice**: Use dev commands for real issues
4. **Share**: Share learning sessions with team
5. **Contribute**: Submit issues and PRs on GitHub

## Command Cheat Sheet

| Category | Command | Purpose |
|----------|---------|---------|
| **Emergency** | `cldev dev urgent <issue>` | Production incident (5min response) |
| **Bug Fix** | `cldev dev fix <bug>` | Critical bug resolution (same day) |
| **Debug** | `cldev dev debug <symptom>` | Systematic investigation |
| **Commit** | `cldev git commit` | Conventional commit |
| **Branch** | `cldev git branch` | Conventional branch |
| **PR/MR** | `cldev git merge-request` | Create pull/merge request |
| **Status** | `cldev git status` | Enhanced git status |
| **Lint** | `cldev quality lint --fix` | Auto-fix linting issues |
| **Format** | `cldev quality format` | Format all code |
| **Test** | `cldev quality test --coverage` | Run tests with coverage |
| **Config** | `cldev config check` | Validate configuration |
| **List** | `cldev config list` | List all commands |

---

**Happy Coding! 🚀**
