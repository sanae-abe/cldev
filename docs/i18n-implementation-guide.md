# cldev Multilingual Implementation Guide

## Overview

This guide documents the pattern for implementing multilingual support across all 29 cldev commands using the OutputHandler i18n system.

## Architecture

### Components

1. **OutputHandler** (`src/cli/output.rs`):
   - `t(key)`: Get localized message
   - `t_format(key, var_name, var_value)`: Get localized message with single variable
   - `t_with_vars(key, vars)`: Get localized message with multiple variables

2. **Messages Catalog** (`src/i18n/messages.json`):
   - Structured JSON with `en` and `ja` language keys
   - Supports variable substitution with `{variable_name}` syntax

3. **I18n Module** (`src/core/i18n.rs`):
   - Language detection from config or `--lang` flag
   - Message loading and variable substitution

## Implementation Pattern

### Step 1: Identify Hardcoded Strings

Example from `urgent.rs`:
```rust
// ❌ Before (hardcoded)
println!("{}", "🚨 URGENT: Production Incident Response".red().bold());
println!("📝 Describe the incident (be specific)");
println!("✅ Session saved");
```

### Step 2: Add Message Keys to messages.json

```json
{
  "en": {
    "urgent-header": "🚨 URGENT: Production Incident Response",
    "urgent-describe-incident": "📝 Describe the incident (be specific)",
    "urgent-session-saved": "✅ Session saved"
  },
  "ja": {
    "urgent-header": "🚨 緊急: 本番インシデント対応",
    "urgent-describe-incident": "📝 インシデントの詳細を記述してください",
    "urgent-session-saved": "✅ セッション保存完了"
  }
}
```

### Step 3: Update Command Implementation

```rust
// ✅ After (multilingual)
pub fn handle_urgent(problem: Option<String>, output: &OutputHandler) -> Result<()> {
    println!("{}", output.t("urgent-header").red().bold());

    let problem_desc = if let Some(p) = problem {
        p
    } else {
        Input::<String>::new()
            .with_prompt(&output.t("urgent-describe-incident"))
            .interact_text()?
    };

    output.success(&output.t("urgent-session-saved"));
    // ...
}
```

### Step 4: Pass OutputHandler to Command

Update command entry point to pass OutputHandler:

```rust
// In src/commands/dev/mod.rs
pub fn execute_urgent(problem: Option<String>, output: &OutputHandler) -> Result<()> {
    handle_urgent(problem, output)
}
```

## Message Key Naming Convention

### Pattern: `{command}-{section}-{purpose}`

Examples:
- `urgent-header`: Main header
- `urgent-describe-incident`: Prompt for incident description
- `urgent-session-saved`: Success message
- `fix-bug-description`: Bug description prompt
- `debug-investigation-checklist`: Investigation checklist header

### Variable Substitution

For messages with dynamic content:

```json
{
  "urgent-session-id": "Session ID: {id}",
  "urgent-duration": "Duration: {minutes} minutes",
  "file-count": "Changed files: {count}"
}
```

Usage:
```rust
output.info(&output.t_format("urgent-session-id", "id", &session.id));
output.info(&output.t_format("urgent-duration", "minutes", &duration.to_string()));
```

For multiple variables:
```rust
use std::collections::HashMap;

let mut vars = HashMap::new();
vars.insert("step", "1");
vars.insert("total", "5");
output.info(&output.t_with_vars("progress-step", &vars));
```

## Command-by-Command Implementation Checklist

### High Priority (17 commands)

#### Dev Commands (7)
- [ ] urgent.rs
- [ ] fix.rs
- [ ] debug.rs
- [ ] feature.rs
- [ ] refactor.rs
- [ ] optimize.rs
- [ ] research.rs

#### Git Commands (4)
- [ ] commit.rs
- [ ] branch.rs
- [ ] merge_request.rs
- [ ] status.rs

#### Config Commands (6)
- [ ] init.rs
- [ ] check.rs
- [ ] edit.rs
- [ ] list.rs
- [ ] maintain.rs (if exists)
- [ ] update_docs.rs (if exists)

### Medium Priority (7 commands)

#### Quality Commands (3)
- [ ] lint.rs
- [ ] format.rs
- [ ] test.rs

