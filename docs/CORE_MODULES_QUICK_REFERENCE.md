# Core Modules Quick Reference

Quick reference guide for cldev core modules. For detailed documentation, see `CORE_MODULES_IMPLEMENTATION.md`.

---

## 🚀 Quick Start

### Import Core Types
```rust
use cldev::{
    Config,
    CldevError, Result,
    I18n, Language,
    SecurePath,
    GitUtils, RemoteType,
    ProjectDetector, ProjectType,
    LearningSession, LearningSessionBuilder,
};
```

---

## 📦 Module Overview

### 1. Configuration (`core::config`)

**Load Configuration:**
```rust
use cldev::Config;

// Load from default location (~/.config/cldev/config.toml)
let config = Config::load(None)?;

// Load from custom path
let config = Config::load(Some(PathBuf::from("/path/to/config.toml")))?;

// Create default configuration
let config = Config::default();
```

**Save Configuration:**
```rust
let config = Config::default();
config.save(None)?; // Saves to default location
```

**Access Configuration Values:**
```rust
let language = &config.general.language;
let auto_push = config.git.auto_push;
let parallel_tasks = config.performance.parallel_tasks;
```

---

### 2. Security (`core::security`)

**Path Validation:**
```rust
use cldev::SecurePath;

// Create secure path validator
let secure = SecurePath::new(PathBuf::from("/home/user/.claude"))?;

// Validate existing file
let safe_path = secure.validate(Path::new("config.toml"))?;

// Validate path that doesn't exist yet
let future_path = secure.validate_non_existent(Path::new("new-file.txt"))?;
```

**Safe Command Execution:**
```rust
use cldev::security::safe_command;

// Execute allowed command
let mut cmd = safe_command("git", &["status"])?;
let output = cmd.output()?;

// This will fail - command not allowed
let result = safe_command("rm", &["-rf", "/"]);
assert!(result.is_err());
```

**File Permissions:**
```rust
use cldev::security::{check_file_permissions, set_secure_permissions};

// Check if file has secure permissions (0o600)
check_file_permissions(Path::new("/path/to/config.toml"))?;

// Set secure permissions
set_secure_permissions(Path::new("/path/to/config.toml"))?;
```

---

### 3. Internationalization (`core::i18n`)

**Basic Usage:**
```rust
use cldev::{I18n, Language};

// Auto-detect language from environment
let i18n = I18n::new();

// Use specific language
let i18n = I18n::with_language(Language::Japanese);

// Get translated message
let msg = i18n.get("config-success");

// Get message with variables
let msg = i18n.format("config-created-at", "path", "/path/to/config");
```

**Multiple Variables:**
```rust
use std::collections::HashMap;

let mut vars = HashMap::new();
vars.insert("command", "cldev config check");
vars.insert("file", "config.toml");
let msg = i18n.get_with_vars("complex-message", &vars);
```

**Supported Languages:**
- `Language::English` - "en"
- `Language::Japanese` - "ja"

---

### 4. Git Utilities (`core::git_utils`)

**Repository Operations:**
```rust
use cldev::GitUtils;

// Open current repository
let git = GitUtils::open_current()?;

// Open repository at specific path
let git = GitUtils::open("/path/to/repo")?;

// Get current branch
let branch = git.current_branch()?;

// Check if working directory is clean
if git.is_clean()? {
    println!("No uncommitted changes");
}

// Get list of changed files
let files = git.changed_files()?;

// Get unpushed commits count
let unpushed = git.unpushed_commits("origin")?;
```

**Remote Detection:**
```rust
use cldev::{GitUtils, RemoteType};

let git = GitUtils::open_current()?;

// Detect remote type
let remote_type = git.detect_remote_type("origin")?;

match remote_type {
    RemoteType::GitHub => println!("GitHub repository"),
    RemoteType::GitLab => println!("GitLab repository"),
    RemoteType::Other => println!("Other Git hosting"),
}
```

**CLI Tool Checking:**
```rust
use cldev::git_utils::{check_gh_cli, check_glab_cli};

if check_gh_cli()? {
    println!("GitHub CLI (gh) is available");
}

if check_glab_cli()? {
    println!("GitLab CLI (glab) is available");
}
```

---

### 5. Project Detection (`core::project_detector`)

