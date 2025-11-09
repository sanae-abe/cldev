//! Session CLI commands
//!
//! Commands for managing development sessions with auto-capture learning records.

use crate::core::SessionContext;
use clap::Subcommand;
use std::path::PathBuf;

/// Session management commands
#[derive(Debug, Clone, Subcommand)]
pub enum SessionCommand {
    /// Start a new development session
    Start {
        /// Session description
        #[arg(short, long)]
        description: Option<String>,
    },
    /// End the current session and optionally create a learning record
    End {
        /// Force creation without prompting
        #[arg(short, long)]
        force: bool,
    },
    /// Show current session status
    Status,
}

/// Handle session commands
pub fn handle_session(cmd: SessionCommand) -> crate::core::Result<()> {
    match cmd {
        SessionCommand::Start { description } => handle_start(description),
        SessionCommand::End { force } => handle_end(force),
        SessionCommand::Status => handle_status(),
    }
}

fn handle_start(description: Option<String>) -> crate::core::Result<()> {
    use chrono::Local;

    let session_id = format!("session-{}", Local::now().format("%Y%m%d-%H%M%S"));
    let ctx = SessionContext::new(session_id.clone());

    // Save session to temp file
    let session_path = get_session_path();
    std::fs::create_dir_all(session_path.parent().unwrap())?;
    let json = serde_json::to_string_pretty(&ctx)?;
    std::fs::write(&session_path, json)?;

    println!("✅ Session started: {}", session_id);
    if let Some(desc) = description {
        println!("   Description: {}", desc);
    }
    println!("   Use `cldev session end` when done");

    Ok(())
}

fn handle_end(force: bool) -> crate::core::Result<()> {
    use crate::core::{analyze_session, generate_level2_markdown, RecordLevel};

    let session_path = get_session_path();
    if !session_path.exists() {
        return Err(crate::core::CldevError::Config(
            "No active session found. Use `cldev session start` first.".to_string(),
        ));
    }

    // Load session
    let json = std::fs::read_to_string(&session_path)?;
    let ctx: SessionContext = serde_json::from_str(&json)?;

    println!("\n📊 Session Analysis");
    println!("   Duration: {} minutes", ctx.duration_minutes());
    println!("   Commands: {}", ctx.command_history.len());
    println!("   Errors: {}", ctx.errors_encountered.len());
    println!("   Files: {}", ctx.unique_files_modified());

    // Analyze session and recommend
    let recommendation = analyze_session(&ctx);
    println!("\n💡 Auto-record Recommendation");
    println!("   Score: {:.1}%", recommendation.score * 100.0);
    println!("   Level: {:?}", recommendation.level);
    println!("   Reason: {}", recommendation.reason);

    // Handle based on level
    match recommendation.level {
        RecordLevel::Full => {
            if force || prompt_user("Create learning record?")? {
                let record = generate_level2_markdown(&ctx, &recommendation);
                save_learning_record(&record)?;
                println!("\n✅ Learning record created: {}", record.id);
            } else {
                println!("\n⏭️  Skipped learning record creation");
            }
        }
        RecordLevel::Background => {
            println!("\n🔄 Background indexing (no manual record needed)");
            // TODO: Phase 7-3 - Background indexing
        }
        RecordLevel::Skip => {
            println!("\n⏭️  Session too short - no record needed");
        }
    }

    // Clean up session file
    std::fs::remove_file(&session_path)?;
    println!("\n✅ Session ended");

    Ok(())
}

fn prompt_user(message: &str) -> crate::core::Result<bool> {
    use std::io::{self, Write};

    print!("{} (y/n): ", message);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_lowercase() == "y")
}

fn save_learning_record(record: &crate::core::LearningRecordV3) -> crate::core::Result<()> {
    use std::fs;

    // Get learning records directory
    let home = dirs::home_dir().ok_or_else(|| {
        crate::core::CldevError::Config("Could not determine home directory".to_string())
    })?;

    let lr_dir = home.join(".cldev").join("learning-records");
    fs::create_dir_all(&lr_dir)?;

    // Save markdown file
    let filename = format!("{}.md", record.id);
    let file_path = lr_dir.join(filename);

    let markdown = record.to_markdown_file();
    fs::write(&file_path, markdown)?;

    println!("   Saved to: {}", file_path.display());

    Ok(())
}

fn handle_status() -> crate::core::Result<()> {
    let session_path = get_session_path();
    if !session_path.exists() {
        println!("ℹ️  No active session");
        println!("   Use `cldev session start` to begin");
        return Ok(());
    }

    // Load session
    let json = std::fs::read_to_string(&session_path)?;
    let ctx: SessionContext = serde_json::from_str(&json)?;

    println!("\n📊 Active Session");
    println!("   ID: {}", ctx.session_id);
    println!("   Duration: {} minutes", ctx.duration_minutes());
    println!("   Commands: {}", ctx.command_history.len());
    println!("   Errors: {}", ctx.errors_encountered.len());
    println!("   Files: {}", ctx.unique_files_modified());
    println!("   Todos: {}", ctx.completed_todos_count());

    Ok(())
}

fn get_session_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".cldev")
        .join("current-session.json")
}
