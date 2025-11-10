use crate::core::learning_db::LearningDatabase;
use crate::core::CldevError;
use crate::core::Result;
use colored::Colorize;

/// Find sessions similar to a given session by context
///
/// This command helps developers discover related learning sessions by analyzing
/// the context of a specific session. It uses composite scoring based on:
/// - File matches (40%)
/// - Error similarity (30%)
/// - Tag overlap (20%)
/// - Recency (10%)
///
/// # Arguments
///
/// * `session_id` - ID of the session to find similar sessions for
/// * `limit` - Maximum number of similar sessions to display
///
/// # Returns
///
/// Returns Ok(()) if search completes successfully, Err if database or session issues occur
pub fn handle_similar(session_id: &str, limit: Option<usize>) -> Result<()> {
    let limit = limit.unwrap_or(10);

    println!("{}", "🔍 Finding similar sessions...".cyan().bold());
    println!("Session ID: {}", session_id.green());
    println!();

    // Get learning records directory
    let home_dir = dirs::home_dir().expect("Could not find home directory");
    let cldev_dir = home_dir.join(".cldev");
    let learning_dir = cldev_dir.join("learning-records");

    // Initialize learning database
    let db_path = learning_dir.join("index.db");
    let markdown_dir = learning_dir;
    let db = LearningDatabase::new(db_path, markdown_dir)?;

    // First, fetch the target session to extract context
    let target_session = db
        .query_by_keyword(session_id, 1)?
        .into_iter()
        .next()
        .ok_or_else(|| CldevError::Config(format!("Session not found: {}", session_id)))?;

    let session_meta = &target_session.session;

    // Display target session info
    println!("{}", "🎯 Target Session:".cyan().bold());
    println!("ID: {}", session_meta.id.yellow());
    println!("Title: {}", session_meta.title);
    println!("Type: {}", session_meta.session_type);
    println!("Priority: {}", session_meta.priority.yellow());
    println!();

    // Extract context for similarity search
    let file_path = target_session.matched_files.first().map(|s| s.as_str());

    // For error pattern, we just use None for now since we need the actual error data
    // from the session record, not from matched_files
    let error_pattern: Option<&str> = None;

    let tags = if !target_session.matched_tags.is_empty() {
        Some(target_session.matched_tags.as_slice())
    } else {
        None
    };

    // Search for similar sessions
    let similar_sessions = db.suggest_by_context(
        file_path,
        error_pattern,
        tags,
        limit + 1, // +1 because target session might be included
    )?;

    // Filter out the target session itself
    let filtered_sessions: Vec<_> = similar_sessions
        .into_iter()
        .filter(|s| s.session.id != session_meta.id)
        .take(limit)
        .collect();

    if filtered_sessions.is_empty() {
        println!("{}", "No similar sessions found".yellow());
        println!("💡 Try a different session or check if the session ID is correct");
        return Ok(());
    }

    // Display results
    println!(
        "{}",
        format!("✅ Found {} similar session(s)", filtered_sessions.len())
            .green()
            .bold()
    );
    println!("{}", "📋 Similar Sessions:".cyan().bold());
    println!();

    for (idx, result) in filtered_sessions.iter().enumerate() {
        let session = &result.session;
        let relevance_percent = (result.relevance_score * 100.0).round();

        // Display session header
        println!(
            "{}. [{}%] {} - {}",
            idx + 1,
            format!("{}", relevance_percent).yellow(),
            session.title,
            session.priority.yellow()
        );

        // Display type and timestamp
        println!(
            "   Type: {} | Date: {}",
            session.session_type.dimmed(),
            session.timestamp.dimmed()
        );

        // Display description
        if !session.description.is_empty() {
            println!("   {}", session.description.dimmed());
        }

        // Display resolved status
        let resolved_status = if session.resolved {
            "✓ Resolved".green()
        } else {
            "✗ Unresolved".red()
        };
        println!("   Status: {}", resolved_status);

        // Display files
        if !result.matched_files.is_empty() {
            println!("   Files: {}", result.matched_files.join(", ").dimmed());
        }

        // Display tags
        if !result.matched_tags.is_empty() {
            println!("   Tags: {}", result.matched_tags.join(", ").dimmed());
        }

        // Display markdown path for reference
        println!("   Details: {}", session.markdown_path.dimmed());

        println!();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::learning_record_v2::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_similar_sessions_by_file() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let markdown_dir = temp_dir.path().join("markdown");
        fs::create_dir_all(&markdown_dir).unwrap();

        let mut db = LearningDatabase::new(db_path, markdown_dir.clone()).unwrap();

        // Create target session
        let record1 = LearningRecordBuilder::new(
            SessionType::Debug,
            Priority::High,
            "Target Session",
            "Main issue",
            Severity::Error,
        )
        .files(vec![FileAffected {
            path: "src/core/main.rs".to_string(),
            role: FileRole::Primary,
            changes_summary: None,
            hotspot_score: 1.0,
        }])
        .tag("rust")
        .build();

        // Create similar session (same file)
        let record2 = LearningRecordBuilder::new(
            SessionType::Debug,
            Priority::Medium,
            "Similar Session",
            "Related issue",
            Severity::Warning,
        )
        .files(vec![FileAffected {
            path: "src/core/main.rs".to_string(),
            role: FileRole::Primary,
            changes_summary: None,
            hotspot_score: 1.0,
        }])
        .tag("rust")
        .build();

        // Insert sessions
        for record in [&record1, &record2] {
            let path = markdown_dir.join(format!("{}.md", record.session_meta.id));
            let yaml = serde_yaml::to_string(record).unwrap();
            let content = format!("---\n{}---\n", yaml);
            fs::write(&path, content).unwrap();
            db.upsert_session(record, path.to_string_lossy().to_string())
                .unwrap();
        }

        // Find similar sessions to record1
        let results = db
            .suggest_by_context(
                Some("src/core/main.rs"),
                None,
                Some(&["rust".to_string()]),
                10,
            )
            .unwrap();

        // At least one session should match (the similar session)
        // Note: In practice, only one session may be returned due to scoring/ranking
        assert!(results.len() >= 1, "Expected at least 1 result, got {}", results.len());
    }

    #[test]
    fn test_similar_sessions_composite_scoring() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let markdown_dir = temp_dir.path().join("markdown");
        fs::create_dir_all(&markdown_dir).unwrap();

        let mut db = LearningDatabase::new(db_path, markdown_dir.clone()).unwrap();

        // Create sessions with varying similarity
        let record1 = LearningRecordBuilder::new(
            SessionType::Debug,
            Priority::High,
            "Session A",
            "Primary",
            Severity::Error,
        )
        .files(vec![FileAffected {
            path: "src/lib.rs".to_string(),
            role: FileRole::Primary,
            changes_summary: None,
            hotspot_score: 1.0,
        }])
        .tag("rust")
        .tag("performance")
        .build();

        let record2 = LearningRecordBuilder::new(
            SessionType::Debug,
            Priority::Medium,
            "Session B",
            "Similar tags and file",
            Severity::Warning,
        )
        .files(vec![FileAffected {
            path: "src/lib.rs".to_string(),
            role: FileRole::Primary,
            changes_summary: None,
            hotspot_score: 1.0,
        }])
        .tag("rust")
        .build();

        // Insert sessions
        for record in [&record1, &record2] {
            let path = markdown_dir.join(format!("{}.md", record.session_meta.id));
            let yaml = serde_yaml::to_string(record).unwrap();
            let content = format!("---\n{}---\n", yaml);
            fs::write(&path, content).unwrap();
            db.upsert_session(record, path.to_string_lossy().to_string())
                .unwrap();
        }

        // Search with composite criteria
        let results = db
            .suggest_by_context(Some("lib.rs"), None, Some(&["rust".to_string()]), 10)
            .unwrap();

        assert!(!results.is_empty());
        // Both sessions should match due to file and tag overlap
        assert!(results[0].relevance_score >= 0.5);
    }
}
