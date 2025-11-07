# Supported Languages and Frameworks

`cldev` automatically detects project types and provides intelligent command suggestions for the following languages and frameworks.

## Supported Languages (12 Total)

### Systems Programming

#### 1. Rust
- **Detection Files**: `Cargo.toml`
- **Extensions**: `.rs`
- **Commands**:
  - Lint: `cargo clippy --all-targets --all-features -- -D warnings`
  - Format: `cargo fmt --check` (check) / `cargo fmt` (fix)
  - Test: `cargo test` (with `cargo-tarpaulin` for coverage)
- **Priority**: Highest (1st)

#### 2. Go
- **Detection Files**: `go.mod`
- **Extensions**: `.go`
- **Commands**:
  - Lint: `go vet ./...`
  - Format: `gofmt -l .` (check) / `go fmt ./...` (fix)
  - Test: `go test ./...` (with `-cover` for coverage)
- **Priority**: High (2nd)

---

### JVM Languages

#### 3. Java
- **Detection Files**: `pom.xml`, `build.gradle`
- **Extensions**: `.java`
- **Commands**:
  - Lint: `mvn checkstyle:check` (Maven) / `./gradlew checkstyleMain` (Gradle)
  - Format: `google-java-format` with `--dry-run` (check) / `-i` (fix)
  - Test: `mvn test` (Maven) / `./gradlew test` (Gradle)
- **Priority**: Medium-High (3rd)

#### 4. Kotlin
- **Detection Files**: `build.gradle.kts`, `build.gradle` + `.kt` files
- **Extensions**: `.kt`, `.kts`
- **Commands**:
  - Lint: `./gradlew ktlintCheck` / `./gradlew ktlintFormat` (fix)
  - Format: Same as lint
  - Test: `./gradlew test`
- **Priority**: Medium-High (4th, detected before Java if `.kts` present)
- **Note**: Distinguishes from Java by detecting `build.gradle.kts` or Kotlin source files

#### 5. Scala
- **Detection Files**: `build.sbt`
- **Extensions**: `.scala`
- **Commands**:
  - Lint: `sbt scalastyle`
  - Format: `sbt scalafmt` (with `--test` for check)
  - Test: `sbt test` (with `coverage` plugin for coverage)
- **Priority**: Medium (5th)

---

### Native Mobile

#### 6. Swift
- **Detection Files**: `Package.swift`
- **Extensions**: `.swift`
- **Commands**:
  - Lint: `swiftlint lint` (with `--fix` for auto-fix)
  - Format: `swiftformat .` (with `--lint` for check)
  - Test: `swift test` (with `--enable-code-coverage` for coverage)
- **Priority**: Medium (6th)

---

### .NET Ecosystem

#### 7. .NET (C#, F#, VB.NET)
- **Detection Files**: `*.csproj`, `*.fsproj`, `*.vbproj`, `*.sln`
- **Extensions**: `.cs`, `.fs`, `.vb`
- **Commands**:
  - Lint: `dotnet format --verify-no-changes` (check) / `dotnet format` (fix)
  - Format: Same as lint
  - Test: `dotnet test` (with `--collect "Code Coverage"` for coverage)
- **Priority**: Medium (7th)

---

### Dynamic Languages

#### 8. Ruby
- **Detection Files**: `Gemfile`, `.ruby-version`
- **Extensions**: `.rb`, `.rake`
- **Commands**:
  - Lint: `bundle exec rubocop` (with `-A` for auto-fix)
  - Format: `bundle exec rubocop -A`
  - Test: `bundle exec rspec` (SimpleCov for coverage)
- **Priority**: Medium-Low (8th)

#### 9. PHP
- **Detection Files**: `composer.json`
- **Extensions**: `.php`
- **Commands**:
  - Lint: `vendor/bin/phpcs` / `vendor/bin/phpcbf` (fix)
  - Format: `vendor/bin/phpcbf` / `vendor/bin/phpcs` (check)
  - Test: `vendor/bin/phpunit` (with `--coverage-html` for coverage)
- **Priority**: Medium-Low (9th)

#### 10. Elixir
- **Detection Files**: `mix.exs`
- **Extensions**: `.ex`, `.exs`
- **Commands**:
  - Lint: `mix credo` (with `--strict` for fix mode)
  - Format: `mix format` (with `--check-formatted` for check)
  - Test: `mix test` / `mix coveralls` (for coverage)
