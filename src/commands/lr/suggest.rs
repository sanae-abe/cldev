use crate::core::learning_db::LearningDatabase;
use crate::core::Result;
use colored::Colorize;

/// Suggest similar problems based on an error message
///
/// This command helps developers find solutions to errors by searching for
/// similar error patterns in past learning records. It uses Levenshtein distance
/// to calculate similarity and returns sessions with similar errors.
///
/// # Arguments
///
/// * `error_msg` - Error message or pattern to search for
/// * `threshold` - Similarity threshold (0.0-1.0), default 0.7
/// * `limit` - Maximum number of results to display
///
/// # Returns
///
/// Returns Ok(()) if search completes successfully, Err if database issues occur
pub fn handle_suggest(error_msg: &str, threshold: Option<f64>, limit: Option<usize>) -> Result<()> {
    let threshold = threshold.unwrap_or(0.7);
    let limit = limit.unwrap_or(10);

    // Validate threshold
    if !(0.0..=1.0).contains(&threshold) {
        println!(
            "{}",
            format!(
                "❌ Invalid threshold: {}. Must be between 0.0 and 1.0",
                threshold
            )
            .red()
        );
        return Ok(());
    }

    println!("{}", "🔍 Searching for similar errors...".cyan().bold());
    println!("Error: {}", error_msg.green());
    println!("Threshold: {:.2}", threshold);
    println!();

    // Get learning records directory
    let home_dir = dirs::home_dir().expect("Could not find home directory");
    let cldev_dir = home_dir.join(".cldev");
    let learning_dir = cldev_dir.join("learning-records");

    // Initialize learning database
    let db_path = learning_dir.join("index.db");
    let markdown_dir = learning_dir;
    let db = LearningDatabase::new(db_path, markdown_dir)?;

    // Find similar errors
    let results = db.find_similar_errors(error_msg, threshold, limit)?;

    if results.is_empty() {
        println!("{}", "No similar errors found".yellow());
        println!("💡 Try lowering the threshold to find more matches");
        return Ok(());
    }

    // Display results
    println!(
        "{}",
        format!("✅ Found {} similar issue(s)", results.len())
            .green()
            .bold()
    );
    println!("{}", "📋 Similar Issues:".cyan().bold());
    println!();

    for (idx, result) in results.iter().enumerate() {
        let session = &result.session;
        let similarity_percent = (result.relevance_score * 100.0).round();

        // Display session header
        println!(
            "{}. [{}%] {} - {}",
            idx + 1,
            format!("{}", similarity_percent).yellow(),
            session.title,
            session.priority.yellow()
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
    fn test_suggest_similar_errors() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let markdown_dir = temp_dir.path().join("markdown");
        fs::create_dir_all(&markdown_dir).unwrap();

        let mut db = LearningDatabase::new(db_path, markdown_dir.clone()).unwrap();

        // Create test records with error signatures
        let record1 = LearningRecordBuilder::new(
            SessionType::Debug,
            Priority::High,
            "Thread Panic",
            "Thread panicked at overflow",
            Severity::Error,
        )
        .build();

        let mut record1_with_error = record1.clone();
        record1_with_error
            .problem
            .error_signatures
            .push(ErrorSignature {
                error_type: "RuntimeError".to_string(),
                pattern: "thread panicked at overflow in main".to_string(),
                stack_trace_hash: None,
            });

        let path1 = markdown_dir.join(format!("{}.md", record1_with_error.session_meta.id));
        let yaml1 = serde_yaml::to_string(&record1_with_error).unwrap();
        let content1 = format!("---\n{}---\n", yaml1);
        fs::write(&path1, content1).unwrap();

        db.upsert_session(&record1_with_error, path1.to_string_lossy().to_string())
            .unwrap();

        // Search for similar error
        let results = db.find_similar_errors("thread panicked", 0.3, 10).unwrap();

        assert!(!results.is_empty());
        assert!(results[0].relevance_score >= 0.3);
    }

    #[test]
    fn test_suggest_threshold_filtering() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let markdown_dir = temp_dir.path().join("markdown");
        fs::create_dir_all(&markdown_dir).unwrap();

        let mut db = LearningDatabase::new(db_path, markdown_dir.clone()).unwrap();

        // Create test record
        let record = LearningRecordBuilder::new(
            SessionType::Debug,
            Priority::High,
            "Specific Error",
            "Very specific error message",
            Severity::Error,
        )
        .build();

        let mut record_with_error = record.clone();
        record_with_error
            .problem
            .error_signatures
            .push(ErrorSignature {
                error_type: "TypeError".to_string(),
                pattern: "cannot read property of undefined".to_string(),
                stack_trace_hash: None,
            });

        let path = markdown_dir.join(format!("{}.md", record_with_error.session_meta.id));
        let yaml = serde_yaml::to_string(&record_with_error).unwrap();
        let content = format!("---\n{}---\n", yaml);
        fs::write(&path, content).unwrap();

        db.upsert_session(&record_with_error, path.to_string_lossy().to_string())
            .unwrap();

        // High threshold - should not match unrelated error
        let results = db
            .find_similar_errors("completely different error", 0.9, 10)
            .unwrap();
        assert!(results.is_empty());

        // Low threshold - should match
        let results = db
            .find_similar_errors("cannot read property", 0.3, 10)
            .unwrap();
        assert!(!results.is_empty());
    }
}
