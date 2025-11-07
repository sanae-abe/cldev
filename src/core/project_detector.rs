//! Project type detection and command generation
//!
//! This module provides automatic project type detection based on configuration files
//! and generates appropriate commands for linting, formatting, and testing.

use crate::core::error::{CldevError, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// Supported project types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectType {
    /// Node.js/JavaScript/TypeScript project (package.json)
    NodeJs,
    /// Rust project (Cargo.toml)
    Rust,
    /// Go project (go.mod)
    Go,
    /// Python project (pyproject.toml, requirements.txt, setup.py)
    Python,
    /// Ruby project (Gemfile)
    Ruby,
    /// Java project (pom.xml, build.gradle)
    Java,
    /// PHP project (composer.json)
    Php,
    /// .NET project (*.csproj, *.sln)
    DotNet,
    /// Elixir project (mix.exs)
    Elixir,
    /// Kotlin project (build.gradle.kts)
    Kotlin,
    /// Swift project (Package.swift)
    Swift,
    /// Scala project (build.sbt)
    Scala,
    /// Unknown or unsupported project type
    Unknown,
}

impl ProjectType {
    /// Get human-readable name for the project type
    pub fn name(&self) -> &'static str {
        match self {
            ProjectType::NodeJs => "Node.js",
            ProjectType::Rust => "Rust",
            ProjectType::Go => "Go",
            ProjectType::Python => "Python",
            ProjectType::Ruby => "Ruby",
            ProjectType::Java => "Java",
            ProjectType::Php => "PHP",
            ProjectType::DotNet => ".NET",
            ProjectType::Elixir => "Elixir",
            ProjectType::Kotlin => "Kotlin",
            ProjectType::Swift => "Swift",
            ProjectType::Scala => "Scala",
            ProjectType::Unknown => "Unknown",
        }
    }

    /// Get file extensions typically used by this project type
    pub fn extensions(&self) -> &[&str] {
        match self {
            ProjectType::NodeJs => &["js", "jsx", "ts", "tsx", "mjs", "cjs"],
            ProjectType::Rust => &["rs"],
            ProjectType::Go => &["go"],
            ProjectType::Python => &["py", "pyi"],
            ProjectType::Ruby => &["rb", "rake"],
            ProjectType::Java => &["java"],
            ProjectType::Php => &["php"],
            ProjectType::DotNet => &["cs", "fs", "vb"],
            ProjectType::Elixir => &["ex", "exs"],
            ProjectType::Kotlin => &["kt", "kts"],
            ProjectType::Swift => &["swift"],
            ProjectType::Scala => &["scala"],
            ProjectType::Unknown => &[],
        }
    }
}

/// Project detector for automatic project type recognition
#[derive(Debug)]
pub struct ProjectDetector {
    /// Root directory of the project
    root: PathBuf,
    /// Detected project type
    project_type: ProjectType,
}

impl ProjectDetector {
    /// Create a new project detector for the given directory
    ///
    /// # Arguments
    /// * `path` - Directory to analyze (defaults to current directory if None)
    ///
    /// # Returns
    /// Result containing ProjectDetector or error
    pub fn new(path: Option<&Path>) -> Result<Self> {
        let root = if let Some(p) = path {
            p.to_path_buf()
        } else {
            std::env::current_dir().map_err(|e| {
                CldevError::Config(format!("Failed to get current directory: {}", e))
            })?
        };

        if !root.exists() {
            return Err(CldevError::Config(format!(
                "Directory does not exist: {}",
                root.display()
            )));
        }

        if !root.is_dir() {
            return Err(CldevError::Config(format!(
                "Path is not a directory: {}",
                root.display()
            )));
        }

        let project_type = Self::detect_project_type(&root)?;

        Ok(Self { root, project_type })
    }

