//! Integration tests for learning session recording
//!
//! Tests the full lifecycle of learning session recording,
//! including creation, saving, loading, and querying.

use cldev::core::error::Result;
use cldev::core::session_recorder::{LearningSession, LearningSessionBuilder};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_session_creation_and_basic_fields() -> Result<()> {
    let session = LearningSession::new("urgent", "Critical production bug");

    assert_eq!(session.session_type, "urgent");
    assert_eq!(session.description, "Critical production bug");
    assert!(!session.resolved);
    assert!(session.root_cause.is_none());
    assert!(session.solution.is_none());
    assert!(session.duration_minutes.is_none());
    assert!(session.tags.is_empty());
    assert!(session.learnings.is_empty());
    assert!(session.files_affected.is_empty());
    assert!(session.steps_taken.is_empty());

    Ok(())
}

#[test]
fn test_session_builder_fluent_api() -> Result<()> {
    let session = LearningSessionBuilder::new("debug", "Memory leak investigation")
        .tag("performance")
        .tag("memory")
        .learning("Always check resource cleanup")
        .learning("Use memory profiler for leak detection")
        .file("src/main.rs")
        .file("src/utils.rs")
        .step("Ran memory profiler")
        .step("Identified leaked objects")
        .root_cause("Missing drop implementation")
        .solution("Added proper Drop trait implementation")
        .resolved(Some(45))
        .metadata("severity", "P2")
        .build();

    assert_eq!(session.session_type, "debug");
    assert_eq!(session.tags.len(), 2);
    assert_eq!(session.learnings.len(), 2);
    assert_eq!(session.files_affected.len(), 2);
    assert_eq!(session.steps_taken.len(), 2);
    assert!(session.root_cause.is_some());
    assert!(session.solution.is_some());
    assert!(session.resolved);
    assert_eq!(session.duration_minutes, Some(45));
    assert_eq!(session.metadata.get("severity"), Some(&"P2".to_string()));

    Ok(())
}

#[test]
fn test_session_save_and_load() -> Result<()> {
    // Create a temporary directory for sessions
    let temp_dir = TempDir::new()?;
    std::env::set_var("HOME", temp_dir.path());

    let session = LearningSessionBuilder::new("feature", "New authentication system")
        .tag("security")
        .tag("authentication")
        .learning("JWT tokens require proper validation")
        .file("src/auth.rs")
        .resolved(Some(120))
        .build();

    // Save session
    let session_id = session.id.clone();
    let saved_path = session.save()?;

    assert!(saved_path.exists());

    // Load session
    let loaded = LearningSession::load(&session_id)?;

    assert_eq!(loaded.id, session_id);
    assert_eq!(loaded.session_type, "feature");
    assert_eq!(loaded.tags.len(), 2);
    assert_eq!(loaded.learnings.len(), 1);
    assert_eq!(loaded.files_affected.len(), 1);
    assert!(loaded.resolved);
    assert_eq!(loaded.duration_minutes, Some(120));

    Ok(())
}

#[test]
fn test_session_list_all() -> Result<()> {
    let temp_dir = TempDir::new()?;
    std::env::set_var("HOME", temp_dir.path());

    // Create multiple sessions
    let session1 = LearningSession::new("urgent", "Production issue 1");
    let session2 = LearningSession::new("debug", "Bug investigation");
    let session3 = LearningSession::new("feature", "New feature");

    session1.save()?;
    session2.save()?;
    session3.save()?;

    // List all sessions
    let sessions = LearningSession::list_all()?;

    assert_eq!(sessions.len(), 3);
    assert!(sessions.iter().any(|id| id.starts_with("urgent_")));
    assert!(sessions.iter().any(|id| id.starts_with("debug_")));
    assert!(sessions.iter().any(|id| id.starts_with("feature_")));

    Ok(())
}

#[test]
fn test_session_find_by_tag() -> Result<()> {
    let temp_dir = TempDir::new()?;
    std::env::set_var("HOME", temp_dir.path());

    // Create sessions with different tags
    let session1 = LearningSessionBuilder::new("urgent", "Production bug")
        .tag("production")
        .tag("security")
        .build();

    let session2 = LearningSessionBuilder::new("debug", "Performance issue")
        .tag("performance")
        .tag("optimization")
        .build();

    let session3 = LearningSessionBuilder::new("feature", "Security feature")
        .tag("security")
        .tag("authentication")
        .build();

    session1.save()?;
    session2.save()?;
    session3.save()?;

    // Find sessions by tag
    let security_sessions = LearningSession::find_by_tag("security")?;
    assert_eq!(security_sessions.len(), 2);

    let performance_sessions = LearningSession::find_by_tag("performance")?;
    assert_eq!(performance_sessions.len(), 1);

    let nonexistent_sessions = LearningSession::find_by_tag("nonexistent")?;
    assert_eq!(nonexistent_sessions.len(), 0);

    Ok(())
}

#[test]
fn test_session_find_by_type() -> Result<()> {
    let temp_dir = TempDir::new()?;
    std::env::set_var("HOME", temp_dir.path());

    // Create sessions of different types
    let session1 = LearningSession::new("urgent", "Urgent issue 1");
    let session2 = LearningSession::new("urgent", "Urgent issue 2");
    let session3 = LearningSession::new("debug", "Debug session");
    let session4 = LearningSession::new("feature", "Feature work");

    session1.save()?;
    session2.save()?;
    session3.save()?;
    session4.save()?;

    // Find by type
    let urgent_sessions = LearningSession::find_by_type("urgent")?;
    assert_eq!(urgent_sessions.len(), 2);

    let debug_sessions = LearningSession::find_by_type("debug")?;
    assert_eq!(debug_sessions.len(), 1);

    let feature_sessions = LearningSession::find_by_type("feature")?;
    assert_eq!(feature_sessions.len(), 1);

    Ok(())
}