**Detect Project Type:**
```rust
use cldev::{ProjectDetector, ProjectType};

// Detect current directory
let detector = ProjectDetector::new(None)?;

// Detect specific directory
let detector = ProjectDetector::new(Some(Path::new("/path/to/project")))?;

match detector.project_type() {
    ProjectType::NodeJs => println!("Node.js project"),
    ProjectType::Rust => println!("Rust project"),
    ProjectType::Go => println!("Go project"),
    ProjectType::Python => println!("Python project"),
    ProjectType::Ruby => println!("Ruby project"),
    ProjectType::Java => println!("Java project"),
    ProjectType::Php => println!("PHP project"),
    ProjectType::DotNet => println!(".NET project"),
    ProjectType::Elixir => println!("Elixir project"),
    ProjectType::Kotlin => println!("Kotlin project"),
    ProjectType::Swift => println!("Swift project"),
    ProjectType::Scala => println!("Scala project"),
    ProjectType::Unknown => println!("Unknown project type"),
}
```

**Generate Commands:**
```rust
// Get lint command
let lint_cmd = detector.get_lint_command(
    true,  // fix
    false  // all files
)?;

// Get format command
let format_cmd = detector.get_format_command(false)?; // check mode

// Get test command
let test_cmd = detector.get_test_command(
    Some("unit"),  // pattern
    true,          // coverage
    false          // watch
)?;
```

**Command Examples by Project Type:**

**Node.js:**
```rust
// Lint: ["npm", "run", "lint:fix"] or ["npx", "eslint", "--fix", "."]
// Format: ["npm", "run", "format"] or ["npx", "prettier", "--write", "."]
// Test: ["npm", "run", "test:coverage"]
```

**Rust:**
```rust
// Lint: ["cargo", "clippy", "--all-targets", "--all-features", "--", "-D", "warnings"]
// Format: ["cargo", "fmt", "--check"]
// Test: ["cargo", "test"] or ["cargo", "tarpaulin"]
```

**Ruby:**
```rust
// Lint: ["bundle", "exec", "rubocop", "-A"]
// Format: ["bundle", "exec", "rubocop", "-A"]
// Test: ["bundle", "exec", "rspec"]
```

**Java:**
```rust
// Lint: ["mvn", "checkstyle:check"] or ["./gradlew", "checkstyleMain"]
// Format: ["google-java-format", "-i"]
// Test: ["mvn", "test"] or ["./gradlew", "test"]
```

**PHP:**
```rust
// Lint: ["vendor/bin/phpcs"] or ["vendor/bin/phpcbf"]
// Format: ["vendor/bin/phpcbf"]
// Test: ["vendor/bin/phpunit", "--coverage-html", "coverage"]
```

**For complete language support, see `docs/SUPPORTED_LANGUAGES.md`**

---

### 6. Session Recording (`core::session_recorder`)

**Create Session:**
```rust
use cldev::LearningSession;

let mut session = LearningSession::new("debug", "Memory leak investigation");

// Add metadata
session.add_tag("performance");
session.add_tag("memory");
session.add_learning("Always check resource cleanup");
session.add_file("src/main.rs");
session.add_step("Ran memory profiler");
session.set_root_cause("Missing drop implementation");
session.set_solution("Implemented Drop trait");
session.mark_resolved(Some(45)); // 45 minutes

// Save session
let path = session.save()?;
```

**Builder Pattern:**
```rust
use cldev::LearningSessionBuilder;

let (session, path) = LearningSessionBuilder::new("urgent", "Production bug")
    .tag("critical")
    .tag("security")
    .learning("Validate all user inputs")
    .file("src/auth.rs")
    .step("Identified SQL injection vulnerability")
    .step("Implemented parameterized queries")
    .root_cause("Missing input sanitization")
    .solution("Added validation middleware")
    .resolved(Some(30))
    .save()?;
```

**Query Sessions:**
```rust
// List all sessions
let session_ids = LearningSession::list_all()?;

// Load specific session
let session = LearningSession::load("urgent_20251107_143000")?;

// Find by tag
let performance_sessions = LearningSession::find_by_tag("performance")?;

// Find by type
let urgent_sessions = LearningSession::find_by_type("urgent")?;
```

---

### 7. Error Handling (`core::error`)

**Using Result Type:**
```rust
use cldev::{CldevError, Result};

fn my_function() -> Result<()> {
    // Use ? operator for error propagation
    let config = Config::load(None)?;

    // Create custom errors
    if some_condition {
        return Err(CldevError::validation("Invalid configuration"));
    }

    Ok(())
}
```