    /// Detect project type based on configuration files
    fn detect_project_type(root: &Path) -> Result<ProjectType> {
        // Priority order for monorepos (compiled languages first, then scripting):
        // 1. Compiled/Static: Rust, Go, Java, Kotlin, Swift, Scala, .NET
        // 2. Dynamic/Scripting: Ruby, PHP, Elixir, Node.js, Python
        // This ensures monorepos with multiple project types are handled correctly

        // Rust (highest priority - systems language)
        if root.join("Cargo.toml").exists() {
            return Ok(ProjectType::Rust);
        }

        // Go (high priority - compiled language)
        if root.join("go.mod").exists() {
            return Ok(ProjectType::Go);
        }

        // Java (Maven or Gradle)
        if root.join("pom.xml").exists() || root.join("build.gradle").exists() {
            // Distinguish between Java and Kotlin
            if root.join("build.gradle.kts").exists() {
                return Ok(ProjectType::Kotlin);
            }
            // Check for Kotlin files if using build.gradle
            if Self::has_files_with_extension(root, &["kt", "kts"]) {
                return Ok(ProjectType::Kotlin);
            }
            return Ok(ProjectType::Java);
        }

        // Kotlin (build.gradle.kts is Kotlin-specific)
        if root.join("build.gradle.kts").exists() {
            return Ok(ProjectType::Kotlin);
        }

        // Swift Package Manager
        if root.join("Package.swift").exists() {
            return Ok(ProjectType::Swift);
        }

        // Scala (SBT build tool)
        if root.join("build.sbt").exists() {
            return Ok(ProjectType::Scala);
        }

        // .NET (C#, F#, VB.NET)
        if Self::has_files_with_extension(root, &["csproj", "fsproj", "vbproj", "sln"]) {
            return Ok(ProjectType::DotNet);
        }

        // Ruby (Gemfile)
        if root.join("Gemfile").exists() {
            return Ok(ProjectType::Ruby);
        }

        // PHP (Composer)
        if root.join("composer.json").exists() {
            return Ok(ProjectType::Php);
        }

        // Elixir (Mix)
        if root.join("mix.exs").exists() {
            return Ok(ProjectType::Elixir);
        }

        // Node.js/JavaScript/TypeScript
        if root.join("package.json").exists() {
            return Ok(ProjectType::NodeJs);
        }

        // Python (multiple indicators)
        if root.join("pyproject.toml").exists()
            || root.join("requirements.txt").exists()
            || root.join("setup.py").exists()
            || root.join("Pipfile").exists()
        {
            return Ok(ProjectType::Python);
        }

        Ok(ProjectType::Unknown)
    }