#[test]
fn test_session_builder_save_directly() -> Result<()> {
    let temp_dir = TempDir::new()?;
    std::env::set_var("HOME", temp_dir.path());

    let (session, path) = LearningSessionBuilder::new("refactor", "Code cleanup")
        .tag("code-quality")
        .learning("Refactoring improves maintainability")
        .resolved(Some(30))
        .save()?;

    assert!(path.exists());
    assert_eq!(session.session_type, "refactor");
    assert!(session.resolved);

    // Verify we can load it back
    let loaded = LearningSession::load(&session.id)?;
    assert_eq!(loaded.session_type, "refactor");

    Ok(())
}

#[test]
fn test_session_mutation_methods() -> Result<()> {
    let mut session = LearningSession::new("optimize", "Performance optimization");

    // Test mutation methods
    session.add_tag("performance");
    session.add_tag("optimization");
    session.add_learning("Profiling revealed bottleneck");
    session.add_file("src/performance.rs");
    session.add_step("Ran profiler");
    session.set_root_cause("Inefficient algorithm");
    session.set_solution("Used better data structure");
    session.mark_resolved(Some(60));
    session.add_metadata("tool", "cargo-flamegraph");

    assert_eq!(session.tags.len(), 2);
    assert_eq!(session.learnings.len(), 1);
    assert_eq!(session.files_affected.len(), 1);
    assert_eq!(session.steps_taken.len(), 1);
    assert_eq!(
        session.root_cause,
        Some("Inefficient algorithm".to_string())
    );
    assert_eq!(
        session.solution,
        Some("Used better data structure".to_string())
    );
    assert!(session.resolved);
    assert_eq!(session.duration_minutes, Some(60));
    assert_eq!(
        session.metadata.get("tool"),
        Some(&"cargo-flamegraph".to_string())
    );

    Ok(())
}

#[test]
fn test_session_add_multiple_tags() -> Result<()> {
    let mut session = LearningSession::new("research", "New technology");

    let tags = vec!["rust".to_string(), "async".to_string(), "tokio".to_string()];

    session.add_tags(tags.clone());

    assert_eq!(session.tags, tags);

    Ok(())
}

#[test]
fn test_session_complex_workflow() -> Result<()> {
    let temp_dir = TempDir::new()?;
    std::env::set_var("HOME", temp_dir.path());

    // Simulate a complex debugging workflow
    let mut session = LearningSession::new("debug", "Memory leak in production");

    // Initial investigation
    session.add_tag("production");
    session.add_tag("memory");
    session.add_tag("critical");

    // Investigation steps
    session.add_step("Checked monitoring dashboards");
    session.add_step("Analyzed heap dumps");
    session.add_step("Reviewed recent deployments");

    // Files affected
    session.add_file("src/cache.rs");
    session.add_file("src/connection_pool.rs");

    // Root cause identified
    session.set_root_cause("Connection pool not releasing resources");

    // Solution implemented
    session.set_solution("Added proper connection cleanup in drop");

    // Learnings
    session.add_learning("Always implement Drop for resource-owning types");
    session.add_learning("Monitor resource usage in production");

    // Metadata
    session.add_metadata("severity", "P1");
    session.add_metadata("impact", "High memory usage");

    // Mark as resolved
    session.mark_resolved(Some(180));

    // Save
    let path = session.save()?;
    assert!(path.exists());

    // Load and verify
    let loaded = LearningSession::load(&session.id)?;
    assert_eq!(loaded.tags.len(), 3);
    assert_eq!(loaded.steps_taken.len(), 3);
    assert_eq!(loaded.files_affected.len(), 2);
    assert_eq!(loaded.learnings.len(), 2);
    assert!(loaded.resolved);
    assert_eq!(loaded.duration_minutes, Some(180));

    Ok(())
}

#[test]
fn test_session_json_serialization() -> Result<()> {
    let temp_dir = TempDir::new()?;
    std::env::set_var("HOME", temp_dir.path());

    let session = LearningSessionBuilder::new("test", "JSON test")
        .tag("serialization")
        .learning("JSON works correctly")
        .build();

    let path = session.save()?;

    // Read raw JSON
    let json_content = fs::read_to_string(&path)?;

    // Verify JSON structure
    assert!(json_content.contains("\"session_type\""));
    assert!(json_content.contains("\"description\""));
    assert!(json_content.contains("\"tags\""));
    assert!(json_content.contains("\"learnings\""));

    Ok(())
}

#[test]
fn test_session_id_format() -> Result<()> {
    let session = LearningSession::new("urgent", "Test");

    // ID should be in format: type_YYYYMMDD_HHMMSS
    assert!(session.id.starts_with("urgent_"));

    let parts: Vec<&str> = session.id.split('_').collect();
    assert_eq!(parts.len(), 3); // type, date, time

    Ok(())
}

#[test]
fn test_session_timestamp_format() -> Result<()> {
    let session = LearningSession::new("test", "Timestamp test");

    // Timestamp should be in format: YYYY-MM-DD HH:MM:SS
    assert!(session.timestamp.contains('-'));
    assert!(session.timestamp.contains(':'));

    let parts: Vec<&str> = session.timestamp.split(' ').collect();
    assert_eq!(parts.len(), 2); // date and time

    Ok(())
}
