# Configuration Usage Examples

## Basic Usage

### Load Configuration

```rust
use cldev::core::Config;
use std::sync::Arc;

// Load from default location (~/.config/cldev/config.toml)
let config: Arc<Config> = Config::load(None)?;

// Access configuration values
println!("Language: {}", config.general.language);
println!("Base branch: {}", config.git.default_base_branch);
println!("Parallel tasks: {}", config.performance.parallel_tasks);
```

### Save Configuration

```rust
use cldev::core::Config;

// Create new configuration
let mut config = Config::default();

// Modify settings
config.general.language = "en".to_string();
config.git.default_base_branch = "develop".to_string();
config.performance.parallel_tasks = 8;

// Save to default location
config.save(None)?;

// Or save to custom location
config.save(Some(PathBuf::from("/tmp/custom_config.toml")))?;
```

### Share Configuration Across Modules

```rust
use std::sync::Arc;
use cldev::core::Config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load once
    let config = Config::load(None)?;
    
    // Share efficiently with Arc
    process_git_command(Arc::clone(&config))?;
    process_quality_command(Arc::clone(&config))?;
    
    Ok(())
}

fn process_git_command(config: Arc<Config>) -> Result<(), Box<dyn std::error::Error>> {
    if config.git.github_cli {
        println!("Using GitHub CLI");
    }
    Ok(())
}

fn process_quality_command(config: Arc<Config>) -> Result<(), Box<dyn std::error::Error>> {
    if config.quality.run_tests_before_commit {
        println!("Running tests...");
    }
    Ok(())
}
```

## Version Management

### Check Version Compatibility

```rust
use cldev::core::{Config, ConfigVersion};

let config = Config::load(None)?;

// Get current version
let current = ConfigVersion::current(); // "1.0.0"

// Parse version
let (major, minor, patch) = ConfigVersion::parse(&config.version)?;
println!("Config version: {}.{}.{}", major, minor, patch);

// Check compatibility
if ConfigVersion::is_compatible(&config.version) {
    println!("Configuration is compatible");
} else {
    println!("Migration required");
}
```

## Configuration Sections

### General Settings

```rust
// Language preference
if config.general.language == "ja" {
    println!("日本語モード");
}

// Directory paths
let claude_dir = &config.general.claude_dir;
let projects_dir = &config.general.projects_dir;
```

### Git Settings

```rust
// CLI availability
if config.git.github_cli {
    // Use gh command
}

if config.git.gitlab_cli {
    // Use glab command
}

// Branch settings
let base = &config.git.default_base_branch;
if config.git.auto_push {
    // Auto push after commit
}
```

### Quality Settings

```rust
// Linting behavior
if config.quality.auto_fix {
    // Run linter with --fix
}

// Testing before commit
if config.quality.run_tests_before_commit {
    // Run test suite
}
```

### Development Settings

```rust
// Branch creation
if config.dev.auto_create_branch {
    let branch_name = format!("{}/feature-name", config.dev.branch_prefix);
}

// Session recording
if config.dev.session_recording {
    // Record development session
}
```

### Learning Record Settings

```rust
// Session directory
let sessions_dir = &config.lr.sessions_dir;

// Auto-save behavior
if config.lr.auto_save {
    // Save learning records automatically
}

// Default tags
for tag in &config.lr.default_tags {
    println!("Tag: {}", tag);
}
```

### UI Settings

```rust
// Output formatting
if config.ui.color {
    // Use colored output
}

if config.ui.emoji {
    // Include emoji in messages
}

if config.ui.progress_bar {
    // Show progress indicators
}
```

### Performance Settings

```rust
// Parallel execution
let max_parallel = config.performance.parallel_tasks;

// Command timeout
let timeout = Duration::from_secs(config.performance.timeout_seconds);
```

## Error Handling

```rust
use cldev::core::{Config, CldevError};

match Config::load(None) {
    Ok(config) => {
        // Use configuration
    }
    Err(CldevError::Io(e)) => {
        eprintln!("IO error: {}", e);
    }
    Err(CldevError::Config(msg)) => {
        eprintln!("Config error: {}", msg);
    }
    Err(CldevError::Validation(msg)) => {
        eprintln!("Validation error: {}", msg);
    }
    Err(e) => {
        eprintln!("Unexpected error: {}", e);
    }
}
```

## Custom Configuration Path

```rust
use std::path::PathBuf;
use cldev::core::Config;

// Load from custom path
let custom_path = PathBuf::from("/etc/cldev/config.toml");
let config = Config::load(Some(custom_path))?;

// Save to custom path
let save_path = PathBuf::from("/tmp/backup_config.toml");
config.save(Some(save_path))?;
```

## Testing Configuration

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_custom_config() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");

        // Create custom config
        let mut config = Config::default();
        config.general.language = "en".to_string();
        config.save(Some(config_path.clone())).unwrap();

        // Load and verify
        let loaded = Config::load(Some(config_path)).unwrap();
        assert_eq!(loaded.general.language, "en");
    }
}
```

## Example TOML Configuration

```toml
# ~/.config/cldev/config.toml
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
