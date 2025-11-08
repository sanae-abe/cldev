//! Project type detection and command generation
//!
//! This module provides automatic project type detection based on configuration files
//! and generates appropriate commands for linting, formatting, and testing.

#![allow(dead_code)]

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

/// Supported frontend/backend frameworks
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Framework {
    // Frontend frameworks
    React,
    Vue,
    Angular,
    Svelte,
    Solid,
    // Meta frameworks
    Next,
    Nuxt,
    SvelteKit,
    Remix,
    Astro,
    Gatsby,
    // Build tools / Dev servers
    Vite,
    Webpack,
    Parcel,
    Rollup,
    Esbuild,
    // Backend frameworks
    Express,
    Fastify,
    NestJS,
    Koa,
    Hapi,
    Django,
    Flask,
    FastAPI,
    Rails,
    Sinatra,
    SpringBoot,
    // Rust frameworks
    Actix,
    Axum,
    Rocket,
    Warp,
    // Unknown
    Unknown,
}

/// Supported build tools and package managers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildTool {
    // Node.js package managers
    Npm,
    Yarn,
    YarnBerry, // Yarn 2+
    Pnpm,
    Bun,
    // Rust
    Cargo,
    // Go
    GoMod,
    // Python
    Pip,
    Poetry,
    Pipenv,
    Uv,
    // Ruby
    Bundler,
    // Java/Kotlin
    Maven,
    Gradle,
    // PHP
    Composer,
    // .NET
    Nuget,
    // Swift
    SwiftPM,
    // Scala
    Sbt,
    // Unknown
    Unknown,
}

/// Supported monorepo tools
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MonorepoType {
    Lerna,
    Nx,
    Turborepo,
    Rush,
    Pnpm,  // pnpm workspaces
    Yarn,  // Yarn workspaces
    Npm,   // npm workspaces
    Cargo, // Cargo workspaces
    None,
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

impl Framework {
    /// Get human-readable name for the framework
    pub fn name(&self) -> &'static str {
        match self {
            Framework::React => "React",
            Framework::Vue => "Vue",
            Framework::Angular => "Angular",
            Framework::Svelte => "Svelte",
            Framework::Solid => "Solid",
            Framework::Next => "Next.js",
            Framework::Nuxt => "Nuxt",
            Framework::SvelteKit => "SvelteKit",
            Framework::Remix => "Remix",
            Framework::Astro => "Astro",
            Framework::Gatsby => "Gatsby",
            Framework::Vite => "Vite",
            Framework::Webpack => "Webpack",
            Framework::Parcel => "Parcel",
            Framework::Rollup => "Rollup",
            Framework::Esbuild => "esbuild",
            Framework::Express => "Express",
            Framework::Fastify => "Fastify",
            Framework::NestJS => "NestJS",
            Framework::Koa => "Koa",
            Framework::Hapi => "Hapi",
            Framework::Django => "Django",
            Framework::Flask => "Flask",
            Framework::FastAPI => "FastAPI",
            Framework::Rails => "Rails",
            Framework::Sinatra => "Sinatra",
            Framework::SpringBoot => "Spring Boot",
            Framework::Actix => "Actix-web",
            Framework::Axum => "Axum",
            Framework::Rocket => "Rocket",
            Framework::Warp => "Warp",
            Framework::Unknown => "Unknown",
        }
    }
}

impl BuildTool {
    /// Get human-readable name for the build tool
    pub fn name(&self) -> &'static str {
        match self {
            BuildTool::Npm => "npm",
            BuildTool::Yarn => "Yarn",
            BuildTool::YarnBerry => "Yarn Berry",
            BuildTool::Pnpm => "pnpm",
            BuildTool::Bun => "Bun",
            BuildTool::Cargo => "Cargo",
            BuildTool::GoMod => "Go Modules",
            BuildTool::Pip => "pip",
            BuildTool::Poetry => "Poetry",
            BuildTool::Pipenv => "Pipenv",
            BuildTool::Uv => "uv",
            BuildTool::Bundler => "Bundler",
            BuildTool::Maven => "Maven",
            BuildTool::Gradle => "Gradle",
            BuildTool::Composer => "Composer",
            BuildTool::Nuget => "NuGet",
            BuildTool::SwiftPM => "Swift Package Manager",
            BuildTool::Sbt => "sbt",
            BuildTool::Unknown => "Unknown",
        }
    }
}