#### Analysis Commands (4)
- [ ] analyze.rs
- [ ] explain.rs
- [ ] review_mr.rs
- [ ] serena.rs

### Low Priority (5 commands)

#### Tech Commands (1)
- [ ] start.rs

#### Ops Commands (2)
- [ ] build.rs
- [ ] deploy.rs

#### Learning Record Commands (4)
- [ ] find.rs
- [ ] stats.rs
- [ ] problems.rs
- [ ] new.rs

#### Todo Commands (1)
- [ ] manage.rs

## Testing Strategy

### Manual Testing

```bash
# Test English output
cldev --lang en dev urgent "test incident"

# Test Japanese output
cldev --lang ja dev urgent "テストインシデント"

# Test config-based language
cldev config init  # Select language during setup
cldev dev urgent "incident"
```

### Automated Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::output::OutputHandler;
    use crate::core::i18n::Language;

    #[test]
    fn test_urgent_english() {
        let output = OutputHandler::with_language(false, false, false, Language::En);
        let msg = output.t("urgent-header");
        assert!(msg.contains("URGENT"));
    }

    #[test]
    fn test_urgent_japanese() {
        let output = OutputHandler::with_language(false, false, false, Language::Ja);
        let msg = output.t("urgent-header");
        assert!(msg.contains("緊急"));
    }
}
```

## Common Patterns

### Interactive Prompts

```rust
// Before
let input = Input::<String>::new()
    .with_prompt("Enter description")
    .interact_text()?;

// After
let input = Input::<String>::new()
    .with_prompt(&output.t("prompt-description"))
    .interact_text()?;
```

### Selection Menus

```rust
// Keep selection items as-is (they are functional, not UI)
let options = vec![
    "Option A",
    "Option B",
];

// But translate the prompt
let selection = Select::new()
    .with_prompt(&output.t("select-option-prompt"))
    .items(&options)
    .interact()?;
```

### Progress Messages

```rust
// Before
output.info("Processing...");
// Do work
output.success("Completed successfully");

// After
output.info(&output.t("processing"));
// Do work
output.success(&output.t("completed-successfully"));
```

### Section Headers

```rust
// Before
println!("{}", "🔍 INVESTIGATION STEPS".cyan().bold());

// After
println!("{}", output.t("section-investigation").cyan().bold());
```

## Message Organization in messages.json

Organize by command and section for clarity:

```json
{
  "en": {
    // Urgent command messages
    "urgent-header": "...",
    "urgent-describe-incident": "...",
    "urgent-impact-assessment": "...",
    "urgent-severity-level": "...",

    // Fix command messages
    "fix-header": "...",
    "fix-bug-description": "...",
    "fix-root-cause": "...",

    // Common messages (reusable across commands)
    "session-saved": "Session saved",
    "session-id": "Session ID: {id}",
    "next-steps": "Next steps:"
  },
  "ja": {
    // ... Japanese translations
  }
}
```

## Implementation Best Practices

1. **Preserve Emojis**: Keep emojis in both languages for visual consistency
2. **Maintain Formatting**: Preserve markdown/ANSI formatting in translations
3. **Keep Selection Items English**: Functional items (like commit types) stay in English
4. **Translate User-Facing Text**: Focus on prompts, headers, and output messages
5. **Use Consistent Terminology**: Maintain a glossary for technical terms
6. **Test Both Languages**: Ensure messages make sense in context for both en/ja

## Migration Strategy

### Phase 1: Core Commands (Week 1)
- Implement: urgent, fix, debug
- Add ~50 message keys
- Validate pattern

### Phase 2: High Priority (Week 2)
- Implement: feature, commit, config/init
- Add ~100 message keys
- Refine patterns

### Phase 3: Medium Priority (Week 3)
- Implement: quality, analysis commands
- Add ~80 message keys

### Phase 4: Low Priority (Week 4)
- Implement: tech, ops, lr, todo commands
- Add ~50 message keys
- Final testing

## Total Estimate

- **Messages to add**: ~280 keys × 2 languages = ~560 entries
- **Files to update**: 29 command files
- **Test coverage**: 58 test cases (2 per command)

## Reference Implementation

See `src/commands/dev/urgent.rs` for a complete example of multilingual implementation.

---

Last Updated: 2025-11-07
Version: 1.0
