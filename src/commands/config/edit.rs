//! Configuration file editor
//!
//! This module provides functionality to open configuration files in the user's
//! preferred editor with support for:
//! - $EDITOR environment variable
//! - --editor command-line option
//! - Config file editor preference
//! - Common editor fallbacks

#![allow(dead_code)]

use crate::cli::output::OutputHandler;
use crate::core::config::Config;
use crate::core::error::{CldevError, Result};
use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

/// Editor selection priority:
/// 1. --editor command-line option
/// 2. Config file editor.command setting
/// 3. $EDITOR environment variable
/// 4. Common editor fallbacks (vim, nano, vi)
const FALLBACK_EDITORS: &[&str] = &["vim", "nano", "vi"];

/// Edit configuration file
///
/// # Arguments
///
/// * `editor_override` - Optional editor command to use (from --editor flag)
/// * `config_path` - Optional path to config file (uses default if None)
/// * `output` - Output handler for formatted messages
///
/// # Returns
///
/// Returns Ok(()) if editor was launched successfully
pub fn edit_config(
    editor_override: Option<String>,
    config_path: Option<PathBuf>,
    output: &OutputHandler,
) -> Result<()> {
    // Determine config file path
    let config_file = config_path.unwrap_or_else(|| {
        Config::default_path().unwrap_or_else(|_| PathBuf::from("~/.config/cldev/config.toml"))
    });

    // Ensure config file exists
    if !config_file.exists() {
        output.warning(&output.i18n().format(
            "config-edit-not-found",
            "path",
            &config_file.display().to_string(),
        ));
        output.info(&output.i18n().get("config-edit-creating"));

        let default_config = Config::default();
        default_config.save(Some(config_file.clone()))?;

        output.success(&output.i18n().format(
            "config-edit-created",
            "path",
            &config_file.display().to_string(),
        ));
    }

    // Determine which editor to use
    let editor_cmd = determine_editor(editor_override, &config_file, output)?;

    output.info(
        &output
            .i18n()
            .format(
                "config-edit-opening-with",
                "path",
                &config_file.display().to_string(),
            )
            .replace("{editor}", &editor_cmd),
    );

    // Launch editor
    launch_editor(&editor_cmd, &config_file, output)?;

    output.success(&output.i18n().get("config-edit-completed"));

    Ok(())
}

/// Determine which editor to use based on priority
fn determine_editor(
    editor_override: Option<String>,
    config_path: &Path,
    output: &OutputHandler,
) -> Result<String> {
    // Priority 1: --editor command-line option
    if let Some(editor) = editor_override {
        output.debug(&format!("Using editor from --editor flag: {}", editor));
        return Ok(editor);
    }

    // Priority 2: Config file editor.command setting
    if config_path.exists() {
        if let Ok(_config) = Config::load(Some(config_path.to_path_buf())) {
            // Note: We don't have editor.command in current Config structure
            // This is a placeholder for future implementation
            output.debug("No editor configured in config file");
        }
    }

    // Priority 3: $EDITOR environment variable
    if let Ok(editor) = env::var("EDITOR") {
        output.debug(&format!("Using editor from $EDITOR: {}", editor));
        return Ok(editor);
    }

    // Priority 4: Try common editor fallbacks
    for &editor in FALLBACK_EDITORS {
        if is_editor_available(editor) {
            output.debug(&format!("Using fallback editor: {}", editor));
            return Ok(editor.to_string());
        }
    }

    // No editor found
    Err(CldevError::editor(
        output.i18n().get("config-edit-no-editor"),
    ))
}

/// Check if an editor command is available in PATH
fn is_editor_available(editor: &str) -> bool {
    Command::new("which")
        .arg(editor)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Launch the editor with the config file
fn launch_editor(editor_cmd: &str, config_path: &PathBuf, output: &OutputHandler) -> Result<()> {
    // Parse editor command and arguments
    let parts: Vec<&str> = editor_cmd.split_whitespace().collect();
    if parts.is_empty() {
        return Err(CldevError::editor("Empty editor command"));
    }

    let (editor, args) = (parts[0], &parts[1..]);

    // Build command
    let mut cmd = Command::new(editor);
    cmd.args(args);
    cmd.arg(config_path);

    output.debug(&format!(
        "Executing: {} {:?} {}",
        editor,
        args,
        config_path.display()
    ));

    // Execute editor
    let status = cmd
        .status()
        .map_err(|e| CldevError::editor(format!("Failed to launch editor '{}': {}", editor, e)))?;

    if !status.success() {
        return Err(CldevError::editor(format!(
            "Editor exited with non-zero status: {}",
            status.code().unwrap_or(-1)
        )));
    }

    Ok(())
}

/// Get editor suggestions for user
pub fn suggest_editors() -> Vec<&'static str> {
    let mut available = Vec::new();

    for &editor in FALLBACK_EDITORS {
        if is_editor_available(editor) {
            available.push(editor);
        }
    }

    // Add common GUI editors
    for &editor in &["code", "subl", "atom", "gedit"] {
        if is_editor_available(editor) {
            available.push(editor);
        }
    }

    available
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fallback_editors_list() {
        assert!(!FALLBACK_EDITORS.is_empty());
        assert!(FALLBACK_EDITORS.contains(&"vim"));
    }

    #[test]
    fn test_editor_availability() {
        // Test with a command that should exist on most systems
        let _result = is_editor_available("ls");
        // Note: This might fail on some systems, but demonstrates the function
        // Just ensure it doesn't panic - no assertion needed
    }

    #[test]
    fn test_suggest_editors() {
        let _suggestions = suggest_editors();
        // Should return a list (might be empty on some systems)
        // Just ensure it doesn't panic - no assertion needed
    }

    #[test]
    fn test_determine_editor_with_override() {
        use crate::cli::output::OutputHandler;
        use std::path::PathBuf;

        let output = OutputHandler::new(false, false, false);
        let config_path = PathBuf::from("/tmp/test.toml");

        let result = determine_editor(Some("custom-editor".to_string()), &config_path, &output);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "custom-editor");
    }
}
