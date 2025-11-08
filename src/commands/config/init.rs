//! Interactive configuration initialization
//!
//! This module provides an interactive setup wizard for first-time configuration
//! of cldev, including language selection, directory detection, and shell setup.

use crate::cli::output::OutputHandler;
use crate::core::config::{Config, GeneralConfig, GitConfig, UiConfig};
use crate::core::error::{CldevError, Result};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use indicatif::{ProgressBar, ProgressStyle};
use std::path::{Path, PathBuf};
use std::process::Command;

/// Language option for the interactive setup
#[derive(Debug, Clone)]
struct LanguageOption {
    code: &'static str,
    display: &'static str,
}

impl std::fmt::Display for LanguageOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display)
    }
}

const LANGUAGES: &[LanguageOption] = &[
    LanguageOption {
        code: "en",
        display: "English",
    },
    LanguageOption {
        code: "ja",
        display: "日本語 (Japanese)",
    },
];

/// Run interactive configuration initialization
pub fn run_interactive_init(force: bool, output: &mut OutputHandler) -> Result<()> {
    let config_path = Config::default_path()?;

    // Check if config already exists
    if config_path.exists() && !force {
        output.warning(&format!(
            "Configuration file already exists: {}",
            config_path.display()
        ));
        output.info("Use --force to overwrite existing configuration");
        return Ok(());
    }

    // Print header
    print_header(output);

    // Create theme for dialoguer
    let theme = ColorfulTheme::default();

    // Step 1: Language selection
    let language = select_language(&theme, output)?;

    // Apply selected language to output handler
    use crate::core::i18n::Language;
    if let Some(lang) = Language::from_code(&language) {
        output.set_language(lang);
    }

    // Step 2: Claude Code directory detection
    let claude_dir = detect_claude_directory(&theme, output)?;

    // Step 3: Project root selection
    let projects_dir = select_projects_directory(&theme, output)?;

    // Step 4: Git CLI detection
    let (github_cli, gitlab_cli) = detect_git_cli(output);

    // Step 5: Shell completion
    let shell_config = detect_shell_and_offer_completion(&theme, output)?;

    // Step 6: Aliases
    let add_aliases = offer_aliases(&theme, output)?;

    // Step 7: Claude Code integration
    let integrate_claude = offer_claude_integration(&theme, &claude_dir, output)?;

    // Generate configuration with progress bar
    let config = generate_config_with_progress(
        language,
        claude_dir.clone(),
        projects_dir,
        github_cli,
        gitlab_cli,
        output,
    )?;

    // Save configuration
    config.save(Some(config_path.clone()))?;

    // Post-setup actions
    if let Some(ref shell_path) = shell_config {
        add_shell_completion(shell_path, output)?;
    }

    if add_aliases {
        suggest_alias_commands(&shell_config, output);
    }

    if integrate_claude {
        setup_claude_integration(&claude_dir, output)?;
    }

    // Success message
    output.success(&output.i18n().format(
        "config-init-success-saved",
        "path",
        &config_path.display().to_string(),
    ));

    // Next steps
    print_next_steps(output);

    Ok(())
}

/// Print ASCII art header
fn print_header(output: &OutputHandler) {
    output.info(&output.i18n().get("config-init-header"));
    output.info(&output.i18n().get("config-init-separator"));
    output.info("");
}

/// Step 1: Language selection
fn select_language(theme: &ColorfulTheme, output: &OutputHandler) -> Result<String> {
    output.info(&output.i18n().get("config-init-step1-language"));

    let selection = Select::with_theme(theme)
        .with_prompt(output.i18n().get("config-init-select-language"))
        .items(LANGUAGES)
        .default(1) // Default to Japanese
        .interact()
        .map_err(|e| CldevError::io(format!("Language selection failed: {}", e)))?;

    let selected = LANGUAGES[selection].code.to_string();
    output.info(&output.i18n().format(
        "config-init-selected",
        "language",
        LANGUAGES[selection].display,
    ));
    output.info("");

    Ok(selected)
}

