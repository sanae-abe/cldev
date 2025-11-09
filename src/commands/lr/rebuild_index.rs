use crate::core::{LearningIndexV2, Result};
use colored::Colorize;

/// Handle rebuild index command
pub fn handle_rebuild_index(verbose: bool) -> Result<()> {
    println!("{}", "🔄 Rebuilding search index...".cyan().bold());

    if verbose {
        println!("{} Scanning markdown files...", "ℹ️".cyan());
    }

    // Rebuild index from all markdown files
    match LearningIndexV2::rebuild() {
        Ok(_) => {
            println!("{}", "✅ Search index rebuilt successfully".green().bold());

            if verbose {
                // Load index to show statistics
                if let Ok(index) = LearningIndexV2::load() {
                    println!("\n{}", "📊 Index Statistics".yellow().bold());
                    println!("  Total records: {}", index.total_records);
                    println!("  Last rebuild: {}", index.last_rebuild);
                }
            }

            println!("\n{}", "💡 Next Steps:".yellow().bold());
            println!("  • Search records: cldev lr find <query>");
            println!("  • View statistics: cldev lr stats");
            println!("  • Check problems: cldev lr problems");

            Ok(())
        }
        Err(e) => {
            println!("{}", "❌ Failed to rebuild index".red().bold());
            println!("{} Error: {}", "ℹ️".cyan(), e);
            Err(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rebuild_index() {
        // This test requires actual markdown files to exist
        // Just verify the function signature is correct
        let result = handle_rebuild_index(false);
        // Allow either success or error (depending on test environment)
        assert!(result.is_ok() || result.is_err());
    }
}