impl MonorepoType {
    /// Get human-readable name for the monorepo type
    pub fn name(&self) -> &'static str {
        match self {
            MonorepoType::Lerna => "Lerna",
            MonorepoType::Nx => "Nx",
            MonorepoType::Turborepo => "Turborepo",
            MonorepoType::Rush => "Rush",
            MonorepoType::Pnpm => "pnpm workspaces",
            MonorepoType::Yarn => "Yarn workspaces",
            MonorepoType::Npm => "npm workspaces",
            MonorepoType::Cargo => "Cargo workspaces",
            MonorepoType::None => "None",
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
    /// Detected frameworks (can be multiple)
    frameworks: Vec<Framework>,
    /// Detected build tool
    build_tool: BuildTool,
    /// Detected monorepo type
    monorepo_type: MonorepoType,
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
        let frameworks = Self::detect_frameworks(&root, project_type);
        let build_tool = Self::detect_build_tool(&root, project_type);
        let monorepo_type = Self::detect_monorepo_type(&root);

        Ok(Self {
            root,
            project_type,
            frameworks,
            build_tool,
            monorepo_type,
        })
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

    /// Detect frameworks used in the project
    fn detect_frameworks(root: &Path, project_type: ProjectType) -> Vec<Framework> {
        let mut frameworks = Vec::new();

        match project_type {
            ProjectType::NodeJs => {
                // Read package.json to detect frameworks
                if let Ok(content) = fs::read_to_string(root.join("package.json")) {
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                        let deps = Self::get_all_dependencies(&json);

                        // Meta frameworks (check first, as they include base frameworks)
                        if deps.contains_key("next") {
                            frameworks.push(Framework::Next);
                            frameworks.push(Framework::React);
                        } else if deps.contains_key("nuxt") {
                            frameworks.push(Framework::Nuxt);
                            frameworks.push(Framework::Vue);
                        } else if deps.contains_key("@sveltejs/kit") {
                            frameworks.push(Framework::SvelteKit);
                            frameworks.push(Framework::Svelte);
                        } else if deps.contains_key("@remix-run/react") {
                            frameworks.push(Framework::Remix);
                            frameworks.push(Framework::React);
                        } else if deps.contains_key("astro") {
                            frameworks.push(Framework::Astro);
                        } else if deps.contains_key("gatsby") {
                            frameworks.push(Framework::Gatsby);
                            frameworks.push(Framework::React);
                        }
                        // Base frameworks (if not already detected via meta framework)
                        else if deps.contains_key("react") || deps.contains_key("react-dom") {
                            frameworks.push(Framework::React);
                        } else if deps.contains_key("vue") {
                            frameworks.push(Framework::Vue);
                        } else if deps.contains_key("@angular/core") {
                            frameworks.push(Framework::Angular);
                        } else if deps.contains_key("svelte") {
                            frameworks.push(Framework::Svelte);
                        } else if deps.contains_key("solid-js") {
                            frameworks.push(Framework::Solid);
                        }

                        // Build tools / Dev servers
                        if deps.contains_key("vite")
                            || root.join("vite.config.ts").exists()
                            || root.join("vite.config.js").exists()
                        {
                            frameworks.push(Framework::Vite);
                        } else if deps.contains_key("webpack")
                            || root.join("webpack.config.js").exists()
                        {
                            frameworks.push(Framework::Webpack);
                        } else if deps.contains_key("parcel") {
                            frameworks.push(Framework::Parcel);
                        } else if deps.contains_key("rollup")
                            || root.join("rollup.config.js").exists()
                        {
                            frameworks.push(Framework::Rollup);
                        } else if deps.contains_key("esbuild") {
                            frameworks.push(Framework::Esbuild);
                        }

                        // Backend frameworks
                        if deps.contains_key("express") {
                            frameworks.push(Framework::Express);
                        }
                        if deps.contains_key("fastify") {
                            frameworks.push(Framework::Fastify);
                        }
                        if deps.contains_key("@nestjs/core") {
                            frameworks.push(Framework::NestJS);
                        }
                        if deps.contains_key("koa") {
                            frameworks.push(Framework::Koa);
                        }
                        if deps.contains_key("@hapi/hapi") {
                            frameworks.push(Framework::Hapi);
                        }
                    }
                }
            }
            ProjectType::Python => {
                // Check requirements.txt, pyproject.toml
                if let Ok(content) = fs::read_to_string(root.join("requirements.txt")) {
                    let lower = content.to_lowercase();
                    if lower.contains("django") {
                        frameworks.push(Framework::Django);
                    }
                    if lower.contains("flask") {
                        frameworks.push(Framework::Flask);
                    }
                    if lower.contains("fastapi") {
                        frameworks.push(Framework::FastAPI);
                    }
                }

                // Check pyproject.toml
                if let Ok(content) = fs::read_to_string(root.join("pyproject.toml")) {
                    let lower = content.to_lowercase();
                    if lower.contains("django") {
                        frameworks.push(Framework::Django);
                    }
                    if lower.contains("flask") {
                        frameworks.push(Framework::Flask);
                    }
                    if lower.contains("fastapi") {
                        frameworks.push(Framework::FastAPI);
                    }
                }
            }
            ProjectType::Ruby => {
                if let Ok(content) = fs::read_to_string(root.join("Gemfile")) {
                    let lower = content.to_lowercase();
                    if lower.contains("'rails'") || lower.contains("\"rails\"") {
                        frameworks.push(Framework::Rails);
                    }
                    if lower.contains("'sinatra'") || lower.contains("\"sinatra\"") {
                        frameworks.push(Framework::Sinatra);
                    }
                }
            }
            ProjectType::Java | ProjectType::Kotlin => {
                // Check for Spring Boot
                if let Ok(content) = fs::read_to_string(root.join("pom.xml")) {
                    if content.contains("spring-boot") {
                        frameworks.push(Framework::SpringBoot);
                    }
                } else if let Ok(content) = fs::read_to_string(root.join("build.gradle")) {
                    if content.contains("spring-boot") {
                        frameworks.push(Framework::SpringBoot);
                    }
                } else if let Ok(content) = fs::read_to_string(root.join("build.gradle.kts")) {
                    if content.contains("spring-boot") {
                        frameworks.push(Framework::SpringBoot);
                    }
                }
            }
            ProjectType::Rust => {
                // Check Cargo.toml for Rust web frameworks
                if let Ok(content) = fs::read_to_string(root.join("Cargo.toml")) {
                    let lower = content.to_lowercase();
                    if lower.contains("actix-web") {
                        frameworks.push(Framework::Actix);
                    }
                    if lower.contains("axum") {
                        frameworks.push(Framework::Axum);
                    }
                    if lower.contains("rocket") {
                        frameworks.push(Framework::Rocket);
                    }
                    if lower.contains("warp") {
                        frameworks.push(Framework::Warp);
                    }
                }
            }
            _ => {}
        }

        if frameworks.is_empty() {
            frameworks.push(Framework::Unknown);
        }

        frameworks
    }