**Error Types:**
```rust
// Configuration errors
CldevError::config("Invalid config format")

// Validation errors
CldevError::validation("Invalid version number")

// Command errors
CldevError::command("Failed to execute git")

// Git errors
CldevError::git("Repository not found")

// I/O errors (auto-converted)
std::fs::read_to_string(path)?

// Security errors
CldevError::security("Path traversal detected")
```

**Error Matching:**
```rust
match my_function() {
    Ok(_) => println!("Success"),
    Err(CldevError::Config(msg)) => eprintln!("Config error: {}", msg),
    Err(CldevError::Validation(msg)) => eprintln!("Validation error: {}", msg),
    Err(e) => eprintln!("Other error: {}", e),
}
```

---

## 🔒 Security Best Practices

### Always Validate Paths
```rust
let secure = SecurePath::new(base_dir)?;
let safe_path = secure.validate(user_input)?;
// Now safe_path is guaranteed to be within base_dir
```

### Use Allowlisted Commands
```rust
// Good - uses allowlist
let cmd = safe_command("git", &["status"])?;

// Bad - arbitrary command execution
let output = Command::new(user_input).output()?; // DON'T DO THIS
```

### Check File Permissions
```rust
// Ensure config file is not world-readable
check_file_permissions(&config_path)?;
```

### Never Store Secrets in Config
```rust
// Good - config only stores non-sensitive settings
pub struct Config {
    pub language: String,
    pub auto_push: bool,
}

// Bad - secrets in config
pub struct Config {
    pub api_key: String,  // DON'T DO THIS
    pub password: String, // DON'T DO THIS
}
```

---

## 🧪 Testing

### Example Test Structure
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_feature() {
        // Arrange
        let temp_dir = TempDir::new().unwrap();
        let test_path = temp_dir.path().join("test.txt");

        // Act
        let result = my_function(&test_path);

        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_error_case() {
        let result = my_function_that_should_fail();
        assert!(result.is_err());

        match result {
            Err(CldevError::Validation(_)) => {},
            _ => panic!("Expected validation error"),
        }
    }
}
```

---

## 📊 Common Patterns

### Arc<Config> Sharing
```rust
use std::sync::Arc;

// Load config once
let config = Config::load(None)?;

// Share across threads/modules
let config_clone = Arc::clone(&config);

// Access without cloning data
println!("Language: {}", config.general.language);
```

### Builder Pattern
```rust
// Fluent API for complex object construction
let session = LearningSessionBuilder::new("debug", "Issue description")
    .tag("performance")
    .learning("Key insight")
    .resolved(Some(30))
    .build();
```

### Error Propagation
```rust
fn outer_function() -> Result<()> {
    // ? operator propagates errors up the call stack
    inner_function()?;
    another_function()?;
    Ok(())
}
```

---

## 🎯 Performance Tips

### 1. Reuse I18n Instance
```rust
// Create once
let i18n = I18n::new();

// Reuse for multiple translations
let msg1 = i18n.get("message1");
let msg2 = i18n.get("message2");
```

### 2. Cache SecurePath Validator
```rust
// Create once for a base directory
let secure = SecurePath::new(base_dir)?;

// Reuse for multiple validations
let path1 = secure.validate(file1)?;
let path2 = secure.validate(file2)?;
```

### 3. Use Arc for Config
```rust
// Arc provides cheap cloning
let config = Config::load(None)?;
let clone = Arc::clone(&config); // Just increments reference count
```

---

## 🔍 Debugging

### Enable Verbose Output
```rust
// In your CLI handler
if verbose {
    println!("Config loaded from: {:?}", config_path);
    println!("Project type detected: {:?}", detector.project_type());
}
```

### Error Context
```rust
// Add context to errors
config.save(None)
    .map_err(|e| {
        eprintln!("Failed to save config: {}", e);
        e
    })?;
```

---

## 📚 Resources

- **Full Documentation**: `CORE_MODULES_IMPLEMENTATION.md`
- **API Docs**: Run `cargo doc --lib --open`
- **Examples**: See `examples/` directory
- **Tests**: See `#[cfg(test)]` modules in each file

---

**Quick Reference Version**: 1.0.0
**Last Updated**: 2025-11-07
