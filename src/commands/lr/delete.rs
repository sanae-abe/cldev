use crate::core::{LearningDatabase, LearningRecordV2, Result};
use colored::Colorize;
use dialoguer::Confirm;
use std::path::PathBuf;

/// Handle delete learning record command
pub fn handle_delete(id: String, skip_confirm: bool) -> Result<()> {
    println!("{}", "🗑️  Deleting learning record...".cyan().bold());
    println!("{} Session ID: {}", "ℹ️".cyan(), id.yellow());

    // Check if record exists
    let record = match LearningRecordV2::load(&id) {
        Ok(r) => r,
        Err(_) => {
            println!("{}", "❌ Learning record not found".red().bold());
            println!("{} Session ID: {}", "ℹ️".cyan(), id);
            return Err(crate::core::CldevError::config(format!(
                "Learning record not found: {}",
                id
            )));
        }
    };

    // Display record info
    println!("\n{}", "📋 Record Information".yellow().bold());
    println!("  Title: {}", record.problem.title);
    println!("  Type: {:?}", record.session_meta.session_type);
    println!("  Priority: {:?}", record.session_meta.priority);
    println!(
        "  Timestamp: {}",
        record.session_meta.timestamp.format("%Y-%m-%d %H:%M:%S")
    );

    // Confirm deletion (unless --yes flag is used)
    if !skip_confirm {
        let confirmed = Confirm::new()
            .with_prompt("Are you sure you want to delete this record?")
            .default(false)
            .interact()
            .unwrap_or(false);

        if !confirmed {
            println!("{}", "ℹ️  Deletion cancelled".cyan());
            return Ok(());
        }
    }

    // Get paths
    let home = std::env::var("HOME")
        .ok()
        .map(PathBuf::from)
        .or_else(dirs::home_dir)
        .ok_or_else(|| crate::core::CldevError::config("Failed to get home directory"))?;

    let records_dir = home.join(".claude").join("learning-records");
    let markdown_path = records_dir.join(format!("{}.md", id));
    let db_path = records_dir.join("learning.db");

    // Delete from database
    let mut db = LearningDatabase::new(db_path, records_dir)?;
    let db_deleted = db.delete_session(&id)?;

    if !db_deleted {
        println!(
            "{} Warning: Record not found in database index",
            "⚠️".yellow()
        );
    }

    // Delete markdown file
    if markdown_path.exists() {
        std::fs::remove_file(&markdown_path).map_err(|e| {
            crate::core::CldevError::config(format!("Failed to delete markdown file: {}", e))
        })?;
        println!("{} Markdown file deleted", "✅".green());
    } else {
        println!("{} Warning: Markdown file not found", "⚠️".yellow());
    }

    println!(
        "\n{}",
        "✅ Learning record deleted successfully".green().bold()
    );
    println!("{} Session ID: {}", "ℹ️".cyan(), id.yellow());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_nonexistent() {
        // Attempting to delete a non-existent record should fail
        let result = handle_delete("nonexistent_id".to_string(), true);
        assert!(result.is_err());
    }
}
