# Shell Completions for cldev

This directory contains shell completion scripts for the `cldev` CLI tool.

## Generating Completions

To generate completions for your shell, use the `cldev completions` command:

```bash
# Generate Bash completions
cldev completions bash

# Generate Zsh completions
cldev completions zsh

# Generate Fish completions
cldev completions fish

# Generate PowerShell completions
cldev completions powershell

# Generate with installation instructions
cldev completions bash --install
```

## Installation Instructions

### Bash

**Option 1: Dynamic loading (recommended)**
Add to your `~/.bashrc`:
```bash
eval "$(cldev completions bash)"
```

**Option 2: Save to file**
```bash
cldev completions bash > ~/.local/share/bash-completion/completions/cldev
```

### Zsh

**Option 1: Dynamic loading (recommended)**
Add to your `~/.zshrc`:
```bash
eval "$(cldev completions zsh)"
```

**Option 2: Save to completion directory**
```bash
# Create completion directory if it doesn't exist
mkdir -p ~/.zsh/completion

# Generate completion file
cldev completions zsh > ~/.zsh/completion/_cldev

# Add to ~/.zshrc
fpath=(~/.zsh/completion $fpath)
autoload -Uz compinit && compinit
```

### Fish

Save to Fish completions directory:
```bash
cldev completions fish > ~/.config/fish/completions/cldev.fish
```

### PowerShell

**Option 1: Dynamic loading**
Add to your PowerShell profile:
```powershell
cldev completions powershell | Out-String | Invoke-Expression
```

**Option 2: Save to file**
```powershell
cldev completions powershell > cldev.ps1
# Then add to your profile:
. path\to\cldev.ps1
```

## Pre-generated Completion Files

This directory may contain pre-generated completion files for convenience:

- `cldev.bash` - Bash completion script
- `cldev.zsh` - Zsh completion script
- `cldev.fish` - Fish completion script
- `_cldev.ps1` - PowerShell completion script

These files can be generated using the build script or manually with the commands above.

## Building Completions

To generate all completion files at once, you can use the Rust API:

```rust
use cldev::cli::completions::generate_all_completions;

generate_all_completions().expect("Failed to generate completions");
```

Or create a simple build script that generates them during the build process.
