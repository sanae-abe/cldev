use crate::cli::args::SearchField;
use crate::core::{LearningRecordV3, Result};
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

    // Load V3 records (new format)
    let v3_ids = LearningRecordV3::list_all().unwrap_or_default();

    if v3_ids.is_empty() {
        println!("{}", "\n⚠️  No learning records found".yellow());
        println!("Create your first record with: cldev session start");
        return Ok(());
    }

    // Search V3 records
    let mut matching_records = Vec::new();
    let query_lower = query.to_lowercase();

    for id in v3_ids {
        if let Ok(record) = LearningRecordV3::load(&id) {
            let matches = match field {
                Some(SearchField::Topic) => record.id.to_lowercase().contains(&query_lower),
                Some(SearchField::Tag) => record
                    .tags
                    .iter()
                    .any(|tag| tag.to_lowercase().contains(&query_lower)),
                Some(SearchField::Content) => {
                    record.markdown_body.to_lowercase().contains(&query_lower)
                }
                None => {
                    // Search all fields
                    record.id.to_lowercase().contains(&query_lower)
                        || record
                            .tags
                            .iter()
                            .any(|tag| tag.to_lowercase().contains(&query_lower))
                        || record.markdown_body.to_lowercase().contains(&query_lower)
                }
            };

            if matches {
                matching_records.push(record);
            }
        }
    }

    // Display results
    if matching_records.is_empty() {
        println!("{}", "\n⚠️  No matching records found".yellow());
        return Ok(());
    }

    println!(
        "\n{} Found {} matching record(s)",
        "✅".green(),
        matching_records.len()
    );

    // Limit results
    let display_count = limit.min(matching_records.len());
    println!("{} Displaying top {}", "ℹ️".cyan(), display_count);

    // Display records
    for (i, record) in matching_records.iter().take(limit).enumerate() {
        display_record_brief(record, i + 1);
    }

    if matching_records.len() > limit {
        println!(
            "\n{} {} more record(s) not shown. Increase --limit to see more.",
            "ℹ️".cyan(),
            matching_records.len() - limit
        );
    }

    // Provide next steps
    println!("\n{}", "💡 Next Steps:".yellow().bold());
    println!("  • View details: Check files in ~/.cldev/learning-records/");
    println!("  • Filter by tag: cldev lr find <query> --field tag");
    println!("  • See stats: cldev lr stats");

    Ok(())
}

/// Display brief V3 record information
fn display_record_brief(record: &LearningRecordV3, index: usize) {
    use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Cell, Color, Table};

    println!("\n{} Record #{}", "📄".cyan(), index);

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec!["Field", "Value"]);

    // ID
    table.add_row(vec!["ID", &record.id]);

    // Created
    table.add_row(vec![
        "Created",
        &record.created.format("%Y-%m-%d %H:%M").to_string(),
    ]);

    // Auto-generated
    let auto_cell = if record.auto_generated {
        Cell::new("Yes").fg(Color::Cyan)
    } else {
        Cell::new("No").fg(Color::White)
    };
    table.add_row(vec![Cell::new("Auto-generated"), auto_cell]);

    // Confidence
    if let Some(conf) = record.confidence {
        table.add_row(vec!["Confidence", &format!("{:.1}%", conf * 100.0)]);
    }

    // Tags
    if !record.tags.is_empty() {
        table.add_row(vec!["Tags", &record.tags.join(", ")]);
    }

    // Status
    let status_cell = match record.status {
        crate::core::RecordStatus::Resolved => Cell::new("Resolved").fg(Color::Green),
        crate::core::RecordStatus::InProgress => Cell::new("In Progress").fg(Color::Yellow),
        crate::core::RecordStatus::Pending => Cell::new("Pending").fg(Color::Cyan),
    };
    table.add_row(vec![Cell::new("Status"), status_cell]);

    // Duration
    if let Some(duration) = record.duration_min {
        table.add_row(vec!["Duration", &format!("{} min", duration)]);
    }

    println!("{}", table);

    // Display first few lines of markdown body
    let lines: Vec<&str> = record.markdown_body.lines().take(3).collect();
    if !lines.is_empty() {
        println!("\n  {} Preview:", "👁️".yellow());
        for line in lines {
            let truncated = if line.chars().count() > 70 {
                let t: String = line.chars().take(67).collect();
                format!("{}...", t)
            } else {
                line.to_string()
            };
            println!("    {}", truncated);
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
