use crate::cli::args::Priority;
use crate::core::{LearningSession, Result};
use colored::Colorize;
use std::collections::HashMap;

/// Handle problems analysis command
pub fn handle_problems(priority: Option<Priority>, recent: bool) -> Result<()> {
    println!("{}", "🔍 Analyzing Problem Patterns...".cyan().bold());

    if let Some(ref prio) = priority {
        println!(
            "{} Filter: {} priority",
            "ℹ️".cyan(),
            format!("{:?}", prio).yellow()
        );
    }

    if recent {
        println!("{} Showing only recent problems", "ℹ️".cyan());
    }

    // Load all sessions
    let session_ids = LearningSession::list_all()?;

    if session_ids.is_empty() {
        println!("{}", "\n⚠️  No learning records found".yellow());
        return Ok(());
    }

    // Load and filter sessions
    let mut unresolved_sessions = Vec::new();

    for id in session_ids {
        if let Ok(session) = LearningSession::load(&id) {
            if !session.resolved {
                // Filter by recency if requested
                if recent {
                    if is_recent(&session.timestamp) {
                        unresolved_sessions.push(session);
                    }
                } else {
                    unresolved_sessions.push(session);
                }
            }
        }
    }

    // Display results
    if unresolved_sessions.is_empty() {
        println!("{}", "\n✅ No unresolved problems found!".green().bold());
        return Ok(());
    }

    println!(
        "\n{} Found {} unresolved problem(s)",
        "⚠️".yellow(),
        unresolved_sessions.len()
    );

    // Analyze and categorize problems
    let analysis = analyze_problems(&unresolved_sessions);

    // Display problem categories
    display_problem_categories(&analysis);

    // Display individual problems
    display_unresolved_problems(&unresolved_sessions, priority);

    // Display recommendations
    display_recommendations(&analysis);

    Ok(())
}

/// Check if timestamp is recent (within last 7 days)
fn is_recent(timestamp: &str) -> bool {
    use chrono::{DateTime, Duration, Local};

    if let Ok(session_time) = DateTime::parse_from_str(timestamp, "%Y-%m-%d %H:%M:%S") {
        let session_local: DateTime<Local> = session_time.into();
        let cutoff = Local::now() - Duration::days(7);
        session_local > cutoff
    } else {
        false
    }
}

/// Problem analysis results
struct ProblemAnalysis {
    by_type: HashMap<String, usize>,
    by_tag: HashMap<String, usize>,
    recurring_files: HashMap<String, usize>,
    total_count: usize,
}

/// Analyze problem patterns
fn analyze_problems(sessions: &[LearningSession]) -> ProblemAnalysis {
    let mut by_type: HashMap<String, usize> = HashMap::new();
    let mut by_tag: HashMap<String, usize> = HashMap::new();
    let mut recurring_files: HashMap<String, usize> = HashMap::new();

    for session in sessions {
        // Count by type
        *by_type.entry(session.session_type.clone()).or_insert(0) += 1;

        // Count by tag
        for tag in &session.tags {
            *by_tag.entry(tag.clone()).or_insert(0) += 1;
        }

        // Track recurring files
        for file in &session.files_affected {
            *recurring_files.entry(file.clone()).or_insert(0) += 1;
        }
    }

    ProblemAnalysis {
        by_type,
        by_tag,
        recurring_files,
        total_count: sessions.len(),
    }
}

/// Display problem categories
fn display_problem_categories(analysis: &ProblemAnalysis) {
    use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Table};

    println!("\n{}", "📊 Problem Categories".green().bold());

    // By type
    if !analysis.by_type.is_empty() {
        let mut type_table = Table::new();
        type_table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_header(vec!["Session Type", "Count"]);

        let mut types: Vec<_> = analysis.by_type.iter().collect();
        types.sort_by(|a, b| b.1.cmp(a.1));

        for (type_name, count) in types {
            type_table.add_row(vec![type_name, &count.to_string()]);
        }

        println!("{}\n", type_table);
    }

    // By tag
    if !analysis.by_tag.is_empty() {
        println!("{}", "🏷️  Most Common Tags".green().bold());
        let mut tag_table = Table::new();
        tag_table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_header(vec!["Tag", "Frequency"]);

        let mut tags: Vec<_> = analysis.by_tag.iter().collect();
        tags.sort_by(|a, b| b.1.cmp(a.1));

        for (tag, count) in tags.iter().take(5) {
            tag_table.add_row(vec![tag, &count.to_string()]);
        }

        println!("{}\n", tag_table);
    }

    // Recurring files
    let hotspots: Vec<_> = analysis
        .recurring_files
        .iter()
        .filter(|(_, count)| **count > 1)
        .collect();

    if !hotspots.is_empty() {
        println!("{}", "🔥 Problem Hotspots (Recurring Files)".red().bold());
        let mut file_table = Table::new();
        file_table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_header(vec!["File", "Occurrences"]);

        let mut sorted_hotspots = hotspots;
        sorted_hotspots.sort_by(|a, b| b.1.cmp(a.1));

        for (file, count) in sorted_hotspots.iter().take(10) {
            file_table.add_row(vec![file, &count.to_string()]);
        }

        println!("{}\n", file_table);
    }
}

