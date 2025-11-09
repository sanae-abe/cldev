//! TF-IDF Search Engine Demo
//!
//! Demonstrates the TF-IDF search functionality for learning records.

use cldev::core::{LearningDatabase, LearningRecordBuilder, Priority, SessionType, Severity};
use tempfile::TempDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== TF-IDF Search Engine Demo ===\n");

    // Create temporary directories for demo
    let temp_dir = TempDir::new()?;
    let db_path = temp_dir.path().join("demo.db");
    let markdown_dir = temp_dir.path().join("records");
    std::fs::create_dir_all(&markdown_dir)?;

    // Initialize database with TF-IDF index
    let mut db = LearningDatabase::new(db_path, markdown_dir.clone())?;
    println!("✓ Database initialized with TF-IDF index\n");

    // Create sample learning records
    println!("Creating sample learning records...");

    let record1 = LearningRecordBuilder::new(
        SessionType::Debug,
        Priority::High,
        "Rust async/await deadlock",
        "Encountered a deadlock when using async/await with tokio runtime. The issue occurred in production.",
        Severity::Critical,
    )
    .tag("rust")
    .tag("async")
    .tag("tokio")
    .tag("deadlock")
    .build();

    let record2 = LearningRecordBuilder::new(
        SessionType::Feature,
        Priority::Medium,
        "Implement CLI argument parsing",
        "Added command-line argument parsing using clap crate for better user experience.",
        Severity::Info,
    )
    .tag("rust")
    .tag("cli")
    .tag("clap")
    .build();

    let record3 = LearningRecordBuilder::new(
        SessionType::Optimize,
        Priority::High,
        "Database query optimization",
        "Optimized SQLite queries by adding proper indexes. Query time reduced from 5s to 50ms.",
        Severity::Warning,
    )
    .tag("database")
    .tag("sqlite")
    .tag("performance")
    .tag("optimization")
    .build();

    let record4 = LearningRecordBuilder::new(
        SessionType::Research,
        Priority::Low,
        "TF-IDF algorithm research",
        "Researched TF-IDF (Term Frequency-Inverse Document Frequency) for implementing semantic search.",
        Severity::Info,
    )
    .tag("algorithm")
    .tag("tfidf")
    .tag("search")
    .tag("research")
    .build();

    // Insert records into database
    let path1 = markdown_dir.join(format!("{}.md", record1.session_meta.id));
    let path2 = markdown_dir.join(format!("{}.md", record2.session_meta.id));
    let path3 = markdown_dir.join(format!("{}.md", record3.session_meta.id));
    let path4 = markdown_dir.join(format!("{}.md", record4.session_meta.id));

    // Create placeholder markdown files
    std::fs::write(&path1, "---\n# Placeholder\n---\n")?;
    std::fs::write(&path2, "---\n# Placeholder\n---\n")?;
    std::fs::write(&path3, "---\n# Placeholder\n---\n")?;
    std::fs::write(&path4, "---\n# Placeholder\n---\n")?;

    db.upsert_session(&record1, path1.to_string_lossy().to_string())?;
    db.upsert_session(&record2, path2.to_string_lossy().to_string())?;
    db.upsert_session(&record3, path3.to_string_lossy().to_string())?;
    db.upsert_session(&record4, path4.to_string_lossy().to_string())?;

    println!("✓ Inserted 4 learning records\n");

    // Display TF-IDF index statistics
    let stats = db.tfidf_stats();
    println!("TF-IDF Index Statistics:");
    println!("  Documents: {}", stats.doc_count);
    println!("  Unique terms: {}", stats.term_count);
    println!("  Avg document length: {:.1} words\n", stats.avg_doc_length);

    // Perform various TF-IDF searches
    println!("=== Search Demonstrations ===\n");

    // Search 1: Rust-related
    println!("1. Search: 'rust async tokio'");
    let results = db.search_with_tfidf("rust async tokio", 3)?;
    println!("   Found {} results:", results.len());
    for (i, result) in results.iter().enumerate() {
        println!(
            "   {}. [Score: {:.3}] {}",
            i + 1,
            result.relevance_score,
            result.session.title
        );
        println!("      Tags: {}", result.matched_tags.join(", "));
    }
    println!();

    // Search 2: Performance optimization
    println!("2. Search: 'optimization performance'");
    let results = db.search_with_tfidf("optimization performance", 3)?;
    println!("   Found {} results:", results.len());
    for (i, result) in results.iter().enumerate() {
        println!(
            "   {}. [Score: {:.3}] {}",
            i + 1,
            result.relevance_score,
            result.session.title
        );
    }
    println!();

    // Search 3: TF-IDF algorithm
    println!("3. Search: 'tfidf algorithm search'");
    let results = db.search_with_tfidf("tfidf algorithm search", 3)?;
    println!("   Found {} results:", results.len());
    for (i, result) in results.iter().enumerate() {
        println!(
            "   {}. [Score: {:.3}] {}",
            i + 1,
            result.relevance_score,
            result.session.title
        );
    }
    println!();

    // Search 4: Database
    println!("4. Search: 'database sqlite'");
    let results = db.search_with_tfidf("database sqlite", 3)?;
    println!("   Found {} results:", results.len());
    for (i, result) in results.iter().enumerate() {
        println!(
            "   {}. [Score: {:.3}] {}",
            i + 1,
            result.relevance_score,
            result.session.title
        );
    }
    println!();

    println!("=== Demo Complete ===");
    println!("\nKey Observations:");
    println!("• TF-IDF scores reflect term relevance and rarity");
    println!("• More specific terms (e.g., 'tokio') yield higher scores");
    println!("• Common terms (e.g., 'database') have lower IDF weights");
    println!("• Results are ranked by combined TF-IDF score");

    Ok(())
}
