# cldev Multilingual Implementation Progress

**Last Updated**: 2025-11-07
**Status**: Phase 1 Complete (Urgent command multilingual support)

## Overview

This document tracks the progress of implementing multilingual support (English/Japanese) across all 29 cldev commands.

## Implementation Summary

### Phase 1: Demonstration Implementation ✅ COMPLETE

**Objective**: Implement complete multilingual support for `urgent` command as a reference pattern.

**Results**:
- ✅ Added 54 message keys to `messages.json` (27 en + 27 ja)
- ✅ Updated `urgent.rs` to use `OutputHandler` i18n methods
- ✅ Modified function signature to accept `OutputHandler` parameter
- ✅ Updated caller in `main.rs` to pass `OutputHandler`
- ✅ Successfully built and compiled
- ✅ Created comprehensive implementation guide

**Files Modified**:
1. `src/i18n/messages.json` - Added urgent command messages
2. `src/commands/dev/urgent.rs` - Replaced hardcoded strings with `output.t()` calls
3. `src/main.rs` - Updated `handle_dev_command()` to pass `OutputHandler`
4. `docs/i18n-implementation-guide.md` - Created detailed implementation guide

**Message Keys Added** (54 total):
```
urgent-header
urgent-separator
urgent-describe-incident
urgent-immediate-actions
urgent-select-affected-areas
urgent-severity-level
urgent-immediate-response-checklist
urgent-investigation-steps
urgent-mitigation-options
urgent-choose-mitigation
urgent-rollback-guide
urgent-rollback-intro
urgent-rollback-step1-4
urgent-incident-documentation
urgent-root-cause-prompt
urgent-immediate-action-prompt
urgent-next-steps
urgent-is-resolved
urgent-session-saved
urgent-incident-resolved
urgent-incident-ongoing
urgent-duration
urgent-session-id
urgent-session-path
urgent-tips-header
urgent-tip-stakeholders
urgent-tip-document
urgent-tip-systematic
urgent-tip-escalate
urgent-checklist-* (10 items)
urgent-investigation-* (6 items)
urgent-nextstep-* (5 items)
```

### Implementation Pattern Established

The `urgent.rs` implementation demonstrates the following pattern:

1. **Function Signature**: Add `output: &OutputHandler` parameter
2. **Headers**: `println!("{}", output.t("key").cyan().bold())`
3. **Prompts**: `.with_prompt(&output.t("key"))`
4. **Success Messages**: `output.success(&output.t("key"))`
5. **Dynamic Messages**: `output.t_format("key", "var", &value)`
6. **List Items**: Keep functional items (options) in English, translate prompts

## Remaining Work

### Phase 2: High Priority Commands (17 total)

**Estimated Effort**: 3-5 days
**Message Keys**: ~150-200

#### Dev Commands (6 remaining)
- [ ] fix.rs (~50 messages) - Similar to urgent.rs
- [ ] debug.rs (~60 messages) - Complex investigation workflow
- [ ] feature.rs (~70 messages) - Multi-step feature implementation
- [ ] refactor.rs (~40 messages)
- [ ] optimize.rs (~45 messages)
- [ ] research.rs (~35 messages)

#### Git Commands (4)
- [ ] commit.rs (~25 messages) - Conventional commits with emoji
- [ ] branch.rs (~20 messages) - Branch naming conventions
- [ ] merge_request.rs (~30 messages) - MR/PR creation
- [ ] status.rs (~15 messages) - Enhanced git status

#### Config Commands (6)
- [ ] init.rs (~40 messages) - Already partially multilingual
- [ ] check.rs (~20 messages)
- [ ] edit.rs (~15 messages)
- [ ] list.rs (~20 messages)
- [ ] maintain.rs (~15 messages, if exists)
- [ ] update_docs.rs (~15 messages, if exists)

### Phase 3: Medium Priority Commands (7 total)

**Estimated Effort**: 2-3 days
**Message Keys**: ~80-100

#### Quality Commands (3)
- [ ] lint.rs (~20 messages) - Linter execution with auto-fix
- [ ] format.rs (~15 messages) - Code formatting
- [ ] test.rs (~25 messages) - Test execution with coverage

#### Analysis Commands (4)
- [ ] analyze.rs (~30 messages) - Project analysis
- [ ] explain.rs (~20 messages) - Code explanation
- [ ] review_mr.rs (~35 messages) - MR/PR review
- [ ] serena.rs (~15 messages) - Semantic analysis

### Phase 4: Low Priority Commands (5 total)

**Estimated Effort**: 1-2 days
**Message Keys**: ~50-60

#### Tech Stack (1)
- [ ] start.rs (~20 messages) - Tech-specific environment startup

#### Operations (2)
- [ ] build.rs (~15 messages) - Build with optimization
- [ ] deploy.rs (~20 messages) - Deployment workflows

#### Learning Records (3)
- [ ] find.rs (~15 messages) - Search learning records
- [ ] stats.rs (~10 messages) - Statistics display
- [ ] problems.rs (~10 messages) - List unsolved problems
- [ ] new.rs (~15 messages) - Create new learning record

#### Todo (1)
- [ ] manage.rs (~20 messages) - Todo management

## Total Estimation