- **Priority**: Medium-Low (10th)

#### 11. Node.js (JavaScript/TypeScript)
- **Detection Files**: `package.json`
- **Extensions**: `.js`, `.jsx`, `.ts`, `.tsx`, `.mjs`, `.cjs`
- **Commands**:
  - Lint: `npm run lint` / `npx eslint --fix .` (auto-detect from package.json)
  - Format: `npm run format` / `npx prettier --write .` (auto-detect)
  - Test: `npm run test` (with coverage and watch mode support)
- **Priority**: Low (11th)

#### 12. Python
- **Detection Files**: `pyproject.toml`, `requirements.txt`, `setup.py`, `Pipfile`
- **Extensions**: `.py`, `.pyi`
- **Commands**:
  - Lint: `ruff check .` (preferred) / `pylint .` / `flake8 .`
  - Format: `black .` / `ruff format .` (with `--check` for check mode)
  - Test: `pytest` (with `--cov` for coverage, `--watch` for watch mode)
- **Priority**: Lowest (12th)

---

## Priority System for Monorepos

When multiple language configuration files are detected, `cldev` uses the following priority order:

1. **Compiled/Static Languages** (highest priority):
   - Rust > Go > Java/Kotlin > Swift > Scala > .NET

2. **Dynamic/Scripting Languages**:
   - Ruby > PHP > Elixir > Node.js > Python

This ensures that in monorepos combining multiple languages, the most performance-critical or foundational language takes precedence.

### Example Priority Resolution

```
Project contains:
- Cargo.toml (Rust)
- package.json (Node.js)
- requirements.txt (Python)

→ Detected as: Rust project (highest priority)
```

---

## Language-Specific Features

### Kotlin vs Java Auto-Detection

`cldev` intelligently distinguishes between Kotlin and Java projects:

1. **Kotlin Detection**:
   - Presence of `build.gradle.kts` (Kotlin DSL)
   - Presence of `.kt` or `.kts` files with `build.gradle`

2. **Java Detection**:
   - Presence of `pom.xml` or `build.gradle` without Kotlin indicators

### .NET Multi-Language Support

The .NET detector supports:
- C# (`.cs`, `*.csproj`)
- F# (`.fs`, `*.fsproj`)
- VB.NET (`.vb`, `*.vbproj`)
- Solution files (`*.sln`)

---

## Command Generation

For each language, `cldev` generates intelligent commands with the following features:

### Lint Commands
- **Auto-fix mode**: Automatically correct style violations when supported
- **All files mode**: Lint entire codebase vs. only changed files
- **Tool detection**: Automatically detect installed linters and use appropriate fallbacks

### Format Commands
- **Check mode**: Verify formatting without making changes
- **Fix mode**: Apply formatting changes automatically
- **Idempotent**: Safe to run multiple times

### Test Commands
- **Pattern filtering**: Run specific test patterns or suites
- **Coverage reporting**: Generate code coverage reports when requested
- **Watch mode**: Re-run tests on file changes (where supported)

---

## Adding New Languages

To add support for a new language:

1. Add the language to `ProjectType` enum in `src/core/project_detector.rs`
2. Implement detection logic in `detect_project_type()`
3. Add lint command generation in `get_lint_command()`
4. Add format command generation in `get_format_command()`
5. Add test command generation in `get_test_command()`
6. Add test cases for detection and command generation
7. Update this documentation

See the existing implementations for reference.

---

## Limitations and Future Support

### Currently Unsupported
- C/C++ (planned)
- Dart/Flutter (planned)
- Haskell (planned)
- OCaml/ReasonML (planned)

### Language-Specific Notes

- **Go**: Watch mode not natively supported; use tools like `gow` or `air`
- **Java**: Google Java Format requires separate download
- **Python**: Prefers `ruff` over `pylint` or `flake8` for performance
- **Ruby**: SimpleCov configuration needed for coverage reporting
- **Swift**: Code coverage requires Xcode 13.3+ or Swift 5.6+

---

## Version Compatibility

- Rust: 1.70+
- Go: 1.18+
- Java: 8+
- Kotlin: 1.5+
- Scala: 2.13+, 3.x
- Swift: 5.5+
- .NET: Core 3.1+, 5.0+, 6.0+
- Node.js: 14+
- Python: 3.8+
- Ruby: 2.7+
- PHP: 7.4+, 8.0+
- Elixir: 1.12+

---

Last updated: 2025-11-07
