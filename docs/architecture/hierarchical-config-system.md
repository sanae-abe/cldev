# Hierarchical Configuration System

## Overview

The cldev project now implements a **3-layer hierarchical configuration system** that follows the CLAUDE.md Layer 3 specification. This system provides flexible, tech-stack-specific configuration with proper priority handling.

## Architecture

### Configuration Layers (Priority: Project > Stack > Global)

```
┌─────────────────────────────────────────────────────────┐
│                    Project Config                        │
│              (.cldev/config.toml)                       │
│         Project-specific overrides                       │
│         Highest Priority                                 │
└───────────────────┬─────────────────────────────────────┘
                    │ overrides
                    ▼
┌─────────────────────────────────────────────────────────┐
│                    Stack Config                          │
│       (~/.config/cldev/stacks/{stack}.toml)             │
│         Tech stack-specific settings                     │
│         Medium Priority                                  │
└───────────────────┬─────────────────────────────────────┘
                    │ overrides
                    ▼
┌─────────────────────────────────────────────────────────┐
│                    Global Config                         │
│           (~/.config/cldev/config.toml)                 │
│         User-wide base settings                          │
│         Lowest Priority                                  │
└─────────────────────────────────────────────────────────┘
```

## Implemented Components

### 1. Core Modules

#### `src/core/stack_config.rs`
- **TechStack enum**: 5 supported stacks (frontend-web, backend-api, mobile-app, data-science, rust-cli)
- **StackConfig struct**: Stack-specific configuration
  - Commands (build, test, lint, etc.)
  - Tools (package manager, formatter, linter, etc.)
  - Quality settings (coverage threshold, complexity limits)
  - Testing configuration
  - Environment requirements
- **Default configurations**: Sensible defaults for each stack
- **TOML serialization/deserialization**

#### `src/core/project_config.rs`
- **ProjectConfig struct**: Project-specific configuration
  - Project metadata (name, description, version)
  - Custom commands
  - Development settings (port, hot reload, etc.)
  - Quality settings (coverage, pre-commit hooks)
  - Git workflow (base branch, commit template)
  - Custom paths
- **Auto-initialization**: `ProjectConfig::init()` with stack-specific defaults
- **`.cldev/` directory management**

#### `src/core/config.rs` (Extended)
- **GeneralConfig**: Added `tech_stack` and `project_name` fields
- **HierarchicalConfig struct**: Holds all 3 layers
- **Hierarchical loading**: `Config::load_hierarchical()`
- **Merge methods**:
  - `effective_tech_stack()`: Get active tech stack
  - `effective_project_name()`: Get active project name
  - `effective_dev_port()`: Merge port settings
  - `effective_base_branch()`: Merge Git settings
  - `merged_commands()`: Combine commands from all layers
  - `effective_coverage_threshold()`: Merge quality settings

### 2. Supported Tech Stacks

| Stack | Languages | Frameworks | Tools | Default Port |
|-------|-----------|------------|-------|--------------|
| **frontend-web** | TypeScript, JavaScript | React, Vue, Angular | npm, Vite, ESLint, Vitest | 3000 |
| **backend-api** | TypeScript, Python, Go, Rust | Express, FastAPI, Gin, Axum | npm/pip/cargo, Jest/pytest | 8080 |
| **mobile-app** | TypeScript, Dart | React Native, Flutter | npm, Metro, ESLint | 8081 |
| **data-science** | Python, R | Jupyter, pandas, PyTorch | pip, Black, pytest | 8888 |
| **rust-cli** | Rust | clap, tokio, serde | Cargo, rustfmt, clippy | N/A |

### 3. Example Configurations

Created comprehensive examples in `examples/configs/`:
- **Global**: `global/config.toml` - Base user configuration
- **Stacks**: 5 stack-specific configs in `stacks/`
- **Project**: `project/config.toml` - Project override example
- **README**: Detailed usage guide with installation instructions

### 4. Integration Tests

Comprehensive test suite in `tests/hierarchical_config_test.rs`:
- ✅ Global config loading
- ✅ Stack config loading
- ✅ Project config loading
- ✅ Hierarchical priority handling
- ✅ Command merging
- ✅ Effective value resolution
- ✅ Tech stack detection
- ✅ Project initialization
- ✅ Fallback to defaults

**Test Results**: 14/14 tests passing

## Usage

### Loading Hierarchical Configuration

```rust
use cldev::core::{Config, HierarchicalConfig, TechStack};
use std::path::PathBuf;

// Load all 3 layers with auto-detection
let config = Config::load_hierarchical(Some(PathBuf::from("./")))?;

// Access merged settings
let tech_stack = config.effective_tech_stack();
let project_name = config.effective_project_name();
let dev_port = config.effective_dev_port();
let base_branch = config.effective_base_branch();
let commands = config.merged_commands();
let coverage = config.effective_coverage_threshold();
```

### Setting Tech Stack

```rust
// In global config
let mut global_config = Config::load(None)?;
global_config.set_tech_stack(Some(TechStack::RustCli));
global_config.save(None)?;
```

### Initializing Project Config

```rust
use cldev::core::{ProjectConfig, TechStack};

// Initialize with tech stack
let config = ProjectConfig::init(
    project_root,
    "my-app".to_string(),
    Some(TechStack::FrontendWeb),
)?;

// Auto-applies stack defaults:
// - dev.port = 3000
// - quality.min_coverage = 80
// - paths.dist = "dist"
```

## Configuration Priority Examples

### Example 1: Development Port

```toml
# Global: (not set)
# Stack (frontend-web.toml): dev_port = 3000
# Project (.cldev/config.toml): port = 4000

Result: 4000 (Project overrides Stack)
```