/// Display unresolved problems
fn display_unresolved_problems(sessions: &[LearningSession], priority: Option<Priority>) {
    println!("{}", "⚠️  Unresolved Problems".yellow().bold());

    // Sort by timestamp (most recent first)
    let mut sorted_sessions = sessions.to_vec();
    sorted_sessions.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

    for (i, session) in sorted_sessions.iter().enumerate() {
        // Apply priority filter if specified
        if let Some(ref prio) = priority {
            // Determine session priority based on tags or other factors
            let session_priority = infer_priority(session);
            if !matches_priority(&session_priority, prio) {
                continue;
            }
        }

        display_problem_brief(session, i + 1);
    }
}

/// Infer priority from session data
fn infer_priority(session: &LearningSession) -> Priority {
    // Check for high-priority indicators
    if session.session_type == "urgent" {
        return Priority::Critical;
    }

    if session.tags.contains(&"security".to_string())
        || session.tags.contains(&"production".to_string())
    {
        return Priority::High;
    }

    if session.tags.contains(&"bug-fix".to_string()) {
        return Priority::Medium;
    }

    Priority::Low
}

/// Check if priorities match
fn matches_priority(session_priority: &Priority, filter: &Priority) -> bool {
    matches!(
        (session_priority, filter),
        (Priority::Critical, Priority::Critical)
            | (Priority::High, Priority::High)
            | (Priority::Medium, Priority::Medium)
            | (Priority::Low, Priority::Low)
    )
}

/// Display brief problem information
fn display_problem_brief(session: &LearningSession, index: usize) {
    use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Cell, Color, Table};

    println!("\n{} Problem #{}", "🔴".red(), index);

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec!["Field", "Value"]);

    // Priority indicator
    let priority = infer_priority(session);
    let priority_color = match priority {
        Priority::Critical => Color::Red,
        Priority::High => Color::Yellow,
        Priority::Medium => Color::Cyan,
        Priority::Low => Color::Green,
    };
    table.add_row(vec![
        Cell::new("Priority"),
        Cell::new(format!("{:?}", priority)).fg(priority_color),
    ]);

    table.add_row(vec!["ID", &session.id]);
    table.add_row(vec!["Type", &session.session_type]);
    table.add_row(vec!["Timestamp", &session.timestamp]);

    let truncated_desc = if session.description.len() > 60 {
        format!("{}...", &session.description[..57])
    } else {
        session.description.clone()
    };
    table.add_row(vec!["Description", &truncated_desc]);

    if !session.tags.is_empty() {
        table.add_row(vec!["Tags", &session.tags.join(", ")]);
    }

    if !session.files_affected.is_empty() {
        let files = if session.files_affected.len() > 3 {
            format!(
                "{}, ... ({} files)",
                session.files_affected[..3].join(", "),
                session.files_affected.len()
            )
        } else {
            session.files_affected.join(", ")
        };
        table.add_row(vec!["Files", &files]);
    }

    println!("{}", table);
}

/// Display recommendations
fn display_recommendations(analysis: &ProblemAnalysis) {
    println!("\n{}", "💡 Recommendations".yellow().bold());

    // Hotspot recommendation
    let critical_hotspots: Vec<_> = analysis
        .recurring_files
        .iter()
        .filter(|(_, count)| **count > 2)
        .collect();

    if !critical_hotspots.is_empty() {
        println!(
            "  {} {} file(s) have recurring issues - consider refactoring:",
            "⚠️".yellow(),
            critical_hotspots.len()
        );
        for (file, count) in critical_hotspots.iter().take(5) {
            println!("    • {} ({} issues)", file, count);
        }
    }

    // Pattern-based recommendations
    if let Some((common_type, count)) = analysis.by_type.iter().max_by_key(|(_, c)| *c) {
        if *count > 2 {
            println!(
                "  {} Multiple {} issues detected ({}) - consider systematic review",
                "💡".cyan(),
                common_type,
                count
            );
        }
    }

    // General recommendations
    if analysis.total_count > 5 {
        println!(
            "  {} {} unresolved issues - consider dedicating time to address backlog",
            "📋".cyan(),
            analysis.total_count
        );
    }

    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_inference() {
        let mut session = LearningSession::new("urgent", "Critical bug");
        let priority = infer_priority(&session);
        assert!(matches!(priority, Priority::Critical));

        session.session_type = "debug".to_string();
        session.tags = vec!["security".to_string()];
        let priority = infer_priority(&session);
        assert!(matches!(priority, Priority::High));
    }

    #[test]
    fn test_priority_matching() {
        assert!(matches_priority(&Priority::High, &Priority::High));
        assert!(!matches_priority(&Priority::High, &Priority::Low));
    }
}
