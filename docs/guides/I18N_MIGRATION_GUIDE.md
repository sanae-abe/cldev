# i18n Migration Guide for Contributors

**Last Updated**: 2025-11-13

This guide helps contributors understand and implement internationalization (i18n) in cldev commands.

## Overview

cldev uses a centralized i18n system supporting:
- **Languages**: English (en), Japanese (ja)
- **Total keys**: 1,079 per language
- **Coverage**: All 33 commands across 9 categories

## Quick Start

### 1. Basic Pattern

**Before (hardcoded string):**
```rust
output.info("Checking configuration...");
```

**After (i18n):**
```rust
output.info(&output.t("config-check-header"));
```

### 2. Parameterized Strings

**Before:**
```rust
output.success(&format!("Created config at {}", path.display()));
```

**After:**
```rust
output.success(&output.t_format(
    "config-check-created-default",
    "path",
    &path.display().to_string()
));
```

### 3. Multiple Variables

**Before:**
```rust
output.info(&format!("Found {} errors in {} files", error_count, file_count));
```

**After:**
```rust
output.info(&output.t_with_vars(
    "error-summary",
    &[
        ("count", &error_count.to_string()),
        ("files", &file_count.to_string()),
    ]
));
```

## Adding New i18n Keys

### Step 1: Identify Strings to Translate

User-facing strings that need i18n:
- ✅ Command output messages
- ✅ Interactive prompts
- ✅ Validation messages
- ✅ Progress indicators
- ✅ Error messages displayed to users

Strings that DON'T need i18n:
- ❌ Internal error messages (CldevError)
- ❌ Debug/trace logs
- ❌ Code identifiers/variable names
- ❌ Technical output (JSON, raw data)

### Step 2: Create Key Names

Follow the naming convention: `{category}-{subcategory}-{detail}`

**Examples:**
```
config-check-header
config-check-file-not-found
git-branch-creating
feature-test-unit
quality-format-running
```

**Guidelines:**
- Use lowercase with hyphens
- Start with command category (config, git, quality, etc.)
- Be descriptive but concise
- Group related keys with same prefix

### Step 3: Add Translations to messages.json

Add to both `en` and `ja` sections in `src/i18n/messages.json`:

```json
{
  "en": {
    "config-check-header": "📋 Configuration Check",
    "config-check-file-not-found": "Configuration file not found: {path}",
    ...
  },
  "ja": {
    "config-check-header": "📋 設定チェック",
    "config-check-file-not-found": "設定ファイルが見つかりません: {path}",
    ...
  }
}
```

**Translation Guidelines:**

1. **Preserve emojis**: Use identical emojis in all languages
   ```json
   "en": "✅ All checks passed",
   "ja": "✅ すべてのチェックに合格しました"
   ```

2. **Keep technical terms in English**: CLI commands, file paths, technical jargon
   ```json
   "en": "Running cargo fmt...",
   "ja": "cargo fmt を実行中..."
   ```

3. **Preserve placeholders**: `{variable}` format must be identical
   ```json
   "en": "Found {count} errors",
   "ja": "{count} 個のエラーが見つかりました"
   ```

4. **Cultural adaptation**: Adjust phrasing to sound natural
   ```json
   "en": "Are you sure?",
   "ja": "よろしいですか？"  // More polite in Japanese
   ```

### Step 4: Update Command Code

Replace hardcoded strings with i18n method calls:

```rust
use crate::cli::output::OutputHandler;

pub fn my_command(
    // ... parameters
    output: &OutputHandler,
) -> Result<()> {
    // Simple string
    output.info(&output.t("my-command-starting"));

    // Parameterized string (single variable)
    output.success(&output.t_format(
        "my-command-completed",
        "duration",
        &duration.to_string()
    ));

    // Multiple variables
    output.info(&output.t_with_vars(
        "my-command-summary",
        &[
            ("files", &file_count.to_string()),
            ("errors", &error_count.to_string()),
        ]
    ));

    // Error message (user-facing)
    if some_error {
        output.error(&output.t("my-command-failed"));
        return Err(CldevError::command(
            // Internal error - no i18n needed
            format!("Internal error: {}", details)
        ));
    }

    Ok(())
}
```

### Step 5: Verify Implementation

**Build and test:**
```bash
# Build
cargo build --release --bin cldev --quiet

# Test both languages
./target/release/cldev --lang en my-command --help
./target/release/cldev --lang ja my-command --help

# Run tests
cargo test --lib --quiet
```

