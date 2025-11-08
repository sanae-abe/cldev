use crate::cli::args::SearchField;
use crate::core::{LearningSession, Result};
use colored::Colorize;

/// Handle find learning records command
pub fn handle_find(query: String, field: Option<SearchField>, limit: usize) -> Result<()> {
    println!("{}", "🔍 Searching learning records...".cyan().bold());
    println!("{} Query: {}", "ℹ️".cyan(), query.green());

    if let Some(ref search_field) = field {
        println!(
            "{} Search field: {}",
            "ℹ️".cyan(),
            format!("{:?}", search_field).yellow()
        );
    }

    // Load all sessions
    let session_ids = LearningSession::list_all()?;

    if session_ids.is_empty() {
        println!("{}", "\n⚠️  No learning records found".yellow());
        println!("Create your first record with: cldev lr new <topic>");
        return Ok(());
    }

    // Search sessions
    let mut matching_sessions = Vec::new();
    let query_lower = query.to_lowercase();

    for id in session_ids {
        if let Ok(session) = LearningSession::load(&id) {
            let matches = match field {
                Some(SearchField::Topic) => {
                    session.description.to_lowercase().contains(&query_lower)
                }
                Some(SearchField::Tag) => session
                    .tags
                    .iter()
                    .any(|tag| tag.to_lowercase().contains(&query_lower)),
                Some(SearchField::Content) => {
                    session.description.to_lowercase().contains(&query_lower)
                        || session
                            .solution
                            .as_ref()
                            .map(|s| s.to_lowercase().contains(&query_lower))
                            .unwrap_or(false)
                        || session
                            .root_cause
                            .as_ref()
                            .map(|s| s.to_lowercase().contains(&query_lower))
                            .unwrap_or(false)
                }
                None => {
                    // Search all fields
                    session.description.to_lowercase().contains(&query_lower)
                        || session
                            .tags
                            .iter()
                            .any(|tag| tag.to_lowercase().contains(&query_lower))
                        || session
                            .solution
                            .as_ref()
                            .map(|s| s.to_lowercase().contains(&query_lower))
                            .unwrap_or(false)
                }
            };

            if matches {
                matching_sessions.push(session);
            }
        }
    }

    // Display results
    if matching_sessions.is_empty() {
        println!("{}", "\n⚠️  No matching records found".yellow());
        return Ok(());
    }

    println!(
        "\n{} Found {} matching record(s)",
        "✅".green(),
        matching_sessions.len()
    );

    // Limit results
    let display_count = limit.min(matching_sessions.len());
    println!("{} Displaying top {}", "ℹ️".cyan(), display_count);

    // Display sessions
    for (i, session) in matching_sessions.iter().take(limit).enumerate() {
        display_session_brief(session, i + 1);
    }

    if matching_sessions.len() > limit {
        println!(
            "\n{} {} more record(s) not shown. Increase --limit to see more.",
            "ℹ️".cyan(),
            matching_sessions.len() - limit
        );
    }

    // Provide next steps
    println!("\n{}", "💡 Next Steps:".yellow().bold());
    println!("  • View details: Check session files in ~/.claude/learning-sessions/");
    println!("  • Filter by tag: cldev lr find <query> --field tag");
    println!("  • See stats: cldev lr stats");

    Ok(())
}

/// Display brief session information
fn display_session_brief(session: &LearningSession, index: usize) {
    use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Cell, Color, Table};

    println!("\n{} Record #{}", "📄".cyan(), index);

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec!["Field", "Value"]);

    // ID
    table.add_row(vec!["ID", &session.id]);

    // Type
    table.add_row(vec!["Type", &session.session_type]);

    // Timestamp
    table.add_row(vec!["Timestamp", &session.timestamp]);

    // Description (safe UTF-8 truncation)
    let truncated_desc = if session.description.chars().count() > 60 {
        let truncated: String = session.description.chars().take(57).collect();
        format!("{}...", truncated)
    } else {
        session.description.clone()
    };
    table.add_row(vec!["Description", &truncated_desc]);

    // Tags
    if !session.tags.is_empty() {
        table.add_row(vec!["Tags", &session.tags.join(", ")]);
    }

    // Status
    let status_cell = if session.resolved {
        Cell::new("Resolved").fg(Color::Green)
    } else {
        Cell::new("Unresolved").fg(Color::Yellow)
    };
    table.add_row(vec![Cell::new("Status"), status_cell]);

    // Duration
    if let Some(duration) = session.duration_minutes {
        table.add_row(vec!["Duration", &format!("{} min", duration)]);
    }

    println!("{}", table);

    // Display key learnings if available
    if !session.learnings.is_empty() {
        println!("\n  {} Key Learnings:", "💡".yellow());
        for (i, learning) in session.learnings.iter().take(3).enumerate() {
            println!("    {}. {}", i + 1, learning);
        }
        if session.learnings.len() > 3 {
            println!("    ... and {} more", session.learnings.len() - 3);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_query_matching() {
        let query = "test".to_lowercase();
        assert!("testing".to_lowercase().contains(&query));
        assert!(!"example".to_lowercase().contains(&query));
    }

    #[test]
    fn test_safe_utf8_truncation() {
        // Test ASCII string
        let ascii = "This is a short description";
        assert_eq!(ascii.len(), 27);
        assert!(ascii.chars().count() < 60);

        // Test Japanese string (multi-byte UTF-8 characters)
        // Creating a string longer than 60 characters
        let japanese = "これは非常に長い説明文です。マルチバイト文字を含むテストケースとして使用します。60文字を超える場合は切り捨てられます。さらに追加のテキストを含めて確実に60文字を超えるようにします。";
        assert!(
            japanese.chars().count() > 60,
            "Japanese string should be > 60 chars, got {}",
            japanese.chars().count()
        );

        // Verify safe truncation doesn't panic
        let truncated: String = japanese.chars().take(57).collect();
        assert_eq!(truncated.chars().count(), 57);
        assert!(truncated.len() <= japanese.len());

        // Verify original code would panic at char boundary
        // This would fail: &japanese[..57] - panics at non-char boundary
        // Our fix uses chars().take(57) which is always safe
    }

    #[test]
    fn test_description_truncation_various_lengths() {
        // Create owned strings to avoid temporary lifetime issues
        let long_b = "B".repeat(60);
        let long_a = "A".repeat(70);
        let long_j = "日本語".repeat(30);

        let test_cases = vec![
            ("Short", false),
            ("This is exactly 50 chars long with some padding!", false),
            (long_b.as_str(), false),
            (long_a.as_str(), true), // Should truncate
            (long_j.as_str(), true), // 90 chars, should truncate
        ];

        for (desc, should_truncate) in test_cases {
            let truncated = if desc.chars().count() > 60 {
                let t: String = desc.chars().take(57).collect();
                format!("{}...", t)
            } else {
                desc.to_string()
            };

            if should_truncate {
                assert!(truncated.ends_with("..."));
                assert!(truncated.chars().count() <= 60);
            } else {
                assert!(!truncated.ends_with("..."));
            }
        }
    }
}