| Category | Commands | Message Keys | Estimated Days |
|----------|----------|--------------|----------------|
| Phase 1 (Done) | 1 | 54 | 1 ✅ |
| Phase 2 (High) | 16 | ~185 | 3-5 |
| Phase 3 (Medium) | 7 | ~90 | 2-3 |
| Phase 4 (Low) | 5 | ~55 | 1-2 |
| **TOTAL** | **29** | **~384** | **7-11 days** |

## Testing Strategy

### Manual Testing Checklist

For each implemented command:
```bash
# Test English output
./target/release/cldev --lang en [command] [args]

# Test Japanese output
./target/release/cldev --lang ja [command] [args]

# Test config-based language (default from cldev config init)
./target/release/cldev [command] [args]
```

### Automated Testing

Add tests to each command module:
```rust
#[test]
fn test_command_english() {
    let output = OutputHandler::with_language(false, false, false, Language::En);
    let msg = output.t("command-key");
    assert!(msg.contains("expected_english_text"));
}

#[test]
fn test_command_japanese() {
    let output = OutputHandler::with_language(false, false, false, Language::Ja);
    let msg = output.t("command-key");
    assert!(msg.contains("期待される日本語"));
}
```

## Implementation Guidelines

### Message Key Naming Convention

**Pattern**: `{command}-{section}-{purpose}`

Examples:
- `fix-header` - Main command header
- `fix-bug-description` - Bug description prompt
- `debug-investigation-steps` - Investigation steps section header
- `commit-type-select` - Commit type selection prompt

### Variable Substitution

For dynamic content:
```json
{
  "file-count": "Changed files: {count}",
  "session-duration": "Duration: {minutes} minutes",
  "command-result": "Result: {status}"
}
```

Usage:
```rust
output.info(&output.t_format("file-count", "count", &files.len().to_string()));
```

For multiple variables:
```rust
use std::collections::HashMap;
let mut vars = HashMap::new();
vars.insert("current", "3");
vars.insert("total", "10");
output.info(&output.t_with_vars("progress-step", &vars));
```

### What to Translate

✅ **DO Translate**:
- Headers and section titles
- User-facing prompts
- Success/error/warning messages
- Help text and descriptions
- Tips and best practices
- Next steps guidance

❌ **DON'T Translate**:
- Git commands (`git commit`, `git push`)
- Shell commands (`npm run dev`, `cargo build`)
- File paths and directory names
- Configuration keys
- Functional selection items (commit types, severity levels)
- Code snippets and examples
- Technical identifiers

### Preservation Rules

1. **Emojis**: Keep in both languages for visual consistency
2. **Formatting**: Preserve ANSI colors, bold, underline
3. **Structure**: Maintain list formatting, indentation
4. **Commands**: Keep command examples verbatim

## Quality Checklist

Before marking a command as complete:

- [ ] All user-facing messages translated
- [ ] Function signature updated to accept `OutputHandler`
- [ ] Caller in `main.rs` (or parent module) updated
- [ ] Message keys follow naming convention
- [ ] Variable substitution tested for dynamic messages
- [ ] Both languages tested manually
- [ ] Unit tests added for key messages
- [ ] Build succeeds without warnings
- [ ] Documentation updated

## Migration Script (Future Enhancement)

For bulk migration, consider creating a migration helper:

```rust
// src/tools/i18n_migrate.rs
pub fn extract_hardcoded_strings(file_path: &str) -> Vec<String> {
    // Parse Rust file and extract println! and dialoguer prompt strings
}

pub fn generate_message_keys(strings: Vec<String>, prefix: &str) -> HashMap<String, String> {
    // Generate message keys following convention
}

pub fn replace_strings_with_t_calls(file_path: &str, replacements: &HashMap<String, String>) {
    // Replace hardcoded strings with output.t() calls
}
```

## References

- **Implementation Guide**: [docs/i18n-implementation-guide.md](./i18n-implementation-guide.md)
- **Reference Implementation**: [src/commands/dev/urgent.rs](../src/commands/dev/urgent.rs)
- **Message Catalog**: [src/i18n/messages.json](../src/i18n/messages.json)
- **OutputHandler**: [src/cli/output.rs](../src/cli/output.rs)
- **I18n Module**: [src/core/i18n.rs](../src/core/i18n.rs)

## Next Steps

1. **Immediate** (This Week):
   - Implement `fix.rs` multilingual support (similar complexity to `urgent.rs`)
   - Implement `debug.rs` multilingual support
   - Test both commands thoroughly

2. **Short-term** (Next 2 Weeks):
   - Complete all Phase 2 high-priority commands
   - Set up automated testing for multilingual support
   - Create migration script for bulk processing

3. **Medium-term** (Next Month):
   - Complete Phase 3 and Phase 4 commands
   - Add language selection to interactive `cldev config init`
   - Document multilingual features in main README

4. **Long-term** (Future):
   - Consider adding more languages (Chinese, Korean, etc.)
   - Implement automatic message extraction tool
   - Create translation workflow for external contributors

---

## Appendix: Command Priority Rationale

**High Priority** (Urgent, Fix, Debug, Feature, Git commands, Config):
- Most frequently used commands
- User-facing interactive workflows
- Critical for daily development

**Medium Priority** (Quality, Analysis):
- Regular but not daily use
- Important for code quality
- Less interactive, more output-focused

**Low Priority** (Tech, Ops, LR, Todo):
- Specialized use cases
- Infrastructure/tooling
- Can fallback to English if needed

---

**Contributors**: Claude Code
**Maintainer**: cldev project team
**License**: Same as cldev project
