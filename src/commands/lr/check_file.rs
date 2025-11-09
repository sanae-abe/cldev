use crate::core::learning_db::LearningDatabase;
use crate::core::Result;
use colored::Colorize;

/// Check if a file is a hotspot and warn about past issues
///
/// This command helps developers be aware of problematic files before editing them.
/// It checks if the given file path appears in the top hotspots and displays
/// warnings with past problems encountered in that file.
///
/// # Arguments
///
/// * `file_path` - Path to the file being edited
///
/// # Returns
///
/// Returns Ok(()) if check completes successfully, Err if database issues occur
pub fn handle_check_file(file_path: &str) -> Result<()> {
    println!("{}", "🔍 Checking file hotspot status...".cyan().bold());

    // Get learning records directory
    let home_dir = dirs::home_dir().expect("Could not find home directory");
    let cldev_dir = home_dir.join(".cldev");
    let learning_dir = cldev_dir.join("learning-records");

    // Initialize learning database
    let db_path = learning_dir.join("index.db");
    let markdown_dir = learning_dir;
    let db = LearningDatabase::new(db_path, markdown_dir)?;

    // Get top hotspots
    let hotspots = db.get_hotspots(20)?;

    // Check if the file is in hotspots
    let matching_hotspot = hotspots
        .iter()
        .find(|h| h.file_path.contains(file_path) || file_path.contains(&h.file_path));

    match matching_hotspot {
        Some(hotspot) => {
            // File is a hotspot - display warning
            println!(
                "{}",
                "⚠️  WARNING: This file is a known hotspot!".yellow().bold()
            );
            println!("File: {}", hotspot.file_path.green());
            println!(
                "Session count: {}",
                hotspot.session_count.to_string().yellow()
            );
            println!("Avg hotspot score: {:.2}", hotspot.avg_hotspot_score);
            println!("Last accessed: {}", hotspot.last_accessed);
            println!();

            // Fetch related sessions for this file
            let related_sessions = db.query_by_file(&hotspot.file_path, 5)?;

            if !related_sessions.is_empty() {
                println!("{}", "📋 Past Issues:".cyan().bold());
                for (idx, result) in related_sessions.iter().enumerate() {
                    let session = &result.session;
                    let resolved_status = if session.resolved {
                        "✓ Resolved".green()
                    } else {
                        "✗ Unresolved".red()
                    };

                    println!(
                        "{}. [{}] {} - {} ({})",
                        idx + 1,
                        session.priority.yellow(),
                        session.title,
                        session.session_type,
                        resolved_status
                    );

                    if !session.description.is_empty() {
                        println!("   {}", session.description.dimmed());
                    }
                }
            }
        }
        None => {
            // File is not a hotspot - no warnings
            println!("{}", "✅ No known issues with this file".green().bold());
            println!("File '{}' is not in the hotspot list", file_path.green());
        }
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
    fn test_check_file_hotspot() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let markdown_dir = temp_dir.path().join("markdown");
        fs::create_dir_all(&markdown_dir).unwrap();

        let mut db = LearningDatabase::new(db_path, markdown_dir.clone()).unwrap();

        // Create a test record with a specific file
        let record = LearningRecordBuilder::new(
            SessionType::Debug,
            Priority::High,
            "Test Hotspot",
            "File with issues",
            Severity::Error,
        )
        .files(vec![FileAffected {
            path: "src/core/test.rs".to_string(),
            role: FileRole::Primary,
            changes_summary: None,
            hotspot_score: 10.0,
        }])
        .build();

        let path = markdown_dir.join(format!("{}.md", record.session_meta.id));
        let yaml = serde_yaml::to_string(&record).unwrap();
        let content = format!("---\n{}---\n", yaml);
        fs::write(&path, content).unwrap();

        db.upsert_session(&record, path.to_string_lossy().to_string())
            .unwrap();

        // Get hotspots and verify
        let hotspots = db.get_hotspots(20).unwrap();
        assert!(!hotspots.is_empty());
        assert!(hotspots[0].file_path.contains("test.rs"));
    }

    #[test]
    fn test_check_file_no_hotspot() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let markdown_dir = temp_dir.path().join("markdown");
        fs::create_dir_all(&markdown_dir).unwrap();

        let db = LearningDatabase::new(db_path, markdown_dir).unwrap();

        // No records - no hotspots
        let hotspots = db.get_hotspots(20).unwrap();
        assert!(hotspots.is_empty());
    }
}
