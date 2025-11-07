# i18n Quick Start Guide

## Installation

The i18n system is already integrated into `cldev`. No additional installation needed.

## Basic Usage

### In Your Code

```rust
use cldev::cli::output::OutputHandler;

fn main() {
    let output = OutputHandler::new(false, false, false);
    
    // Simple localized message
    output.success(&output.t("command-success"));
    
    // Message with single variable
    output.info(&output.t_format("next-step", "command", "cldev config check"));
    
    // Message with multiple variables
    let mut vars = HashMap::new();
    vars.insert("field", "timeout");
    vars.insert("value", "invalid");
    output.error(&output.t_with_vars("invalid-value", &vars));
}
```

### Direct I18n Usage

```rust
use cldev::core::i18n::{I18n, Language};

// Automatic language detection
let i18n = I18n::new();
let msg = i18n.get("command-success");

// Explicit language
let i18n_ja = I18n::with_language(Language::Japanese);
let msg_ja = i18n_ja.get("command-success");

// Variable substitution
let msg = i18n.format("next-step", "command", "cldev config check");
```

## Language Selection

The system automatically detects language from environment:

```bash
# English
LANG=en_US.UTF-8 cldev config check

# Japanese  
LANG=ja_JP.UTF-8 cldev config check
```

## Available Messages

See all available message keys in `/Users/sanae.abe/projects/cldev/src/i18n/messages.json`

Common messages:
- `command-success` / `command-failed`
- `config-check-success` / `config-init-success`
- `next-step` (requires {command} variable)
- `file-not-found` (requires {path} variable)
- `validation-failed` (requires {count} variable)

## Adding New Messages

1. Edit `src/i18n/messages.json`:
```json
{
  "en": {
    "my-message": "Hello {name}"
  },
  "ja": {
    "my-message": "こんにちは {name}"
  }
}
```

2. Use in code:
```rust
output.info(&output.t_format("my-message", "name", "World"));
```

## Demo

Run the complete demo:
```bash
cargo run --example i18n_demo
```

## Documentation

Full documentation: `/Users/sanae.abe/projects/cldev/docs/i18n.md`
