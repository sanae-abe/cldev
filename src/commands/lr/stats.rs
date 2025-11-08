use crate::cli::args::TimePeriod;
use crate::core::{LearningSession, Result};
use chrono::{DateTime, Duration, Local, TimeZone};
use colored::Colorize;
use std::collections::HashMap;

/// Handle learning statistics command
pub fn handle_stats(period: TimePeriod, detailed: bool) -> Result<()> {
    println!("{}", "📊 Learning Record Statistics".cyan().bold());

    let period_name = match period {
        TimePeriod::Day => "Today",
        TimePeriod::Week => "This Week",
        TimePeriod::Month => "This Month",
        TimePeriod::Year => "This Year",
    };
    println!("{} Period: {}\n", "ℹ️".cyan(), period_name.yellow());

    // Load all sessions
    let session_ids = LearningSession::list_all()?;

    if session_ids.is_empty() {
        println!("{}", "⚠️  No learning records found".yellow());
        return Ok(());
    }

    // Filter sessions by time period
    let cutoff_date = calculate_cutoff_date(period);
    let mut sessions = Vec::new();

    for id in session_ids {
        if let Ok(session) = LearningSession::load(&id) {
            if is_within_period(&session.timestamp, cutoff_date) {
                sessions.push(session);
            }
        }
    }

    if sessions.is_empty() {
        println!("{}", "⚠️  No records found for this period".yellow());
        return Ok(());
    }

    // Calculate statistics
    let stats = calculate_statistics(&sessions);

    // Display statistics
    display_overview_stats(&stats);

    if detailed {
        display_detailed_stats(&stats, &sessions);
    }

    // Display insights
    display_insights(&stats);

    Ok(())
}

/// Calculate cutoff date for time period
fn calculate_cutoff_date(period: TimePeriod) -> DateTime<Local> {
    let now = Local::now();

    match period {
        TimePeriod::Day => now - Duration::days(1),
        TimePeriod::Week => now - Duration::weeks(1),
        TimePeriod::Month => now - Duration::days(30),
        TimePeriod::Year => now - Duration::days(365),
    }
}

/// Check if session is within period
fn is_within_period(timestamp: &str, cutoff: DateTime<Local>) -> bool {
    // Try parsing with timezone first (new format)
    if let Ok(session_time) = DateTime::parse_from_str(timestamp, "%Y-%m-%d %H:%M:%S %z") {
        let session_local: DateTime<Local> = session_time.into();
        return session_local > cutoff;
    }

    // Fallback: parse without timezone (legacy format) - assume local timezone
    if let Ok(naive_time) = chrono::NaiveDateTime::parse_from_str(timestamp, "%Y-%m-%d %H:%M:%S") {
        let session_local = Local.from_local_datetime(&naive_time).single();
        if let Some(session_local) = session_local {
            return session_local > cutoff;
        }
    }

    // Last resort: try parsing date only
    if let Ok(naive_date) = chrono::NaiveDate::parse_from_str(timestamp, "%Y-%m-%d") {
        let naive_time = naive_date.and_hms_opt(0, 0, 0).unwrap();
        let session_local = Local.from_local_datetime(&naive_time).single();
        if let Some(session_local) = session_local {
            return session_local > cutoff;
        }
    }

    false
}

/// Statistics structure
struct Statistics {
    total_sessions: usize,
    resolved_count: usize,
    unresolved_count: usize,
    total_duration_minutes: u32,
    avg_duration_minutes: f64,
    session_types: HashMap<String, usize>,
    tag_frequency: HashMap<String, usize>,
    files_affected_count: usize,
}

/// Calculate statistics from sessions
fn calculate_statistics(sessions: &[LearningSession]) -> Statistics {
    let mut session_types: HashMap<String, usize> = HashMap::new();
    let mut tag_frequency: HashMap<String, usize> = HashMap::new();
    let mut total_duration = 0u32;
    let mut duration_count = 0;
    let mut files_count = 0;
    let mut resolved = 0;

    for session in sessions {
        // Session types
        *session_types
            .entry(session.session_type.clone())
            .or_insert(0) += 1;

        // Tags
        for tag in &session.tags {
            *tag_frequency.entry(tag.clone()).or_insert(0) += 1;
        }

        // Duration
        if let Some(duration) = session.duration_minutes {
            total_duration += duration;
            duration_count += 1;
        }

        // Files
        files_count += session.files_affected.len();

        // Resolved status
        if session.resolved {
            resolved += 1;
        }
    }

    let avg_duration = if duration_count > 0 {
        total_duration as f64 / duration_count as f64
    } else {
        0.0
    };

    Statistics {
        total_sessions: sessions.len(),
        resolved_count: resolved,
        unresolved_count: sessions.len() - resolved,
        total_duration_minutes: total_duration,
        avg_duration_minutes: avg_duration,
        session_types,
        tag_frequency,
        files_affected_count: files_count,
    }
}