/// Step 2: Detect Claude Code directory
fn detect_claude_directory(theme: &ColorfulTheme, output: &OutputHandler) -> Result<PathBuf> {
    output.info(&output.i18n().get("config-init-step2-claude-dir"));

    let home_dir =
        dirs::home_dir().ok_or_else(|| CldevError::config("Cannot determine home directory"))?;
    let default_claude_dir = home_dir.join(".claude");

    if default_claude_dir.exists() {
        output.info(&output.i18n().format(
            "config-init-claude-detected",
            "path",
            &default_claude_dir.display().to_string(),
        ));
        output.info("");
        Ok(default_claude_dir)
    } else {
        output.warning(&output.i18n().get("config-init-claude-not-found"));

        let custom_path: String = Input::with_theme(theme)
            .with_prompt(output.i18n().get("config-init-claude-prompt"))
            .default(default_claude_dir.display().to_string())
            .interact_text()
            .map_err(|e| CldevError::io(format!("Input failed: {}", e)))?;

        let path = PathBuf::from(custom_path);
        output.info(&output.i18n().format(
            "config-init-claude-using",
            "path",
            &path.display().to_string(),
        ));
        output.info("");

        Ok(path)
    }
}

/// Step 3: Select projects directory
fn select_projects_directory(theme: &ColorfulTheme, output: &OutputHandler) -> Result<PathBuf> {
    output.info(&output.i18n().get("config-init-step3-projects-dir"));

    let home_dir =
        dirs::home_dir().ok_or_else(|| CldevError::config("Cannot determine home directory"))?;
    let default_projects_dir = home_dir.join("projects");

    let custom_path: String = Input::with_theme(theme)
        .with_prompt(output.i18n().get("config-init-projects-prompt"))
        .default(default_projects_dir.display().to_string())
        .interact_text()
        .map_err(|e| CldevError::io(format!("Input failed: {}", e)))?;

    let path = PathBuf::from(custom_path);
    output.info(&output.i18n().format(
        "config-init-claude-using",
        "path",
        &path.display().to_string(),
    ));
    output.info("");

    Ok(path)
}

/// Step 4: Detect Git CLI tools
fn detect_git_cli(output: &OutputHandler) -> (bool, bool) {
    output.info(&output.i18n().get("config-init-step4-git-cli"));

    let github_cli = Command::new("gh")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    let gitlab_cli = Command::new("glab")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if github_cli {
        output.info(&output.i18n().get("config-init-gh-detected"));
    } else {
        output.warning(&output.i18n().get("config-init-gh-not-found"));
    }

    if gitlab_cli {
        output.info(&output.i18n().get("config-init-glab-detected"));
    } else {
        output.warning(&output.i18n().get("config-init-glab-not-found"));
    }

    output.info("");

    (github_cli, gitlab_cli)
}

/// Step 5: Detect shell and offer completion setup
fn detect_shell_and_offer_completion(
    theme: &ColorfulTheme,
    output: &OutputHandler,
) -> Result<Option<PathBuf>> {
    output.info(&output.i18n().get("config-init-step5-shell"));

    // Detect current shell
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "unknown".to_string());
    let shell_name = shell.split('/').next_back().unwrap_or("unknown");

    output.info(
        &output
            .i18n()
            .format("config-init-shell-detected", "shell", shell_name),
    );

    // Determine shell config file
    let config_file = match shell_name {
        "zsh" => dirs::home_dir().map(|h| h.join(".zshrc")),
        "bash" => dirs::home_dir().map(|h| h.join(".bashrc")),
        "fish" => dirs::home_dir().map(|h| h.join(".config/fish/config.fish")),
        _ => None,
    };

    if let Some(ref config_path) = config_file {
        output.info(&output.i18n().format(
            "config-init-shell-config",
            "path",
            &config_path.display().to_string(),
        ));

        let add_completion = Confirm::with_theme(theme)
            .with_prompt(output.i18n().get("config-init-shell-prompt"))
            .default(true)
            .interact()
            .map_err(|e| CldevError::io(format!("Confirmation failed: {}", e)))?;

        output.info("");

        if add_completion {
            return Ok(config_file);
        }
    } else {
        output.warning(
            &output
                .i18n()
                .format("config-init-shell-unsupported", "shell", shell_name),
        );
        output.info("");
    }

    Ok(None)
}