### Example 2: Commands

```toml
# Stack (rust-cli.toml):
[commands]
build = "cargo build"
test = "cargo test"

# Project (.cldev/config.toml):
[commands]
build = "cargo build --release"
custom = "cargo run --example demo"

Merged Result:
build = "cargo build --release"  # Project override
test = "cargo test"              # From stack
custom = "cargo run --example demo"  # Project-only
```

### Example 3: Coverage Threshold

```toml
# Global: (not set)
# Stack (backend-api.toml): coverage_threshold = 90
# Project: min_coverage = 85

Result: 85 (Project overrides Stack)
```

## File Locations

### Global Configuration
```
~/.config/cldev/config.toml
```

### Stack Configurations
```
~/.config/cldev/stacks/
├── frontend-web.toml
├── backend-api.toml
├── mobile-app.toml
├── data-science.toml
└── rust-cli.toml
```

### Project Configuration
```
project_root/
└── .cldev/
    └── config.toml
```

## Security

- Global config requires **600 permissions** (owner read/write only)
- Stack configs are user-specific (stored in user config directory)
- Project configs can be committed to version control
- **Never store secrets** in config files - use environment variables

## Benefits

1. **Separation of Concerns**
   - Global: Personal preferences
   - Stack: Technology-specific defaults
   - Project: Project-specific overrides

2. **Tech Stack Awareness**
   - Automatic tool detection
   - Sensible defaults per stack
   - Stack-specific commands

3. **Flexibility**
   - Override at any level
   - Mix and match settings
   - Easy to understand priority

4. **DRY Principle**
   - Define once in stack config
   - Reuse across projects
   - Override only when needed

5. **Type Safety**
   - Compile-time validation
   - Strong typing with Rust
   - Clear error messages

## Migration Guide

### From Simple Config

1. **Identify your tech stack**:
   ```bash
   # Choose from: frontend-web, backend-api, mobile-app, data-science, rust-cli
   ```

2. **Set tech stack in global config**:
   ```toml
   [general]
   tech_stack = "rust-cli"
   ```

3. **Create stack config** (optional):
   ```bash
   cp examples/configs/stacks/rust-cli.toml ~/.config/cldev/stacks/
   ```

4. **Initialize project config**:
   ```bash
   cd your-project
   cldev config init --stack rust-cli
   ```

## API Reference

### TechStack

```rust
pub enum TechStack {
    FrontendWeb,
    BackendApi,
    MobileApp,
    DataScience,
    RustCli,
}

impl TechStack {
    pub fn as_str(&self) -> &'static str;
    pub fn from_str(s: &str) -> Result<Self>;
    pub fn all() -> Vec<Self>;
}
```

### StackConfig

```rust
pub struct StackConfig {
    pub stack: StackMetadata,
    pub commands: HashMap<String, String>,
    pub tools: ToolsConfig,
    pub quality: StackQualityConfig,
    pub testing: TestingConfig,
    pub environment: EnvironmentConfig,
}

impl StackConfig {
    pub fn load(stack: &TechStack) -> Result<Self>;
    pub fn save(&self, stack: &TechStack) -> Result<()>;
    pub fn default_for_stack(stack: &TechStack) -> Self;
}
```

### ProjectConfig

```rust
pub struct ProjectConfig {
    pub project: ProjectMetadata,
    pub commands: HashMap<String, String>,
    pub dev: ProjectDevConfig,
    pub quality: ProjectQualityConfig,
    pub git: ProjectGitConfig,
    pub paths: ProjectPaths,
}

impl ProjectConfig {
    pub fn load(project_root: &Path) -> Result<Self>;
    pub fn save(&self, project_root: &Path) -> Result<()>;
    pub fn init(
        project_root: &Path,
        name: String,
        tech_stack: Option<TechStack>
    ) -> Result<Self>;
}
```

### HierarchicalConfig

```rust
pub struct HierarchicalConfig {
    pub global: Arc<Config>,
    pub stack: Option<StackConfig>,
    pub project: Option<ProjectConfig>,
}

impl HierarchicalConfig {
    pub fn effective_tech_stack(&self) -> Option<TechStack>;
    pub fn effective_project_name(&self) -> Option<String>;
    pub fn effective_dev_port(&self) -> Option<u16>;
    pub fn effective_base_branch(&self) -> String;
    pub fn merged_commands(&self) -> HashMap<String, String>;
    pub fn effective_coverage_threshold(&self) -> Option<u8>;
}
```

### Config (Extended)

```rust
impl Config {
    pub fn load_hierarchical(
        project_root: Option<PathBuf>
    ) -> Result<HierarchicalConfig>;

    pub fn get_tech_stack(&self) -> Option<TechStack>;
    pub fn set_tech_stack(&mut self, stack: Option<TechStack>);
    pub fn get_project_name(&self) -> Option<&str>;
    pub fn set_project_name(&mut self, name: Option<String>);
}
```

## Future Enhancements

1. **Config Validation**: Validate settings against stack requirements
2. **Config Migration**: Auto-migrate between config versions
3. **Config Templates**: Pre-built templates for common project types
4. **Config Inheritance**: Allow stacks to inherit from base stacks
5. **Config Profiles**: Environment-specific configs (dev, staging, prod)

## Related Documentation

- [Configuration Examples](../examples/configs/README.md)
- [CLAUDE.md Layer 3](~/.claude/layers/layer-3-config-reference.md)
- [Tech Stack Guide](./tech-stacks.md) (to be created)

## Conclusion

The hierarchical configuration system provides a flexible, maintainable, and type-safe way to manage settings across different scopes. It follows best practices from modern development tools while maintaining simplicity and clarity.