**Verify key counts match:**
```bash
python3 -c "
import json
data = json.load(open('src/i18n/messages.json'))
en_count = len(data['en'])
ja_count = len(data['ja'])
print(f'en: {en_count} keys')
print(f'ja: {ja_count} keys')
if en_count == ja_count:
    print(f'✅ Both languages match')
else:
    print(f'❌ Mismatch!')
"
```

## Common Patterns

### Interactive Selection Lists

```rust
use dialoguer::Select;

let options = vec![
    output.t("option-1"),
    output.t("option-2"),
    output.t("option-3"),
];

let selection = Select::new()
    .with_prompt(output.t("select-prompt"))
    .items(&options)
    .interact()?;
```

### Progress Messages

```rust
use indicatif::ProgressBar;

let pb = ProgressBar::new(total);
pb.set_message(output.t("progress-analyzing"));
// ... work
pb.set_message(output.t("progress-complete"));
pb.finish();
```

### Validation Results

```rust
// Success case
output.success(&output.t("validation-passed"));

// Failure case with details
output.error(&output.t_format(
    "validation-failed-count",
    "count",
    &failed_count.to_string()
));
```

## Testing i18n Changes

### Manual Testing

```bash
# Test English output
cldev --lang en config check

# Test Japanese output
cldev --lang ja config check

# Test with environment variable
export CLDEV_LANG=ja
cldev config check
```

### Automated Testing

Add language-specific tests:

```rust
#[test]
fn test_my_command_english() {
    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.arg("--lang")
        .arg("en")
        .args(["my-command"])
        .assert()
        .success()
        .stdout(predicate::str::contains("expected English text"));
}

#[test]
fn test_my_command_japanese() {
    let mut cmd = Command::cargo_bin("cldev").unwrap();

    cmd.arg("--lang")
        .arg("ja")
        .args(["my-command"])
        .assert()
        .success()
        .stdout(predicate::str::contains("期待される日本語テキスト"));
}
```

## Migration Checklist

When migrating a command to i18n:

- [ ] Identify all user-facing strings in the command
- [ ] Create descriptive key names following naming convention
- [ ] Add translations to messages.json for BOTH en and ja
- [ ] Update command code to use output.t() methods
- [ ] Verify key counts match between languages
- [ ] Build and test with both languages
- [ ] Run full test suite (cargo test --all-features)
- [ ] Update command documentation if needed

## Common Mistakes to Avoid

1. **Missing Japanese translation**
   - Always add to both "en" AND "ja" sections
   - Verify key counts match after editing

2. **Hardcoded strings in format macros**
   ```rust
   // ❌ Wrong
   output.info(&format!("Processing {}...", name));

   // ✅ Correct
   output.info(&output.t_format("processing", "name", &name));
   ```

3. **Translating internal errors**
   ```rust
   // ❌ Wrong - internal error doesn't need i18n
   return Err(CldevError::command(output.t("internal-error")));

   // ✅ Correct - internal errors use format!
   return Err(CldevError::command(format!("Failed to parse: {}", e)));
   ```

4. **Inconsistent placeholder names**
   ```json
   // ❌ Wrong - different placeholders
   "en": "Found {count} errors",
   "ja": "エラーが {num} 個見つかりました"

   // ✅ Correct - same placeholder
   "en": "Found {count} errors",
   "ja": "{count} 個のエラーが見つかりました"
   ```

5. **Missing emojis in translations**
   ```json
   // ❌ Wrong - missing emoji in Japanese
   "en": "✅ All checks passed",
   "ja": "すべてのチェックに合格しました"

   // ✅ Correct - emoji in both
   "en": "✅ All checks passed",
   "ja": "✅ すべてのチェックに合格しました"
   ```

## Resources

- **Implementation Progress**: `docs/i18n-implementation-progress.md`
- **i18n System Architecture**: `docs/architecture/i18n.md`
- **Translation File**: `src/i18n/messages.json`
- **Example Commands**: See `src/commands/git/branch.rs`, `src/commands/config/check.rs`

## Getting Help

If you have questions about i18n implementation:

1. Check existing command implementations for patterns
2. Review `docs/i18n-implementation-progress.md` for guidelines
3. Search `src/i18n/messages.json` for similar key examples
4. Ask in project discussions or issues

---

**Migration Complete**: All 33 commands are now fully internationalized. Future commands should follow this guide from the start.