    /// Detect build tool used in the project
    fn detect_build_tool(root: &Path, project_type: ProjectType) -> BuildTool {
        match project_type {
            ProjectType::NodeJs => {
                // Check lock files first (most reliable)
                if root.join("bun.lockb").exists() {
                    return BuildTool::Bun;
                }
                if root.join("pnpm-lock.yaml").exists() {
                    return BuildTool::Pnpm;
                }
                if root.join("yarn.lock").exists() {
                    // Check if it's Yarn Berry (v2+)
                    if root.join(".yarnrc.yml").exists() {
                        return BuildTool::YarnBerry;
                    }
                    return BuildTool::Yarn;
                }
                if root.join("package-lock.json").exists() {
                    return BuildTool::Npm;
                }

                // Check for package.json with packageManager field
                if let Ok(content) = fs::read_to_string(root.join("package.json")) {
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                        if let Some(pkg_manager) =
                            json.get("packageManager").and_then(|v| v.as_str())
                        {
                            if pkg_manager.starts_with("pnpm") {
                                return BuildTool::Pnpm;
                            }
                            if pkg_manager.starts_with("yarn") {
                                return BuildTool::Yarn;
                            }
                            if pkg_manager.starts_with("bun") {
                                return BuildTool::Bun;
                            }
                            if pkg_manager.starts_with("npm") {
                                return BuildTool::Npm;
                            }
                        }
                    }
                }

                // Default to npm if package.json exists
                BuildTool::Npm
            }
            ProjectType::Rust => BuildTool::Cargo,
            ProjectType::Go => BuildTool::GoMod,
            ProjectType::Python => {
                // Check for Poetry
                if root.join("poetry.lock").exists() || root.join("pyproject.toml").exists() {
                    if let Ok(content) = fs::read_to_string(root.join("pyproject.toml")) {
                        if content.contains("[tool.poetry]") {
                            return BuildTool::Poetry;
                        }
                    }
                }
                // Check for Pipenv
                if root.join("Pipfile").exists() || root.join("Pipfile.lock").exists() {
                    return BuildTool::Pipenv;
                }
                // Check for uv
                if root.join("uv.lock").exists() {
                    return BuildTool::Uv;
                }
                // Default to pip
                BuildTool::Pip
            }
            ProjectType::Ruby => BuildTool::Bundler,
            ProjectType::Java | ProjectType::Kotlin => {
                if root.join("pom.xml").exists() {
                    BuildTool::Maven
                } else {
                    BuildTool::Gradle
                }
            }
            ProjectType::Php => BuildTool::Composer,
            ProjectType::DotNet => BuildTool::Nuget,
            ProjectType::Swift => BuildTool::SwiftPM,
            ProjectType::Scala => BuildTool::Sbt,
            _ => BuildTool::Unknown,
        }
    }

    /// Detect monorepo type
    fn detect_monorepo_type(root: &Path) -> MonorepoType {
        // Check for Turborepo
        if root.join("turbo.json").exists() {
            return MonorepoType::Turborepo;
        }

        // Check for Nx
        if root.join("nx.json").exists() {
            return MonorepoType::Nx;
        }

        // Check for Lerna
        if root.join("lerna.json").exists() {
            return MonorepoType::Lerna;
        }

        // Check for Rush
        if root.join("rush.json").exists() {
            return MonorepoType::Rush;
        }

        // Check for pnpm workspaces
        if root.join("pnpm-workspace.yaml").exists() {
            return MonorepoType::Pnpm;
        }

        // Check for Cargo workspaces
        if let Ok(content) = fs::read_to_string(root.join("Cargo.toml")) {
            if content.contains("[workspace]") {
                return MonorepoType::Cargo;
            }
        }

        // Check package.json for workspaces (Yarn or npm)
        if let Ok(content) = fs::read_to_string(root.join("package.json")) {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                if json.get("workspaces").is_some() {
                    // Determine if it's Yarn or npm based on lock file
                    if root.join("yarn.lock").exists() {
                        return MonorepoType::Yarn;
                    }
                    if root.join("package-lock.json").exists() {
                        return MonorepoType::Npm;
                    }
                    // Default to npm if no lock file
                    return MonorepoType::Npm;
                }
            }
        }

        MonorepoType::None
    }

    /// Helper to get all dependencies from package.json
    fn get_all_dependencies(json: &serde_json::Value) -> std::collections::HashMap<String, String> {
        let mut deps = std::collections::HashMap::new();

        if let Some(dependencies) = json.get("dependencies").and_then(|v| v.as_object()) {
            for (key, value) in dependencies {
                deps.insert(key.clone(), value.as_str().unwrap_or_default().to_string());
            }
        }

        if let Some(dev_dependencies) = json.get("devDependencies").and_then(|v| v.as_object()) {
            for (key, value) in dev_dependencies {
                deps.insert(key.clone(), value.as_str().unwrap_or_default().to_string());
            }
        }

        deps
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

    /// Get the detected frameworks
    pub fn frameworks(&self) -> &[Framework] {
        &self.frameworks
    }

    /// Get the detected build tool
    pub fn build_tool(&self) -> BuildTool {
        self.build_tool
    }

    /// Get the detected monorepo type
    pub fn monorepo_type(&self) -> MonorepoType {
        self.monorepo_type
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
                // Don't fail on warnings, just report them
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

    #[test]
    fn test_framework_detection_react() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(
            temp_dir.path().join("package.json"),
            r#"{"dependencies": {"react": "^18.0.0", "react-dom": "^18.0.0"}}"#,
        )
        .unwrap();

        let detector = ProjectDetector::new(Some(temp_dir.path())).unwrap();
        assert_eq!(detector.project_type(), ProjectType::NodeJs);
        assert!(detector.frameworks().contains(&Framework::React));
    }

    #[test]
    fn test_framework_detection_next() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(
            temp_dir.path().join("package.json"),
            r#"{"dependencies": {"next": "^14.0.0", "react": "^18.0.0"}}"#,
        )
        .unwrap();

        let detector = ProjectDetector::new(Some(temp_dir.path())).unwrap();
        assert!(detector.frameworks().contains(&Framework::Next));
        assert!(detector.frameworks().contains(&Framework::React));
    }

    #[test]
    fn test_framework_detection_vue() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(
            temp_dir.path().join("package.json"),
            r#"{"dependencies": {"vue": "^3.0.0"}}"#,
        )
        .unwrap();

        let detector = ProjectDetector::new(Some(temp_dir.path())).unwrap();
        assert!(detector.frameworks().contains(&Framework::Vue));
    }

    #[test]
    fn test_framework_detection_vite() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(
            temp_dir.path().join("package.json"),
            r#"{"devDependencies": {"vite": "^5.0.0"}}"#,
        )
        .unwrap();

        let detector = ProjectDetector::new(Some(temp_dir.path())).unwrap();
        assert!(detector.frameworks().contains(&Framework::Vite));
    }

    #[test]
    fn test_framework_detection_rust_axum() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(
            temp_dir.path().join("Cargo.toml"),
            r#"[package]
name = "test"

[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
"#,
        )
        .unwrap();

        let detector = ProjectDetector::new(Some(temp_dir.path())).unwrap();
        assert_eq!(detector.project_type(), ProjectType::Rust);
        assert!(detector.frameworks().contains(&Framework::Axum));
    }

    #[test]
    fn test_build_tool_detection_npm() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("package.json"), r#"{"name": "test"}"#).unwrap();
        fs::write(temp_dir.path().join("package-lock.json"), "{}").unwrap();

        let detector = ProjectDetector::new(Some(temp_dir.path())).unwrap();
        assert_eq!(detector.build_tool(), BuildTool::Npm);
    }

    #[test]
    fn test_build_tool_detection_yarn() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("package.json"), r#"{"name": "test"}"#).unwrap();
        fs::write(temp_dir.path().join("yarn.lock"), "").unwrap();

        let detector = ProjectDetector::new(Some(temp_dir.path())).unwrap();
        assert_eq!(detector.build_tool(), BuildTool::Yarn);
    }

    #[test]
    fn test_build_tool_detection_pnpm() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("package.json"), r#"{"name": "test"}"#).unwrap();
        fs::write(temp_dir.path().join("pnpm-lock.yaml"), "").unwrap();

        let detector = ProjectDetector::new(Some(temp_dir.path())).unwrap();
        assert_eq!(detector.build_tool(), BuildTool::Pnpm);
    }

    #[test]
    fn test_build_tool_detection_bun() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("package.json"), r#"{"name": "test"}"#).unwrap();
        fs::write(temp_dir.path().join("bun.lockb"), "").unwrap();

        let detector = ProjectDetector::new(Some(temp_dir.path())).unwrap();
        assert_eq!(detector.build_tool(), BuildTool::Bun);
    }

    #[test]
    fn test_build_tool_detection_cargo() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(
            temp_dir.path().join("Cargo.toml"),
            "[package]\nname = \"test\"",
        )
        .unwrap();

        let detector = ProjectDetector::new(Some(temp_dir.path())).unwrap();
        assert_eq!(detector.build_tool(), BuildTool::Cargo);
    }

    #[test]
    fn test_build_tool_detection_poetry() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(
            temp_dir.path().join("pyproject.toml"),
            r#"[tool.poetry]
name = "test"
version = "0.1.0"
"#,
        )
        .unwrap();

        let detector = ProjectDetector::new(Some(temp_dir.path())).unwrap();
        assert_eq!(detector.build_tool(), BuildTool::Poetry);
    }

    #[test]
    fn test_monorepo_detection_turborepo() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("package.json"), r#"{"name": "test"}"#).unwrap();
        fs::write(temp_dir.path().join("turbo.json"), r#"{"pipeline": {}}"#).unwrap();

        let detector = ProjectDetector::new(Some(temp_dir.path())).unwrap();
        assert_eq!(detector.monorepo_type(), MonorepoType::Turborepo);
    }

    #[test]
    fn test_monorepo_detection_nx() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("package.json"), r#"{"name": "test"}"#).unwrap();
        fs::write(temp_dir.path().join("nx.json"), r#"{}"#).unwrap();

        let detector = ProjectDetector::new(Some(temp_dir.path())).unwrap();
        assert_eq!(detector.monorepo_type(), MonorepoType::Nx);
    }

    #[test]
    fn test_monorepo_detection_lerna() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("package.json"), r#"{"name": "test"}"#).unwrap();
        fs::write(
            temp_dir.path().join("lerna.json"),
            r#"{"version": "0.0.0"}"#,
        )
        .unwrap();

        let detector = ProjectDetector::new(Some(temp_dir.path())).unwrap();
        assert_eq!(detector.monorepo_type(), MonorepoType::Lerna);
    }

    #[test]
    fn test_monorepo_detection_pnpm_workspace() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("package.json"), r#"{"name": "test"}"#).unwrap();
        fs::write(
            temp_dir.path().join("pnpm-workspace.yaml"),
            "packages:\n  - 'packages/*'",
        )
        .unwrap();

        let detector = ProjectDetector::new(Some(temp_dir.path())).unwrap();
        assert_eq!(detector.monorepo_type(), MonorepoType::Pnpm);
    }

    #[test]
    fn test_monorepo_detection_npm_workspace() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(
            temp_dir.path().join("package.json"),
            r#"{"name": "test", "workspaces": ["packages/*"]}"#,
        )
        .unwrap();
        fs::write(temp_dir.path().join("package-lock.json"), "{}").unwrap();

        let detector = ProjectDetector::new(Some(temp_dir.path())).unwrap();
        assert_eq!(detector.monorepo_type(), MonorepoType::Npm);
    }

    #[test]
    fn test_monorepo_detection_cargo_workspace() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(
            temp_dir.path().join("Cargo.toml"),
            r#"[workspace]
members = ["crates/*"]
"#,
        )
        .unwrap();

        let detector = ProjectDetector::new(Some(temp_dir.path())).unwrap();
        assert_eq!(detector.monorepo_type(), MonorepoType::Cargo);
    }

    #[test]
    fn test_monorepo_detection_none() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("package.json"), r#"{"name": "test"}"#).unwrap();

        let detector = ProjectDetector::new(Some(temp_dir.path())).unwrap();
        assert_eq!(detector.monorepo_type(), MonorepoType::None);
    }

    #[test]
    fn test_framework_names() {
        assert_eq!(Framework::React.name(), "React");
        assert_eq!(Framework::Vue.name(), "Vue");
        assert_eq!(Framework::Next.name(), "Next.js");
        assert_eq!(Framework::Vite.name(), "Vite");
        assert_eq!(Framework::Axum.name(), "Axum");
    }

    #[test]
    fn test_build_tool_names() {
        assert_eq!(BuildTool::Npm.name(), "npm");
        assert_eq!(BuildTool::Yarn.name(), "Yarn");
        assert_eq!(BuildTool::Pnpm.name(), "pnpm");
        assert_eq!(BuildTool::Cargo.name(), "Cargo");
        assert_eq!(BuildTool::Poetry.name(), "Poetry");
    }

    #[test]
    fn test_monorepo_type_names() {
        assert_eq!(MonorepoType::Turborepo.name(), "Turborepo");
        assert_eq!(MonorepoType::Nx.name(), "Nx");
        assert_eq!(MonorepoType::Lerna.name(), "Lerna");
        assert_eq!(MonorepoType::Pnpm.name(), "pnpm workspaces");
        assert_eq!(MonorepoType::None.name(), "None");
    }
}
