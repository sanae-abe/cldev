use crate::cli::args::TimePeriod;
use crate::core::{LearningRecordV3, LearningSession, RecordStatus, Result};
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

    // Load V1 sessions
    let session_ids = LearningSession::list_all()?;
    let cutoff_date = calculate_cutoff_date(period);
    let mut v1_sessions = Vec::new();

    for id in session_ids {
        if let Ok(session) = LearningSession::load(&id) {
            if is_within_period(&session.timestamp, cutoff_date) {
                v1_sessions.push(session);
            }
        }
    }

    // Load V3 records
    let v3_ids = LearningRecordV3::list_all()?;
    let mut v3_records = Vec::new();

    for id in v3_ids {
        if let Ok(record) = LearningRecordV3::load(&id) {
            if record.created > cutoff_date {
                v3_records.push(record);
            }
        }
    }

    if v1_sessions.is_empty() && v3_records.is_empty() {
        println!("{}", "⚠️  No learning records found".yellow());
        return Ok(());
    }

    // Calculate combined statistics
    let stats = calculate_combined_statistics(&v1_sessions, &v3_records);

    // Display statistics
    display_overview_stats(&stats);

    if detailed {
        display_detailed_stats(&stats, &v1_sessions, &v3_records);
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
    v1_count: usize,
    v3_count: usize,
    resolved_count: usize,
    unresolved_count: usize,
    total_duration_minutes: u32,
    avg_duration_minutes: f64,
    session_types: HashMap<String, usize>,
    tag_frequency: HashMap<String, usize>,
    files_affected_count: usize,
}

/// Calculate combined statistics from V1 sessions and V3 records
fn calculate_combined_statistics(
    v1_sessions: &[LearningSession],
    v3_records: &[LearningRecordV3],
) -> Statistics {
    let mut session_types: HashMap<String, usize> = HashMap::new();
    let mut tag_frequency: HashMap<String, usize> = HashMap::new();
    let mut total_duration = 0u32;
    let mut duration_count = 0;
    let mut files_count = 0;
    let mut resolved = 0;

    // Process V1 sessions
    for session in v1_sessions {
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

    // Process V3 records
    for record in v3_records {
        // Extract session type from ID or tags (V3 format)
        let session_type = if let Some(tag) = record.tags.first() {
            tag.clone()
        } else {
            "general".to_string()
        };
        *session_types.entry(session_type).or_insert(0) += 1;

        // Tags
        for tag in &record.tags {
            *tag_frequency.entry(tag.clone()).or_insert(0) += 1;
        }

        // Duration
        if let Some(duration) = record.duration_min {
            total_duration += duration as u32;
            duration_count += 1;
        }

        // Resolved status
        if record.status == RecordStatus::Resolved {
            resolved += 1;
        }
    }

    let total_sessions = v1_sessions.len() + v3_records.len();
    let avg_duration = if duration_count > 0 {
        total_duration as f64 / duration_count as f64
    } else {
        0.0
    };

    Statistics {
        total_sessions,
        v1_count: v1_sessions.len(),
        v3_count: v3_records.len(),
        resolved_count: resolved,
        unresolved_count: total_sessions - resolved,
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
    table.add_row(vec![
        "  V1 Sessions",
        &format!("{} (legacy format)", stats.v1_count),
    ]);
    table.add_row(vec![
        "  V3 Records",
        &format!("{} (new format)", stats.v3_count),
    ]);
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

    if stats.files_affected_count > 0 {
        table.add_row(vec![
            "Files Affected",
            &stats.files_affected_count.to_string(),
        ]);
    }

    println!("{}", "📈 Overview".green().bold());
    println!("{}\n", table);
}

/// Display detailed statistics
fn display_detailed_stats(
    stats: &Statistics,
    v1_sessions: &[LearningSession],
    v3_records: &[LearningRecordV3],
) {
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

    // Learning insights count (V1 sessions only)
    let total_learnings: usize = v1_sessions.iter().map(|s| s.learnings.len()).sum();
    if total_learnings > 0 {
        println!("{}", "💡 Learning Insights".green().bold());
        println!("  Total insights captured (V1): {}\n", total_learnings);
    }

    // V3 records summary
    if !v3_records.is_empty() {
        println!("{}", "📝 V3 Records Summary".green().bold());
        let auto_generated = v3_records.iter().filter(|r| r.auto_generated).count();
        let manual = v3_records.len() - auto_generated;
        println!("  Auto-generated: {}", auto_generated);
        println!("  Manual: {}", manual);

        let avg_confidence: f64 = v3_records.iter().filter_map(|r| r.confidence).sum::<f64>()
            / v3_records
                .iter()
                .filter(|r| r.confidence.is_some())
                .count()
                .max(1) as f64;

        if avg_confidence > 0.0 {
            println!("  Avg. AI Confidence: {:.1}%\n", avg_confidence * 100.0);
        } else {
            println!();
        }
    }
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
        let v1_sessions = vec![];
        let v3_records = vec![];
        let stats = calculate_combined_statistics(&v1_sessions, &v3_records);
        assert_eq!(stats.total_sessions, 0);
        assert_eq!(stats.v1_count, 0);
        assert_eq!(stats.v3_count, 0);
    }
}
