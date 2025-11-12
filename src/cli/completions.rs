#![allow(dead_code)]

use clap::CommandFactory;
use clap_complete::{generate, Shell};
use std::io;

use crate::cli::args::Cli;
use crate::cli::output::OutputHandler;

/// Generate shell completions for the specified shell
///
/// # Arguments
///
/// * `shell` - The target shell (Bash, Zsh, Fish, PowerShell)
///
/// # Examples
///
/// ```
/// use cldev::cli::completions::generate_completions;
/// use cldev::cli::help;
/// use cldev::core::i18n::Language;
/// use clap_complete::Shell;
///
/// // Initialize help system before generating completions
/// help::init_help_i18n(Language::English);
///
/// // Generate Bash completions
/// generate_completions(Shell::Bash);
/// ```
pub fn generate_completions(shell: Shell) {
    let mut cmd = Cli::command();
    let bin_name = cmd.get_name().to_string();

    generate(shell, &mut cmd, bin_name, &mut io::stdout());
}

/// Generate completions for all supported shells to files
///
/// This function generates completion scripts for all supported shells
/// and saves them to the `completions/` directory.
///
/// # Returns
///
/// Returns `Ok(())` if all completions were generated successfully,
/// or an error if any generation failed.
///
/// # Examples
///
/// ```no_run
/// use cldev::cli::completions::generate_all_completions;
/// use cldev::cli::output::OutputHandler;
///
/// let output = OutputHandler::default();
/// generate_all_completions(&output).expect("Failed to generate completions");
/// ```
pub fn generate_all_completions(output: &OutputHandler) -> io::Result<()> {
    use std::fs;
    use std::path::Path;

    let completions_dir = Path::new("completions");

    // Create completions directory if it doesn't exist
    if !completions_dir.exists() {
        fs::create_dir(completions_dir)?;
    }

    let mut cmd = Cli::command();
    let bin_name = cmd.get_name().to_string();

    // Generate for each shell
    let shells = [
        (Shell::Bash, "cldev.bash"),
        (Shell::Zsh, "cldev.zsh"),
        (Shell::Fish, "cldev.fish"),
        (Shell::PowerShell, "_cldev.ps1"),
    ];

    for (shell, filename) in shells.iter() {
        let output_path = completions_dir.join(filename);
        let mut file = fs::File::create(&output_path)?;

        generate(*shell, &mut cmd, bin_name.clone(), &mut file);

        output.success(&format!(
            "Generated {} completion: {}",
            shell_name(*shell),
            output_path.display()
        ));
    }

    Ok(())
}

/// Get a human-readable name for the shell
fn shell_name(shell: Shell) -> &'static str {
    match shell {
        Shell::Bash => "Bash",
        Shell::Zsh => "Zsh",
        Shell::Fish => "Fish",
        Shell::PowerShell => "PowerShell",
        _ => "Unknown",
    }
}

/// Print installation instructions for the generated completions
pub fn print_installation_instructions(shell: Shell, output: &OutputHandler) {
    output.print_newline();
    output.info(&format!(
        "{} Completion Installation Instructions:",
        shell_name(shell)
    ));
    output.println_raw(get_installation_instructions(shell));
}

/// Get installation instructions for a specific shell
fn get_installation_instructions(shell: Shell) -> &'static str {
    match shell {
        Shell::Bash => {
            r#"
Add to your ~/.bashrc:
    eval "$(cldev completions bash)"

Or save to a file:
    cldev completions bash > ~/.local/share/bash-completion/completions/cldev
"#
        }
        Shell::Zsh => {
            r#"
Add to your ~/.zshrc:
    eval "$(cldev completions zsh)"

Or save to a completion directory:
    cldev completions zsh > ~/.zsh/completion/_cldev
    # Then add to ~/.zshrc:
    fpath=(~/.zsh/completion $fpath)
    autoload -Uz compinit && compinit
"#
        }
        Shell::Fish => {
            r#"
Save to Fish completions directory:
    cldev completions fish > ~/.config/fish/completions/cldev.fish
"#
        }
        Shell::PowerShell => {
            r#"
Add to your PowerShell profile:
    cldev completions powershell | Out-String | Invoke-Expression

Or save to a file and source it:
    cldev completions powershell > cldev.ps1
    # Then add to your profile:
    . path\to\cldev.ps1
"#
        }
        _ => "No installation instructions available for this shell.",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_name() {
        assert_eq!(shell_name(Shell::Bash), "Bash");
        assert_eq!(shell_name(Shell::Zsh), "Zsh");
        assert_eq!(shell_name(Shell::Fish), "Fish");
        assert_eq!(shell_name(Shell::PowerShell), "PowerShell");
    }

    #[test]
    fn test_installation_instructions_not_empty() {
        let shells = [Shell::Bash, Shell::Zsh, Shell::Fish, Shell::PowerShell];

        for shell in shells.iter() {
            let instructions = get_installation_instructions(*shell);
            assert!(
                !instructions.is_empty(),
                "Instructions for {:?} should not be empty",
                shell
            );
        }
    }
}