/// Display overview statistics
fn display_overview_stats(stats: &Statistics) {
    use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Table};

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec!["Metric", "Value"]);

    table.add_row(vec!["Total Sessions", &stats.total_sessions.to_string()]);
    table.add_row(vec!["Resolved", &stats.resolved_count.to_string()]);
    table.add_row(vec!["Unresolved", &stats.unresolved_count.to_string()]);

    let resolution_rate = if stats.total_sessions > 0 {
        (stats.resolved_count as f64 / stats.total_sessions as f64) * 100.0
    } else {
        0.0
    };
    table.add_row(vec!["Resolution Rate", &format!("{:.1}%", resolution_rate)]);

    if stats.total_duration_minutes > 0 {
        table.add_row(vec![
            "Total Time Invested",
            &format!(
                "{} min ({:.1} hrs)",
                stats.total_duration_minutes,
                stats.total_duration_minutes as f64 / 60.0
            ),
        ]);
        table.add_row(vec![
            "Avg. Resolution Time",
            &format!("{:.1} min", stats.avg_duration_minutes),
        ]);
    }

    table.add_row(vec![
        "Files Affected",
        &stats.files_affected_count.to_string(),
    ]);

    println!("{}", "📈 Overview".green().bold());
    println!("{}\n", table);
}

/// Display detailed statistics
fn display_detailed_stats(stats: &Statistics, sessions: &[LearningSession]) {
    use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Table};

    // Session types breakdown
    println!("{}", "📋 Session Types".green().bold());
    let mut type_table = Table::new();
    type_table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec!["Type", "Count", "Percentage"]);

    let mut types: Vec<_> = stats.session_types.iter().collect();
    types.sort_by(|a, b| b.1.cmp(a.1));

    for (type_name, count) in types {
        let percentage = (*count as f64 / stats.total_sessions as f64) * 100.0;
        type_table.add_row(vec![
            type_name,
            &count.to_string(),
            &format!("{:.1}%", percentage),
        ]);
    }
    println!("{}\n", type_table);

    // Top tags
    println!("{}", "🏷️  Top Tags".green().bold());
    let mut tag_table = Table::new();
    tag_table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec!["Tag", "Frequency"]);

    let mut tags: Vec<_> = stats.tag_frequency.iter().collect();
    tags.sort_by(|a, b| b.1.cmp(a.1));

    for (tag, count) in tags.iter().take(10) {
        tag_table.add_row(vec![tag, &count.to_string()]);
    }
    println!("{}\n", tag_table);

    // Learning insights count
    let total_learnings: usize = sessions.iter().map(|s| s.learnings.len()).sum();
    println!("{}", "💡 Learning Insights".green().bold());
    println!("  Total insights captured: {}\n", total_learnings);
}

/// Display insights and recommendations
fn display_insights(stats: &Statistics) {
    println!("{}", "🎯 Insights & Recommendations".yellow().bold());

    // Resolution rate insight
    let resolution_rate = if stats.total_sessions > 0 {
        (stats.resolved_count as f64 / stats.total_sessions as f64) * 100.0
    } else {
        0.0
    };

    if resolution_rate < 50.0 {
        println!(
            "  {} Low resolution rate ({:.1}%). Consider reviewing unresolved items.",
            "⚠️".yellow(),
            resolution_rate
        );
    } else if resolution_rate > 80.0 {
        println!(
            "  {} Excellent resolution rate ({:.1}%)!",
            "✅".green(),
            resolution_rate
        );
    }

    // Time investment insight
    if stats.avg_duration_minutes > 0.0 {
        if stats.avg_duration_minutes > 120.0 {
            println!(
                "  {} Average resolution time is high ({:.1} min). Consider breaking down complex tasks.",
                "💡".yellow(),
                stats.avg_duration_minutes
            );
        } else if stats.avg_duration_minutes < 30.0 {
            println!(
                "  {} Quick average resolution time ({:.1} min) - efficient problem solving!",
                "✅".green(),
                stats.avg_duration_minutes
            );
        }
    }

    // Most common issue types
    if let Some((most_common_type, count)) =
        stats.session_types.iter().max_by_key(|(_, count)| *count)
    {
        println!(
            "  {} Most common session type: {} ({} occurrences)",
            "📊".cyan(),
            most_common_type,
            count
        );
    }

    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cutoff_calculation() {
        let cutoff = calculate_cutoff_date(TimePeriod::Day);
        let now = Local::now();
        assert!(cutoff < now);
    }

    #[test]
    fn test_statistics_calculation() {
        let sessions = vec![];
        let stats = calculate_statistics(&sessions);
        assert_eq!(stats.total_sessions, 0);
    }
}
