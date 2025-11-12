# cldev - Rust CLI Development Context

## Project Overview

**Type**: Rust CLI tool unifying 33 development commands
**Tech Stack**: Rust 1.70+, clap 4.5, serde, tokio
**Performance**: 1.5MB binary, 21ms startup (1.5x faster than gh CLI)
**i18n**: 4 languages (en/ja/zh/zh-TW) - 584 keys each
**Commands**: 33 across 9 categories (config/dev/git/quality/ops/tech/analysis/lr/todo)

---

## Critical Rules (cldev-specific)

### Security (Strict Enforcement)

**NEVER access environment files:**
```
.env, .env.*, .envrc → READ/EDIT/CREATE ALL FORBIDDEN
```

**Path operations:**
- Use `secure_path_canonicalize` for all file paths
- Validate against path traversal attacks

**Before ANY commit:**
```bash
cargo clippy --all-targets --all-features  # MUST pass
cargo test --lib --quiet                   # MUST pass (--quiet: 詳細出力は失敗時のみ)
```

### Quality (Auto-enforced)

**Rust file edited → Auto format check:**
```bash
cargo fmt --check  # Auto-run on *.rs edit
cargo fmt          # Auto-fix if check fails
```

**Critical files require confirmation:**
- `Cargo.toml` / `Cargo.lock` → Confirm before edit
- `src/core/*.rs` → Extra caution (backward compatibility)

**Test coverage:**
- Minimum: 70% overall
- New features: 80% coverage required

### i18n (Strict - 4 Languages)

**Required languages:** en (English), ja (日本語), zh (简体中文), zh-TW (繁體中文)

**Rules:**
- `src/i18n/messages.json` edited → Validate ALL 4 languages have same keys
- New feature → Add translations for ALL 4 languages before PR
- No hardcoded strings in user-facing output

**Validation:**
```bash
# Check all languages have 584 keys
python3 -c "import json; data=json.load(open('src/i18n/messages.json'));
print(f'en: {len(data[\"en\"])} keys');
print(f'ja: {len(data[\"ja\"])} keys');
print(f'zh: {len(data[\"zh\"])} keys');
print(f'zh-TW: {len(data[\"zh-TW\"])} keys')"
```

---

## Common Workflows

### Feature Development (feature/*)
```
1. git checkout -b feature/xxx
2. [implement]
3. cargo fmt && cargo clippy
4. cargo test
5. git commit -m "feat(category): description"
6. gh pr create
7. (optional) cldev lr new  # Learning record
```

### Bug Fix (fix/*)
```
1. git checkout -b fix/xxx
2. [analyze → fix]
3. cargo test  # Include regression test
4. git commit -m "fix(category): description"
5. cldev lr new  # Document root cause
```

### Emergency (hotfix/*) - 5min response
```
1. git checkout -b hotfix/xxx
2. [minimal fix for availability]
3. cargo test --lib --quiet  # Fast essential tests only
4. git commit -m "hotfix: description"
5. (if production) Deploy immediately
6. cldev lr new  # Post-mortem REQUIRED
```

### Refactoring
```
1. cargo test  # Baseline (save output)
2. [incremental refactoring]
3. cargo test  # After EACH change
4. cargo clippy
5. git commit -m "refactor(category): description"
```

---

## Quality Commands (Quick Reference)

```bash
# Full quality check
cargo fmt && cargo clippy --all-targets --all-features && cargo test

# Fast check (pre-commit)
cargo fmt --check && cargo clippy --quiet && cargo test --lib --quiet

# Build release
cargo build --release --bin cldev

# Verify binary
./target/release/cldev --version
time ./target/release/cldev --version  # Should be ~21ms

# i18n check
./target/release/cldev --lang en --help
./target/release/cldev --lang ja --help
./target/release/cldev --lang zh --help
./target/release/cldev --lang zh-TW --help
```

---

## Project Structure

```
src/
├── main.rs
├── lib.rs
├── cli/           # CLI argument parsing
├── commands/      # 33 commands in 9 categories
│   ├── config/    # init, check, list, edit, maintain, update_docs
│   ├── dev/       # feature, fix, debug, urgent, refactor, optimize, research
│   ├── git/       # branch, commit, status, merge_request
│   ├── quality/   # format, lint, test
│   ├── ops/       # build, deploy
│   ├── tech/      # start
│   ├── analysis/  # analyze, explain, review_mr, serena
│   ├── lr/        # new, find, stats, problems
│   └── todo/      # manage
└── core/          # Core functionality
    ├── config.rs             # Config management
    ├── i18n.rs               # i18n system
    ├── security.rs           # Security utilities
    ├── learning_record_v2.rs # Learning records
    └── project_detector.rs   # Auto-detect project types

tests/
├── unit tests (lib)
├── integration tests
└── e2e tests

docs/
├── architecture/
├── development/
├── guides/
└── implementation/

.cldev/config.toml  # User configuration
```

---

## Naming Conventions

**Git branches:**
- `feature/*` - New features
- `fix/*` - Bug fixes
- `hotfix/*` - Emergency production fixes
- `refactor/*` - Code refactoring
- `docs/*` - Documentation only

**Commit messages:** (Conventional Commits)
```
feat(category): Add new command
fix(category): Fix bug description
refactor(category): Refactor description
docs(category): Update documentation
test(category): Add tests
chore(category): Maintenance tasks
```

**Categories:** config, dev, git, quality, ops, tech, analysis, lr, todo

---

## Dependencies & Versions

**Critical:**
- Rust: 1.70+ (MSRV)
- clap: 4.5 (CLI framework)
- serde: 1.0 (Serialization)
- tokio: Latest (Async runtime)

**Update strategy:**
- Patch updates: Auto-apply
- Minor updates: Review changelog
- Major updates: Full testing required

---

## Performance Targets

- Startup time: ≤ 25ms (currently 21ms)
- Binary size: ≤ 2MB (currently 1.5MB)
- Memory usage: ≤ 10MB
- Test suite: ≤ 60s (full), ≤ 5s (lib only)

---

## Emergency Contacts & References

**Quick fixes:**
```bash
# Build broken
cargo clean && cargo build

# Tests failing
cargo test --lib -- --nocapture

# Clippy errors
cargo clippy --fix --allow-dirty --allow-staged
```

**Documentation:**
- Architecture: `docs/architecture/`
- User Guide: `docs/USER_GUIDE.md`
- Developer Guide: `docs/DEVELOPER_GUIDE.md`