/// Step 6: Offer to add aliases
fn offer_aliases(theme: &ColorfulTheme, output: &OutputHandler) -> Result<bool> {
    output.info(&output.i18n().get("config-init-step6-aliases"));
    output.info(&output.i18n().get("config-init-aliases-suggested"));
    output.info(&output.i18n().get("config-init-aliases-c"));
    output.info(&output.i18n().get("config-init-aliases-cconfig"));
    output.info(&output.i18n().get("config-init-aliases-cdev"));

    let add_aliases = Confirm::with_theme(theme)
        .with_prompt(output.i18n().get("config-init-aliases-prompt"))
        .default(true)
        .interact()
        .map_err(|e| CldevError::io(format!("Confirmation failed: {}", e)))?;

    output.info("");

    Ok(add_aliases)
}

/// Step 7: Offer Claude Code integration
fn offer_claude_integration(
    theme: &ColorfulTheme,
    claude_dir: &Path,
    output: &OutputHandler,
) -> Result<bool> {
    output.info(&output.i18n().get("config-init-step7-claude-integration"));

    let claude_md = claude_dir.join("CLAUDE.md");

    if !claude_md.exists() {
        output.warning(&output.i18n().get("config-init-claude-md-not-found"));
        output.info(&output.i18n().get("config-init-claude-manual-setup"));
        output.info("");
        return Ok(false);
    }

    // Check if already integrated
    if let Ok(content) = std::fs::read_to_string(&claude_md) {
        if content.contains("cldev lr find") {
            output.success(&output.i18n().get("config-init-claude-already-integrated"));
            output.info("");
            return Ok(false);
        }
    }

    output.info(&output.i18n().get("config-init-claude-integration-info"));
    output.info(&output.i18n().get("config-init-claude-integration-benefit"));
    output.info("");

    let integrate = Confirm::with_theme(theme)
        .with_prompt(output.i18n().get("config-init-claude-integration-prompt"))
        .default(true)
        .interact()
        .map_err(|e| CldevError::io(format!("Confirmation failed: {}", e)))?;

    output.info("");

    Ok(integrate)
}

/// Setup Claude Code integration
fn setup_claude_integration(claude_dir: &Path, output: &OutputHandler) -> Result<()> {
    use std::fs::OpenOptions;
    use std::io::Write;

    let claude_md = claude_dir.join("CLAUDE.md");

    if !claude_md.exists() {
        output.warning(&output.i18n().get("config-init-claude-md-not-found"));
        return Ok(());
    }

    // Check if already integrated
    if let Ok(content) = std::fs::read_to_string(&claude_md) {
        if content.contains("cldev lr find") {
            output.info(&output.i18n().get("config-init-claude-already-integrated"));
            return Ok(());
        }
    }

    // Integration text
    let integration_text = r#"

---

## 📖 学習記録活用（cldev統合）

### 過去の問題・解決策検索
```bash
cldev lr find "認証"              # キーワード検索
cldev lr find "JWT" --field tag   # タグ検索
cldev lr stats                    # 統計表示
cldev lr problems                 # 未解決問題一覧
```

### 学習記録の場所
`~/.claude/learning-sessions/*.md`

### 自動参照推奨タイミング
- `/urgent`, `/fix`, `/debug` 実行時
- エラー調査時（過去の類似問題確認）
- 技術的決定の背景確認

### 記録フォーマット
各学習記録は以下を含む：
- 問題の説明
- 根本原因
- 解決策
- 重要な学び
- 関連ファイル

---
"#;

    // Append to CLAUDE.md
    let mut file = OpenOptions::new()
        .append(true)
        .open(&claude_md)
        .map_err(|e| CldevError::io(format!("Failed to open CLAUDE.md: {}", e)))?;

    file.write_all(integration_text.as_bytes())
        .map_err(|e| CldevError::io(format!("Failed to write to CLAUDE.md: {}", e)))?;

    output.success(&output.i18n().get("config-init-claude-integration-added"));
    output.info(&output.i18n().format(
        "config-init-claude-integration-review",
        "path",
        &claude_md.display().to_string(),
    ));

    Ok(())
}

