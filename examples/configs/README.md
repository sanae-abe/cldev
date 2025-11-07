# Configuration Examples

This directory contains example configuration files for the hierarchical configuration system.

## Configuration Hierarchy

cldev uses a 3-layer configuration system with the following priority:

**Project > Stack > Global**

1. **Global** (`~/.config/cldev/config.toml`) - Base configuration for all projects
2. **Stack** (`~/.config/cldev/stacks/{stack_name}.toml`) - Tech stack-specific settings
3. **Project** (`./.cldev/config.toml`) - Project-specific overrides

## Directory Structure

```
examples/configs/
├── global/
│   └── config.toml              # Global configuration example
├── stacks/
│   ├── frontend-web.toml        # Frontend web development
│   ├── backend-api.toml         # Backend API development
│   ├── mobile-app.toml          # Mobile app development
│   ├── data-science.toml        # Data science and ML
│   └── rust-cli.toml            # Rust CLI development
├── project/
│   └── config.toml              # Project configuration example
└── README.md                     # This file
```

## Supported Tech Stacks

### 1. Frontend Web (`frontend-web`)
- **Languages**: TypeScript, JavaScript, HTML, CSS
- **Frameworks**: React, Vue, Angular, Svelte
- **Tools**: npm, Vite, ESLint, Prettier, Vitest
- **Default Port**: 3000
- **Coverage Threshold**: 80%

### 2. Backend API (`backend-api`)
- **Languages**: TypeScript, Python, Go, Rust
- **Frameworks**: Express, FastAPI, Gin, Axum
- **Tools**: npm/pip/cargo, Jest/pytest, ESLint/pylint/clippy
- **Default Port**: 8080
- **Coverage Threshold**: 90%

### 3. Mobile App (`mobile-app`)
- **Languages**: TypeScript, Dart, Swift, Kotlin
- **Frameworks**: React Native, Flutter, Expo
- **Tools**: npm, Metro, ESLint, Jest
- **Default Port**: 8081
- **Coverage Threshold**: 70%

### 4. Data Science (`data-science`)
- **Languages**: Python, R
- **Frameworks**: Jupyter, pandas, scikit-learn, PyTorch
- **Tools**: pip, Black, pylint, pytest
- **Default Port**: 8888
- **Coverage Threshold**: 75%

### 5. Rust CLI (`rust-cli`)
- **Languages**: Rust
- **Frameworks**: clap, tokio, serde
- **Tools**: Cargo, rustfmt, clippy
- **Coverage Threshold**: 80%

## Installation

### 1. Global Configuration

Copy the global config to your home directory:

```bash
mkdir -p ~/.config/cldev
cp examples/configs/global/config.toml ~/.config/cldev/config.toml
```

Edit and customize:
```bash
$EDITOR ~/.config/cldev/config.toml
```

### 2. Stack Configuration

Copy the desired stack configs:

```bash
mkdir -p ~/.config/cldev/stacks
cp examples/configs/stacks/*.toml ~/.config/cldev/stacks/
```

Or copy individual stacks:
```bash
cp examples/configs/stacks/rust-cli.toml ~/.config/cldev/stacks/
```

### 3. Project Configuration

For each project, initialize with:

```bash
cd /path/to/your/project
mkdir -p .cldev
cp /path/to/cldev/examples/configs/project/config.toml .cldev/config.toml
```

Edit project-specific settings:
```bash
$EDITOR .cldev/config.toml
```

## Usage Examples

### Setting Tech Stack in Global Config

```toml
# ~/.config/cldev/config.toml
[general]
tech_stack = "rust-cli"  # Use rust-cli stack by default
```

### Overriding Commands in Project Config

```toml
# ./.cldev/config.toml
[commands]
test = "cargo test --all-features"  # Override default test command
build = "cargo build --release"      # Custom build command
```

### Merging Behavior

When you run a command, settings are merged with this priority:

1. **Project commands** override **Stack commands** override **defaults**
2. **Project dev.port** overrides **Stack dev.port**
3. **Project quality.min_coverage** overrides **Stack coverage_threshold**

Example:
```
Global: dev.port = (not set)
Stack:  dev.port = 3000
Project: dev.port = 4000
Result: 4000 is used
```

## Configuration Loading

The configuration is loaded automatically by cldev:

```rust
use cldev::core::{Config, HierarchicalConfig};

// Load hierarchical config with auto-detection
let config = Config::load_hierarchical(Some("./".into()))?;

// Access merged settings
let port = config.effective_dev_port();
let stack = config.effective_tech_stack();
let commands = config.merged_commands();
```

## Best Practices

1. **Global Config**: Set your personal preferences and common settings
2. **Stack Config**: Define stack-specific tools and commands
3. **Project Config**: Override only what's specific to the project

### Security

- Never store secrets in config files
- Use environment variables for sensitive data
- Config files should have restrictive permissions (600)

### Version Control

```gitignore
# Commit example configs
examples/configs/

# Don't commit actual project config (team-specific)
.cldev/config.toml

# But do commit a template
.cldev/config.toml.example
```

## Testing Configuration

Test your configuration setup:

```bash
# Check effective configuration
cldev config check

# List all configuration sources
cldev config list

# Show merged commands
cldev config show commands
```

## Migration Guide

### From Simple Config to Hierarchical

1. **Identify your tech stack**: Choose from the 5 supported stacks
2. **Move stack-specific settings**: Extract to stack config
3. **Keep project overrides**: In project config
4. **Test merged config**: Use `cldev config check`

## Troubleshooting

### Config not loading?

Check file locations:
```bash
ls -la ~/.config/cldev/config.toml
ls -la ~/.config/cldev/stacks/
ls -la .cldev/config.toml
```

### Wrong values being used?

Check merge priority:
```bash
cldev config show --verbose
```

### Permission errors?

Fix permissions:
```bash
chmod 600 ~/.config/cldev/config.toml
chmod 600 ~/.config/cldev/stacks/*.toml
```

## Further Reading

- [Configuration Documentation](../../docs/configuration.md)
- [Tech Stack Guide](../../docs/tech-stacks.md)
- [CLAUDE.md Layer 3](~/.claude/layers/layer-3-config-reference.md)
