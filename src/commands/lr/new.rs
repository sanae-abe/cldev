use crate::core::{LearningSession, LearningSessionBuilder, Result};
use colored::Colorize;
use dialoguer::{Input, MultiSelect};
use std::process::Command;

/// Handle new learning record command
pub fn handle_new(topic: String, edit: bool) -> Result<()> {
    println!("{}", "📝 Creating new learning record...".cyan().bold());
    println!("{} Topic: {}", "ℹ️".cyan(), topic.green());

    // Create session with basic info
    let mut builder = LearningSessionBuilder::new("learning", &topic);

    // Collect additional information interactively
    println!("\n{}", "📋 Additional Information (optional)".yellow());

    // Tags
    let tag_options = vec![
        "bug-fix",
        "feature",
        "optimization",
        "refactoring",
        "research",
        "security",
        "testing",
        "documentation",
        "deployment",
        "performance",
    ];

    let tag_selections = MultiSelect::new()
        .with_prompt("Select relevant tags (Space to select, Enter to confirm)")
        .items(&tag_options)
        .interact_opt()?;

    if let Some(selections) = tag_selections {
        let selected_tags: Vec<String> = selections
            .iter()
            .map(|&i| tag_options[i].to_string())
            .collect();
        builder = builder.tags(selected_tags);
    }

    // Description/Context
    let context: String = Input::new()
        .with_prompt("Brief context or problem description (optional)")
        .allow_empty(true)
        .interact_text()?;

    if !context.is_empty() {
        builder = builder.metadata("context", context);
    }

    // Build and save the session
    let (session, path) = builder.save()?;

    println!(
        "\n{} Learning record created successfully!",
        "✅".green().bold()
    );
    println!("{} Session ID: {}", "ℹ️".cyan(), session.id.yellow());
    println!(
        "{} Saved to: {}",
        "ℹ️".cyan(),
        path.display().to_string().cyan()
    );

    // Display session summary
    display_session_summary(&session);

    // Open editor if requested
    if edit {
        open_session_in_editor(&path.to_string_lossy())?;
    }

    // Provide next steps
    println!("\n{}", "💡 Next Steps:".yellow().bold());
    println!("  • Add learnings: Edit the session file directly");
    println!("  • Mark resolved: Update 'resolved' field to true");
    println!("  • Add files: List affected files in 'files_affected'");
    println!("  • View all: cldev lr find {}", topic);

    Ok(())
}

/// Display session summary
fn display_session_summary(session: &LearningSession) {
    use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Table};

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec!["Field", "Value"]);

    table.add_row(vec!["ID", &session.id]);
    table.add_row(vec!["Type", &session.session_type]);
    table.add_row(vec!["Timestamp", &session.timestamp]);
    table.add_row(vec!["Description", &session.description]);

    if !session.tags.is_empty() {
        table.add_row(vec!["Tags", &session.tags.join(", ")]);
    }

    println!("\n{}", table);
}

/// Open session file in editor
fn open_session_in_editor(path: &str) -> Result<()> {
    println!("\n{} Opening in editor...", "📝".cyan());

    // Try different editors in order of preference
    let editors = vec![
        std::env::var("EDITOR").ok(),
        Some("code".to_string()),  // VS Code
        Some("vim".to_string()),   // Vim
        Some("nano".to_string()),  // Nano
        Some("emacs".to_string()), // Emacs
    ];

    for editor in editors.into_iter().flatten() {
        let result = Command::new(&editor).arg(path).status();

        if let Ok(status) = result {
            if status.success() {
                return Ok(());
            }
        }
    }

    println!(
        "{} Could not open editor. Edit manually: {}",
        "⚠️".yellow(),
        path
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_creation() {
        let session = LearningSessionBuilder::new("learning", "Test topic")
            .tag("test")
            .build();

        assert_eq!(session.description, "Test topic");
        assert_eq!(session.tags.len(), 1);
    }
}