/// Generate configuration with progress bar
fn generate_config_with_progress(
    language: String,
    claude_dir: PathBuf,
    projects_dir: PathBuf,
    github_cli: bool,
    gitlab_cli: bool,
    output: &OutputHandler,
) -> Result<Config> {
    output.info(&output.i18n().get("config-init-generating"));

    let pb = ProgressBar::new(5);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .expect("Invalid progress bar template")
            .progress_chars("#>-"),
    );

    let msg_init = output.i18n().get("config-init-progress-initializing");
    pb.set_message(msg_init);
    std::thread::sleep(std::time::Duration::from_millis(200));
    pb.inc(1);

    let msg_general = output.i18n().get("config-init-progress-general");
    pb.set_message(msg_general);
    let general = GeneralConfig {
        language: language.clone(),
        claude_dir: claude_dir.clone(),
        projects_dir: projects_dir.clone(),
        tech_stack: None,
        project_name: None,
    };
    std::thread::sleep(std::time::Duration::from_millis(200));
    pb.inc(1);

    let msg_git = output.i18n().get("config-init-progress-git");
    pb.set_message(msg_git);
    let git = GitConfig {
        github_cli,
        gitlab_cli,
        default_base_branch: "main".to_string(),
        auto_push: true,
    };
    std::thread::sleep(std::time::Duration::from_millis(200));
    pb.inc(1);

    let msg_ui = output.i18n().get("config-init-progress-ui");
    pb.set_message(msg_ui);
    let ui = UiConfig {
        color: true,
        emoji: true,
        progress_bar: true,
    };
    std::thread::sleep(std::time::Duration::from_millis(200));
    pb.inc(1);

    let msg_finalizing = output.i18n().get("config-init-progress-finalizing");
    pb.set_message(msg_finalizing);
    let config = Config {
        general,
        git,
        ui,
        ..Default::default()
    };
    std::thread::sleep(std::time::Duration::from_millis(200));
    pb.inc(1);

    let msg_complete = output.i18n().get("config-init-progress-complete");
    pb.finish_with_message(msg_complete);
    output.info("");

    Ok(config)
}

/// Add shell completion to config file
fn add_shell_completion(shell_config_path: &PathBuf, output: &OutputHandler) -> Result<()> {
    use std::io::Write;

    let completion_line = "\n# cldev shell completion\neval \"$(cldev completion)\"";

    // Read existing content
    let existing = std::fs::read_to_string(shell_config_path).unwrap_or_default();

    // Check if already added
    if existing.contains("cldev completion") {
        output.info(&output.i18n().get("config-init-shell-completion-exists"));
        return Ok(());
    }

    // Append completion line
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open(shell_config_path)
        .map_err(|e| {
            CldevError::io(format!(
                "Failed to open shell config {}: {}",
                shell_config_path.display(),
                e
            ))
        })?;

    file.write_all(completion_line.as_bytes()).map_err(|e| {
        CldevError::io(format!(
            "Failed to write to shell config {}: {}",
            shell_config_path.display(),
            e
        ))
    })?;

    output.success(&output.i18n().format(
        "config-init-shell-completion-added",
        "path",
        &shell_config_path.display().to_string(),
    ));

    Ok(())
}

/// Suggest alias commands
fn suggest_alias_commands(shell_config: &Option<PathBuf>, output: &OutputHandler) {
    output.info(&output.i18n().get("config-init-aliases-instructions"));

    if let Some(config_path) = shell_config {
        output.info(&format!(
            "   echo \"alias c='cldev'\" >> {}",
            config_path.display()
        ));
        output.info(&format!(
            "   echo \"alias cconfig='cldev config'\" >> {}",
            config_path.display()
        ));
        output.info(&format!(
            "   echo \"alias cdev='cldev dev'\" >> {}",
            config_path.display()
        ));
    } else {
        output.info("   alias c='cldev'");
        output.info("   alias cconfig='cldev config'");
        output.info("   alias cdev='cldev dev'");
    }
}

/// Print next steps after setup
fn print_next_steps(output: &OutputHandler) {
    output.info(&output.i18n().get("config-init-next-steps"));
    output.list_item(&output.i18n().get("config-init-next-reload-shell"));
    output.list_item(&output.i18n().get("config-init-next-edit"));
    output.list_item(&output.i18n().get("config-init-next-check"));
    output.list_item(&output.i18n().get("config-init-next-list"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_display() {
        assert_eq!(LANGUAGES[0].to_string(), "English");
        assert_eq!(LANGUAGES[1].to_string(), "日本語 (Japanese)");
    }

    #[test]
    fn test_detect_git_cli() {
        let output = OutputHandler::default();
        let (gh, glab) = detect_git_cli(&output);

        // At least one should be available in development environment
        // This test just ensures the function runs without panicking
        let _ = (gh, glab); // Use variables to avoid unused warning
    }
}
