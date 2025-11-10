use crate::core::{LearningRecordV3, RecordStatus, Result};
use colored::Colorize;
use dialoguer::MultiSelect;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

/// Handle new learning record command
pub fn handle_new(topic: String, edit: bool) -> Result<()> {
    println!("{}", "📝 Creating new learning record...".cyan().bold());
    println!("{} Topic: {}", "ℹ️".cyan(), topic.green());

    // Generate unique ID
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let id = format!("{}-{}", sanitize_topic(&topic), timestamp);

    // Collect tags interactively
    println!("\n{}", "📋 Additional Information (optional)".yellow());

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

    let tags: Vec<String> = if let Some(selections) = tag_selections {
        selections
            .iter()
            .map(|&i| tag_options[i].to_string())
            .collect()
    } else {
        Vec::new()
    };

    // Create minimal markdown template
    let markdown_body = create_template(&topic);

    // Create V3 record
    let mut record = LearningRecordV3::new(id.clone(), markdown_body);
    record.tags = tags;
    record.status = RecordStatus::Pending;

    // Save to file
    let path = save_record(&record)?;

    println!(
        "\n{} Learning record created successfully!",
        "✅".green().bold()
    );
    println!("{} Record ID: {}", "ℹ️".cyan(), record.id.yellow());
    println!(
        "{} Saved to: {}",
        "ℹ️".cyan(),
        path.display().to_string().cyan()
    );

    // Display summary
    display_record_summary(&record);

    // Open editor if requested
    if edit {
        open_record_in_editor(&path.to_string_lossy())?;
    }

    // Provide next steps
    println!("\n{}", "💡 Next Steps:".yellow().bold());
    println!("  • Edit the markdown: Fill in Problem, Solution, and Key Takeaways");
    println!("  • Mark in progress: Change status to 'in_progress' in frontmatter");
    println!("  • Mark resolved: Change status to 'resolved' when completed");
    println!("  • Find similar: cldev lr find {}", topic);

    Ok(())
}

/// Create markdown template for the record
fn create_template(topic: &str) -> String {
    format!(
        r#"# {}

## Problem

[Describe the issue or learning opportunity]

## Solution

[Document what you learned]

## Key Takeaways

-
"#,
        topic
    )
}

/// Sanitize topic for use in filename
fn sanitize_topic(topic: &str) -> String {
    topic
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                c.to_ascii_lowercase()
            } else if c.is_whitespace() {
                '-'
            } else {
                '_'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .trim_matches('_')
        .chars()
        .take(50) // Limit length
        .collect()
}

/// Save V3 record to file
fn save_record(record: &LearningRecordV3) -> Result<PathBuf> {
    use crate::core::CldevError;

    // Get learning records directory
    let home =
        dirs::home_dir().ok_or_else(|| CldevError::config("Failed to get home directory"))?;

    let records_dir = home.join(".cldev").join("learning-records");
    if !records_dir.exists() {
        fs::create_dir_all(&records_dir)?;
    }

    // Save as markdown file with V3 frontmatter
    let path = records_dir.join(format!("{}.md", record.id));
    let content = record.to_markdown_file();
    fs::write(&path, content)?;

    Ok(path)
}

/// Display record summary
fn display_record_summary(record: &LearningRecordV3) {
    use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Table};

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec!["Field", "Value"]);

    table.add_row(vec!["ID", &record.id]);
    table.add_row(vec![
        "Created",
        &record.created.format("%Y-%m-%d %H:%M").to_string(),
    ]);
    table.add_row(vec![
        "Status",
        &format!("{:?}", record.status).to_lowercase(),
    ]);

    if !record.tags.is_empty() {
        table.add_row(vec!["Tags", &record.tags.join(", ")]);
    }

    println!("\n{}", table);
}

/// Open record file in editor
fn open_record_in_editor(path: &str) -> Result<()> {
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
    fn test_sanitize_topic() {
        assert_eq!(
            sanitize_topic("Fix TypeScript Error"),
            "fix-typescript-error"
        );
        assert_eq!(sanitize_topic("API/Client Bug"), "api_client-bug");
        assert_eq!(sanitize_topic("   spaces   "), "spaces");
        assert_eq!(
            sanitize_topic(
                "Very Long Topic Name That Should Be Truncated Because It Exceeds Maximum Length"
            ),
            "very-long-topic-name-that-should-be-truncated-beca"
        );
    }

    #[test]
    fn test_create_template() {
        let template = create_template("Test Topic");
        assert!(template.contains("# Test Topic"));
        assert!(template.contains("## Problem"));
        assert!(template.contains("## Solution"));
        assert!(template.contains("## Key Takeaways"));
    }

    #[test]
    fn test_record_creation() {
        let id = "test-record-123".to_string();
        let body = create_template("Test");
        let record = LearningRecordV3::new(id.clone(), body);

        assert_eq!(record.id, id);
        assert_eq!(record.status, RecordStatus::Pending);
        assert!(!record.auto_generated);
        assert!(record.markdown_body.contains("# Test"));
    }
}