    /// Check if directory has files with specific extensions
    fn has_files_with_extension(root: &Path, extensions: &[&str]) -> bool {
        if let Ok(entries) = fs::read_dir(root) {
            for entry in entries.flatten() {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        if let Some(ext) = entry.path().extension() {
                            if let Some(ext_str) = ext.to_str() {
                                if extensions.contains(&ext_str) {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }
        false
    }

    /// Get the detected project type
    pub fn project_type(&self) -> ProjectType {
        self.project_type
    }

    /// Get the project root directory
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Get lint command for the detected project type
    ///
    /// # Arguments
    /// * `fix` - Whether to auto-fix issues
    /// * `all` - Whether to lint all files (not just changed)
    ///
    /// # Returns
    /// Command and arguments as a vector
    pub fn get_lint_command(&self, fix: bool, all: bool) -> Result<Vec<String>> {
        match self.project_type {
            ProjectType::NodeJs => {
                let mut cmd = vec!["npm".to_string(), "run".to_string()];

                // Try to detect which linter is configured
                if self.has_script("lint:fix") && fix {
                    cmd.push("lint:fix".to_string());
                } else if self.has_script("lint") {
                    cmd.push("lint".to_string());
                } else if self.has_dependency("eslint") {
                    cmd = vec!["npx".to_string(), "eslint".to_string()];
                    if fix {
                        cmd.push("--fix".to_string());
                    }
                    cmd.push(".".to_string());
                } else {
                    return Err(CldevError::Config(
                        "No linter found. Please install ESLint or configure a lint script."
                            .to_string(),
                    ));
                }

                // Add TypeScript type checking if available
                if self.has_script("type-check") && all {
                    // Type checking is usually separate
                }

                Ok(cmd)
            }
            ProjectType::Rust => {
                let mut cmd = vec!["cargo".to_string(), "clippy".to_string()];
                if all {
                    cmd.push("--all-targets".to_string());
                    cmd.push("--all-features".to_string());
                }
                cmd.push("--".to_string());
                cmd.push("-D".to_string());
                cmd.push("warnings".to_string());
                Ok(cmd)
            }
            ProjectType::Go => {
                // Go vet is the standard linter
                let cmd = if all {
                    vec!["go".to_string(), "vet".to_string(), "./...".to_string()]
                } else {
                    vec!["go".to_string(), "vet".to_string()]
                };
                Ok(cmd)
            }
            ProjectType::Python => {
                // Try to detect which linter is installed
                if self.has_command("ruff") {
                    let mut cmd = vec!["ruff".to_string(), "check".to_string()];
                    if fix {
                        cmd.push("--fix".to_string());
                    }
                    cmd.push(".".to_string());
                    Ok(cmd)
                } else if self.has_command("pylint") {
                    Ok(vec!["pylint".to_string(), ".".to_string()])
                } else if self.has_command("flake8") {
                    Ok(vec!["flake8".to_string(), ".".to_string()])
                } else {
                    Err(CldevError::Config(
                        "No Python linter found. Please install ruff, pylint, or flake8."
                            .to_string(),
                    ))
                }
            }
            ProjectType::Ruby => {
                let mut cmd = vec![
                    "bundle".to_string(),
                    "exec".to_string(),
                    "rubocop".to_string(),
                ];
                if fix {
                    cmd.push("-A".to_string()); // Auto-correct all offenses
                }
                Ok(cmd)
            }
            ProjectType::Java => {
                // Maven: checkstyle plugin
                if self.root.join("pom.xml").exists() {
                    Ok(vec!["mvn".to_string(), "checkstyle:check".to_string()])
                } else {
                    // Gradle: checkstyle plugin
                    Ok(vec!["./gradlew".to_string(), "checkstyleMain".to_string()])
                }
            }
            ProjectType::Php => {
                let mut cmd = vec!["vendor/bin/phpcs".to_string()];
                if fix {
                    cmd = vec!["vendor/bin/phpcbf".to_string()]; // PHP Code Beautifier and Fixer
                }
                Ok(cmd)
            }
            ProjectType::DotNet => {
                let mut cmd = vec!["dotnet".to_string(), "format".to_string()];
                if !fix {
                    cmd.push("--verify-no-changes".to_string());
                }
                Ok(cmd)
            }
            ProjectType::Elixir => {
                let mut cmd = vec!["mix".to_string(), "credo".to_string()];
                if fix {
                    cmd.push("--strict".to_string());
                }
                Ok(cmd)
            }
            ProjectType::Kotlin => {
                // ktlint is the most popular Kotlin linter
                let mut cmd = vec!["./gradlew".to_string(), "ktlintCheck".to_string()];
                if fix {
                    cmd = vec!["./gradlew".to_string(), "ktlintFormat".to_string()];
                }
                Ok(cmd)
            }
            ProjectType::Swift => {
                // SwiftLint
                let mut cmd = vec!["swiftlint".to_string(), "lint".to_string()];
                if fix {
                    cmd.push("--fix".to_string());
                }
                Ok(cmd)
            }
            ProjectType::Scala => {
                // Scalastyle or scalafix
                Ok(vec!["sbt".to_string(), "scalastyle".to_string()])
            }
            ProjectType::Unknown => Err(CldevError::Config(
                "Unknown project type. Cannot determine lint command.".to_string(),
            )),
        }
    }

    /// Get format command for the detected project type
    ///
    /// # Arguments
    /// * `check` - Whether to check formatting without modifying files
    ///
    /// # Returns
    /// Command and arguments as a vector
    pub fn get_format_command(&self, check: bool) -> Result<Vec<String>> {
        match self.project_type {
            ProjectType::NodeJs => {
                if self.has_script("format:check") && check {
                    Ok(vec![
                        "npm".to_string(),
                        "run".to_string(),
                        "format:check".to_string(),
                    ])
                } else if self.has_script("format") {
                    Ok(vec![
                        "npm".to_string(),
                        "run".to_string(),
                        "format".to_string(),
                    ])
                } else if self.has_dependency("prettier") {
                    let mut cmd = vec!["npx".to_string(), "prettier".to_string()];
                    if check {
                        cmd.push("--check".to_string());
                    } else {
                        cmd.push("--write".to_string());
                    }
                    cmd.push(".".to_string());
                    Ok(cmd)
                } else {
                    Err(CldevError::Config(
                        "No formatter found. Please install Prettier or configure a format script."
                            .to_string(),
                    ))
                }
            }
            ProjectType::Rust => {
                let mut cmd = vec!["cargo".to_string(), "fmt".to_string()];
                if check {
                    cmd.push("--".to_string());
                    cmd.push("--check".to_string());
                }
                Ok(cmd)
            }
            ProjectType::Go => {
                let cmd = if check {
                    vec!["gofmt".to_string(), "-l".to_string(), ".".to_string()]
                } else {
                    vec!["go".to_string(), "fmt".to_string(), "./...".to_string()]
                };
                Ok(cmd)
            }
            ProjectType::Python => {
                if self.has_command("black") {
                    let mut cmd = vec!["black".to_string()];
                    if check {
                        cmd.push("--check".to_string());
                    }
                    cmd.push(".".to_string());
                    Ok(cmd)
                } else if self.has_command("ruff") {
                    let mut cmd = vec!["ruff".to_string(), "format".to_string()];
                    if check {
                        cmd.push("--check".to_string());
                    }
                    cmd.push(".".to_string());
                    Ok(cmd)
                } else {
                    Err(CldevError::Config(
                        "No Python formatter found. Please install black or ruff.".to_string(),
                    ))
                }
            }
            ProjectType::Ruby => {
                let mut cmd = vec![
                    "bundle".to_string(),
                    "exec".to_string(),
                    "rubocop".to_string(),
                ];
                if !check {
                    cmd.push("-A".to_string()); // Auto-correct
                }
                Ok(cmd)
            }
            ProjectType::Java => {
                // Google Java Format
                if check {
                    Ok(vec![
                        "java".to_string(),
                        "-jar".to_string(),
                        "google-java-format.jar".to_string(),
                        "--dry-run".to_string(),
                        "--set-exit-if-changed".to_string(),
                    ])
                } else {
                    Ok(vec![
                        "java".to_string(),
                        "-jar".to_string(),
                        "google-java-format.jar".to_string(),
                        "-i".to_string(),
                    ])
                }
            }
            ProjectType::Php => {
                let mut cmd = vec!["vendor/bin/phpcbf".to_string()];
                if check {
                    cmd = vec!["vendor/bin/phpcs".to_string()];
                }
                Ok(cmd)
            }
            ProjectType::DotNet => {
                let mut cmd = vec!["dotnet".to_string(), "format".to_string()];
                if check {
                    cmd.push("--verify-no-changes".to_string());
                }
                Ok(cmd)
            }
            ProjectType::Elixir => {
                let mut cmd = vec!["mix".to_string(), "format".to_string()];
                if check {
                    cmd.push("--check-formatted".to_string());
                }
                Ok(cmd)
            }
            ProjectType::Kotlin => {
                let mut cmd = vec!["./gradlew".to_string(), "ktlintFormat".to_string()];
                if check {
                    cmd = vec!["./gradlew".to_string(), "ktlintCheck".to_string()];
                }
                Ok(cmd)
            }
            ProjectType::Swift => {
                let mut cmd = vec!["swiftformat".to_string(), ".".to_string()];
                if check {
                    cmd.push("--lint".to_string());
                }
                Ok(cmd)
            }
            ProjectType::Scala => {
                let mut cmd = vec!["sbt".to_string(), "scalafmt".to_string()];
                if check {
                    cmd.push("--test".to_string());
                }
                Ok(cmd)
            }
            ProjectType::Unknown => Err(CldevError::Config(
                "Unknown project type. Cannot determine format command.".to_string(),
            )),
        }
    }

    /// Get test command for the detected project type
    ///
    /// # Arguments
    /// * `pattern` - Test pattern to filter tests
    /// * `coverage` - Whether to generate coverage report
    /// * `watch` - Whether to run in watch mode
    ///
    /// # Returns
    /// Command and arguments as a vector
    pub fn get_test_command(
        &self,
        pattern: Option<&str>,
        coverage: bool,
        watch: bool,
    ) -> Result<Vec<String>> {
        match self.project_type {
            ProjectType::NodeJs => {
                let mut cmd = vec!["npm".to_string(), "run".to_string()];

                // Determine which test command to use
                if coverage && self.has_script("test:coverage") {
                    cmd.push("test:coverage".to_string());
                } else if watch && self.has_script("test:watch") {
                    cmd.push("test:watch".to_string());
                } else if self.has_script("test") {
                    cmd.push("test".to_string());
                } else {
                    return Err(CldevError::Config(
                        "No test script found in package.json.".to_string(),
                    ));
                }

                // Add pattern if specified
                if let Some(p) = pattern {
                    cmd.push("--".to_string());
                    cmd.push(p.to_string());
                }

                Ok(cmd)
            }
            ProjectType::Rust => {
                let mut cmd = vec!["cargo".to_string(), "test".to_string()];

                if let Some(p) = pattern {
                    cmd.push(p.to_string());
                }

                if coverage {
                    // Use cargo-tarpaulin if available
                    if self.has_command("cargo-tarpaulin") {
                        cmd = vec!["cargo".to_string(), "tarpaulin".to_string()];
                    } else {
                        return Err(CldevError::Config(
                            "Coverage requested but cargo-tarpaulin not found. Install with: cargo install cargo-tarpaulin".to_string()
                        ));
                    }
                }

                Ok(cmd)
            }
            ProjectType::Go => {
                let mut cmd = vec!["go".to_string(), "test".to_string()];

                if coverage {
                    cmd.push("-cover".to_string());
                    cmd.push("-coverprofile=coverage.out".to_string());
                }

                if watch {
                    return Err(CldevError::Config(
                        "Watch mode not natively supported in Go. Consider using tools like 'gow' or 'air'.".to_string()
                    ));
                }

                if let Some(p) = pattern {
                    cmd.push("-run".to_string());
                    cmd.push(p.to_string());
                }

                cmd.push("./...".to_string());
                Ok(cmd)
            }
            ProjectType::Python => {
                if !self.has_command("pytest") {
                    return Err(CldevError::Config(
                        "pytest not found. Please install pytest.".to_string(),
                    ));
                }

                let mut cmd = vec!["pytest".to_string()];

                if coverage {
                    cmd.push("--cov".to_string());
                    cmd.push("--cov-report=html".to_string());
                }

                if watch {
                    cmd.push("--watch".to_string());
                }

                if let Some(p) = pattern {
                    cmd.push("-k".to_string());
                    cmd.push(p.to_string());
                }

                Ok(cmd)
            }
            ProjectType::Ruby => {
                let mut cmd = vec![
                    "bundle".to_string(),
                    "exec".to_string(),
                    "rspec".to_string(),
                ];

                if let Some(p) = pattern {
                    cmd.push("--pattern".to_string());
                    cmd.push(p.to_string());
                }

                if coverage {
                    // SimpleCov is typically configured in spec_helper.rb
                    // No additional flags needed
                }

                Ok(cmd)
            }
            ProjectType::Java => {
                if self.root.join("pom.xml").exists() {
                    // Maven
                    let mut cmd = vec!["mvn".to_string(), "test".to_string()];
                    if let Some(p) = pattern {
                        cmd.push(format!("-Dtest={}", p));
                    }
                    Ok(cmd)
                } else {
                    // Gradle
                    let mut cmd = vec!["./gradlew".to_string(), "test".to_string()];
                    if let Some(p) = pattern {
                        cmd.push(format!("--tests={}", p));
                    }
                    Ok(cmd)
                }
            }
            ProjectType::Php => {
                let mut cmd = vec!["vendor/bin/phpunit".to_string()];

                if coverage {
                    cmd.push("--coverage-html".to_string());
                    cmd.push("coverage".to_string());
                }

                if let Some(p) = pattern {
                    cmd.push("--filter".to_string());
                    cmd.push(p.to_string());
                }

                Ok(cmd)
            }
            ProjectType::DotNet => {
                let mut cmd = vec!["dotnet".to_string(), "test".to_string()];

                if coverage {
                    cmd.push("--collect".to_string());
                    cmd.push("Code Coverage".to_string());
                }

                if let Some(p) = pattern {
                    cmd.push("--filter".to_string());
                    cmd.push(p.to_string());
                }

                Ok(cmd)
            }
            ProjectType::Elixir => {
                let mut cmd = vec!["mix".to_string(), "test".to_string()];

                if coverage {
                    cmd = vec!["mix".to_string(), "coveralls".to_string()];
                }

                if let Some(p) = pattern {
                    cmd.push(p.to_string());
                }

                Ok(cmd)
            }
            ProjectType::Kotlin => {
                let mut cmd = vec!["./gradlew".to_string(), "test".to_string()];

                if let Some(p) = pattern {
                    cmd.push(format!("--tests={}", p));
                }

                Ok(cmd)
            }
            ProjectType::Swift => {
                let mut cmd = vec!["swift".to_string(), "test".to_string()];

                if let Some(p) = pattern {
                    cmd.push("--filter".to_string());
                    cmd.push(p.to_string());
                }

                if coverage {
                    cmd.push("--enable-code-coverage".to_string());
                }

                Ok(cmd)
            }
            ProjectType::Scala => {
                let mut cmd = vec!["sbt".to_string(), "test".to_string()];

                if let Some(p) = pattern {
                    cmd.push("--".to_string());
                    cmd.push("-z".to_string());
                    cmd.push(p.to_string());
                }

                if coverage {
                    cmd = vec![
                        "sbt".to_string(),
                        "coverage".to_string(),
                        "test".to_string(),
                        "coverageReport".to_string(),
                    ];
                }

                Ok(cmd)
            }
            ProjectType::Unknown => Err(CldevError::Config(
                "Unknown project type. Cannot determine test command.".to_string(),
            )),
        }
    }

    /// Check if package.json has a specific script
    fn has_script(&self, script_name: &str) -> bool {
        if self.project_type != ProjectType::NodeJs {
            return false;
        }

        let package_json_path = self.root.join("package.json");
        if let Ok(content) = fs::read_to_string(package_json_path) {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                if let Some(scripts) = json.get("scripts") {
                    return scripts.get(script_name).is_some();
                }
            }
        }

        false
    }

    /// Check if package.json has a specific dependency
    fn has_dependency(&self, dep_name: &str) -> bool {
        if self.project_type != ProjectType::NodeJs {
            return false;
        }

        let package_json_path = self.root.join("package.json");
        if let Ok(content) = fs::read_to_string(package_json_path) {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                if let Some(deps) = json.get("dependencies") {
                    if deps.get(dep_name).is_some() {
                        return true;
                    }
                }
                if let Some(dev_deps) = json.get("devDependencies") {
                    if dev_deps.get(dep_name).is_some() {
                        return true;
                    }
                }
            }
        }

        false
    }

    /// Check if a command is available in PATH
    fn has_command(&self, cmd: &str) -> bool {
        which::which(cmd).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_project(dir: &Path, project_type: ProjectType) {
        match project_type {
            ProjectType::NodeJs => {
                fs::write(dir.join("package.json"), r#"{"name": "test"}"#).unwrap();
            }
            ProjectType::Rust => {
                fs::write(dir.join("Cargo.toml"), "[package]\nname = \"test\"").unwrap();
            }
            ProjectType::Go => {
                fs::write(dir.join("go.mod"), "module test").unwrap();
            }
            ProjectType::Python => {
                fs::write(dir.join("pyproject.toml"), "[project]\nname = \"test\"").unwrap();
            }
            ProjectType::Ruby => {
                fs::write(dir.join("Gemfile"), "source 'https://rubygems.org'").unwrap();
            }
            ProjectType::Java => {
                fs::write(dir.join("pom.xml"), "<project></project>").unwrap();
            }
            ProjectType::Php => {
                fs::write(dir.join("composer.json"), r#"{"name": "test/test"}"#).unwrap();
            }
            ProjectType::DotNet => {
                fs::write(dir.join("test.csproj"), "<Project></Project>").unwrap();
            }
            ProjectType::Elixir => {
                fs::write(dir.join("mix.exs"), "defmodule Test.MixProject do\nend").unwrap();
            }
            ProjectType::Kotlin => {
                fs::write(dir.join("build.gradle.kts"), "plugins { kotlin(\"jvm\") }").unwrap();
            }
            ProjectType::Swift => {
                fs::write(dir.join("Package.swift"), "// swift-tools-version:5.5").unwrap();
            }
            ProjectType::Scala => {
                fs::write(dir.join("build.sbt"), "name := \"test\"").unwrap();
            }
            ProjectType::Unknown => {}
        }
    }

    #[test]
    fn test_detect_nodejs_project() {
        let temp_dir = TempDir::new().unwrap();
        create_test_project(temp_dir.path(), ProjectType::NodeJs);

        let detector = ProjectDetector::new(Some(temp_dir.path())).unwrap();
        assert_eq!(detector.project_type(), ProjectType::NodeJs);
    }

    #[test]
    fn test_detect_rust_project() {
        let temp_dir = TempDir::new().unwrap();
        create_test_project(temp_dir.path(), ProjectType::Rust);

        let detector = ProjectDetector::new(Some(temp_dir.path())).unwrap();
        assert_eq!(detector.project_type(), ProjectType::Rust);
    }

    #[test]
    fn test_detect_go_project() {
        let temp_dir = TempDir::new().unwrap();
        create_test_project(temp_dir.path(), ProjectType::Go);

        let detector = ProjectDetector::new(Some(temp_dir.path())).unwrap();
        assert_eq!(detector.project_type(), ProjectType::Go);
    }

    #[test]
    fn test_detect_python_project() {
        let temp_dir = TempDir::new().unwrap();
        create_test_project(temp_dir.path(), ProjectType::Python);

        let detector = ProjectDetector::new(Some(temp_dir.path())).unwrap();
        assert_eq!(detector.project_type(), ProjectType::Python);
    }

    #[test]
    fn test_unknown_project() {
        let temp_dir = TempDir::new().unwrap();

        let detector = ProjectDetector::new(Some(temp_dir.path())).unwrap();
        assert_eq!(detector.project_type(), ProjectType::Unknown);
    }

    #[test]
    fn test_detect_ruby_project() {
        let temp_dir = TempDir::new().unwrap();
        create_test_project(temp_dir.path(), ProjectType::Ruby);

        let detector = ProjectDetector::new(Some(temp_dir.path())).unwrap();
        assert_eq!(detector.project_type(), ProjectType::Ruby);
    }

    #[test]
    fn test_detect_java_project() {
        let temp_dir = TempDir::new().unwrap();
        create_test_project(temp_dir.path(), ProjectType::Java);

        let detector = ProjectDetector::new(Some(temp_dir.path())).unwrap();
        assert_eq!(detector.project_type(), ProjectType::Java);
    }

    #[test]
    fn test_detect_php_project() {
        let temp_dir = TempDir::new().unwrap();
        create_test_project(temp_dir.path(), ProjectType::Php);

        let detector = ProjectDetector::new(Some(temp_dir.path())).unwrap();
        assert_eq!(detector.project_type(), ProjectType::Php);
    }

    #[test]
    fn test_detect_dotnet_project() {
        let temp_dir = TempDir::new().unwrap();
        create_test_project(temp_dir.path(), ProjectType::DotNet);

        let detector = ProjectDetector::new(Some(temp_dir.path())).unwrap();
        assert_eq!(detector.project_type(), ProjectType::DotNet);
    }

    #[test]
    fn test_detect_elixir_project() {
        let temp_dir = TempDir::new().unwrap();
        create_test_project(temp_dir.path(), ProjectType::Elixir);

        let detector = ProjectDetector::new(Some(temp_dir.path())).unwrap();
        assert_eq!(detector.project_type(), ProjectType::Elixir);
    }

    #[test]
    fn test_detect_kotlin_project() {
        let temp_dir = TempDir::new().unwrap();
        create_test_project(temp_dir.path(), ProjectType::Kotlin);

        let detector = ProjectDetector::new(Some(temp_dir.path())).unwrap();
        assert_eq!(detector.project_type(), ProjectType::Kotlin);
    }

    #[test]
    fn test_detect_swift_project() {
        let temp_dir = TempDir::new().unwrap();
        create_test_project(temp_dir.path(), ProjectType::Swift);

        let detector = ProjectDetector::new(Some(temp_dir.path())).unwrap();
        assert_eq!(detector.project_type(), ProjectType::Swift);
    }

    #[test]
    fn test_detect_scala_project() {
        let temp_dir = TempDir::new().unwrap();
        create_test_project(temp_dir.path(), ProjectType::Scala);

        let detector = ProjectDetector::new(Some(temp_dir.path())).unwrap();
        assert_eq!(detector.project_type(), ProjectType::Scala);
    }

    #[test]
    fn test_monorepo_priority() {
        let temp_dir = TempDir::new().unwrap();

        // Create multiple project files
        fs::write(
            temp_dir.path().join("Cargo.toml"),
            "[package]\nname = \"test\"",
        )
        .unwrap();
        fs::write(temp_dir.path().join("package.json"), r#"{"name": "test"}"#).unwrap();
        fs::write(temp_dir.path().join("requirements.txt"), "pytest").unwrap();

        let detector = ProjectDetector::new(Some(temp_dir.path())).unwrap();
        // Rust should have highest priority
        assert_eq!(detector.project_type(), ProjectType::Rust);
    }

    #[test]
    fn test_kotlin_vs_java_detection() {
        let temp_dir = TempDir::new().unwrap();

        // Create build.gradle.kts (Kotlin-specific)
        fs::write(
            temp_dir.path().join("build.gradle.kts"),
            "plugins { kotlin(\"jvm\") }",
        )
        .unwrap();

        let detector = ProjectDetector::new(Some(temp_dir.path())).unwrap();
        assert_eq!(detector.project_type(), ProjectType::Kotlin);
    }

    #[test]
    fn test_project_type_names() {
        assert_eq!(ProjectType::NodeJs.name(), "Node.js");
        assert_eq!(ProjectType::Rust.name(), "Rust");
        assert_eq!(ProjectType::Go.name(), "Go");
        assert_eq!(ProjectType::Python.name(), "Python");
        assert_eq!(ProjectType::Ruby.name(), "Ruby");
        assert_eq!(ProjectType::Java.name(), "Java");
        assert_eq!(ProjectType::Php.name(), "PHP");
        assert_eq!(ProjectType::DotNet.name(), ".NET");
        assert_eq!(ProjectType::Elixir.name(), "Elixir");
        assert_eq!(ProjectType::Kotlin.name(), "Kotlin");
        assert_eq!(ProjectType::Swift.name(), "Swift");
        assert_eq!(ProjectType::Scala.name(), "Scala");
        assert_eq!(ProjectType::Unknown.name(), "Unknown");
    }

    #[test]
    fn test_project_type_extensions() {
        assert!(ProjectType::Ruby.extensions().contains(&"rb"));
        assert!(ProjectType::Java.extensions().contains(&"java"));
        assert!(ProjectType::Php.extensions().contains(&"php"));
        assert!(ProjectType::DotNet.extensions().contains(&"cs"));
        assert!(ProjectType::Elixir.extensions().contains(&"ex"));
        assert!(ProjectType::Kotlin.extensions().contains(&"kt"));
        assert!(ProjectType::Swift.extensions().contains(&"swift"));
        assert!(ProjectType::Scala.extensions().contains(&"scala"));
    }
}
