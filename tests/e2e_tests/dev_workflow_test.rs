//! E2E tests for development workflow commands
//!
//! Tests the complete workflows for /urgent, /fix, and /debug commands,
//! simulating real-world usage scenarios.

use cldev::core::config::Config;
use cldev::core::error::Result;
use cldev::core::session_recorder::LearningSession;
use git2::Repository;
use std::fs;
use tempfile::TempDir;

/// Helper to initialize a test environment with Git repository and config
fn setup_test_env() -> Result<TempDir> {
    let temp_dir = TempDir::new()?;
    let repo_path = temp_dir.path();

    // Initialize Git repository
    let repo = Repository::init(repo_path)?;
    let mut config = repo.config()?;
    config.set_str("user.name", "Test User")?;
    config.set_str("user.email", "test@example.com")?;

    // Create initial commit
    let file_path = repo_path.join("README.md");
    fs::write(&file_path, "# Test Project\n")?;

    let mut index = repo.index()?;
    index.add_path(std::path::Path::new("README.md"))?;
    index.write()?;

    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    let signature = repo.signature()?;

    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        "Initial commit",
        &tree,
        &[],
    )?;

    // Create cldev config
    let config_dir = repo_path.join(".config/cldev");
    fs::create_dir_all(&config_dir)?;

    let mut cldev_config = Config::default();
    cldev_config.save(Some(config_dir.join("config.toml")))?;

    Ok(temp_dir)
}

#[test]
fn test_urgent_workflow_simulation() -> Result<()> {
    let temp_dir = setup_test_env()?;
    std::env::set_var("HOME", temp_dir.path());

    // Simulate urgent incident workflow
    // 1. Create a learning session for the incident
    let mut session =
        LearningSession::new("urgent", "Production database connection pool exhausted");

    // 2. Add incident details
    session.add_tag("production");
    session.add_tag("database");
    session.add_tag("P0-Critical");

    // 3. Document investigation steps
    session.add_step("Checked monitoring dashboards");
    session.add_step("Reviewed database connection metrics");
    session.add_step("Identified connection leak in API service");

    // 4. Add affected files
    session.add_file("src/db/connection_pool.rs");
    session.add_file("src/api/handlers.rs");

    // 5. Set root cause
    session.set_root_cause("API handlers not releasing database connections");

    // 6. Set solution
    session.set_solution("Added proper connection cleanup using RAII pattern");

    // 7. Add learnings
    session.add_learning("Always use RAII for resource management");
    session.add_learning("Monitor connection pool metrics in production");

    // 8. Mark as resolved
    session.mark_resolved(Some(15));

    // 9. Save session
    let path = session.save()?;

    // Verify session was saved
    assert!(path.exists());

    // Verify session can be loaded
    let loaded = LearningSession::load(&session.id)?;
    assert_eq!(loaded.session_type, "urgent");
    assert!(loaded.resolved);
    assert_eq!(loaded.duration_minutes, Some(15));
    assert!(loaded.tags.contains(&"P0-Critical".to_string()));

    Ok(())
}

#[test]
fn test_fix_workflow_simulation() -> Result<()> {
    let temp_dir = setup_test_env()?;
    std::env::set_var("HOME", temp_dir.path());

    // Simulate bug fix workflow
    // 1. Create session for bug fix
    let mut session = LearningSession::new("fix", "User authentication fails with expired tokens");

    // 2. Add bug details
    session.add_tag("authentication");
    session.add_tag("security");
    session.add_tag("P1-High");

    // 3. Investigation steps
    session.add_step("Reproduced the issue locally");
    session.add_step("Analyzed JWT token validation logic");
    session.add_step("Identified token expiry check bug");

    // 4. Affected files
    session.add_file("src/auth/jwt.rs");
    session.add_file("tests/auth/jwt_tests.rs");

    // 5. Root cause
    session.set_root_cause("Token expiry was using wrong timezone");

    // 6. Solution
    session.set_solution("Fixed timezone handling to use UTC consistently");

    // 7. Learnings
    session.add_learning("Always use UTC for time comparisons");
    session.add_learning("Add timezone tests for time-sensitive logic");

    // 8. Metadata
    session.add_metadata("priority", "P1");
    session.add_metadata("user_impact", "High");

    // 9. Resolve
    session.mark_resolved(Some(45));

    // Save and verify
    let path = session.save()?;
    assert!(path.exists());

    let loaded = LearningSession::load(&session.id)?;
    assert_eq!(loaded.session_type, "fix");
    assert!(loaded.resolved);
    assert_eq!(loaded.steps_taken.len(), 3);

    Ok(())
}

#[test]
fn test_debug_workflow_simulation() -> Result<()> {
    let temp_dir = setup_test_env()?;
    std::env::set_var("HOME", temp_dir.path());

    // Simulate debugging workflow
    // 1. Create debug session
    let mut session = LearningSession::new("debug", "Application crashes with segmentation fault");

    // 2. Add debugging details
    session.add_tag("crash");
    session.add_tag("memory");
    session.add_tag("unsafe");

    // 3. Debugging steps
    session.add_step("Reproduced crash with debug build");
    session.add_step("Ran with Valgrind for memory analysis");
    session.add_step("Added debug logging around unsafe code");
    session.add_step("Used gdb to analyze stack trace");

    // 4. Files investigated
    session.add_file("src/ffi/bindings.rs");
    session.add_file("src/unsafe_utils.rs");

    // 5. Root cause
    session.set_root_cause("Null pointer dereference in FFI boundary");

    // 6. Solution
    session.set_solution("Added null checks before dereferencing FFI pointers");

    // 7. Learnings
    session.add_learning("Always validate FFI pointers before use");
    session.add_learning("Use debug tools (gdb, Valgrind) for memory issues");
    session.add_learning("Document safety invariants for unsafe code");

    // 8. Metadata
    session.add_metadata("tool", "gdb");
    session.add_metadata("severity", "P2");

    // 9. Mark resolved
    session.mark_resolved(Some(90));

    // Verify
    let path = session.save()?;
    assert!(path.exists());

    let loaded = LearningSession::load(&session.id)?;
    assert_eq!(loaded.session_type, "debug");
    assert_eq!(loaded.steps_taken.len(), 4);
    assert_eq!(loaded.learnings.len(), 3);

    Ok(())
}

#[test]
fn test_multi_session_workflow() -> Result<()> {
    let temp_dir = setup_test_env()?;
    std::env::set_var("HOME", temp_dir.path());

    // Simulate multiple related sessions
    // Session 1: Initial urgent response
    let mut session1 = LearningSession::new("urgent", "API rate limiting not working");
    let path1 = session1.save()?;

    // Session 2: Debug investigation
    let mut session2 = LearningSession::new("debug", "Investigating rate limiter logic");
    session2.add_tag("api");
    session2.add_tag("rate-limiting");
    let path2 = session2.save()?;

    // Session 3: Fix implementation
    let mut session3 = LearningSession::new("fix", "Fixed rate limiter algorithm");
    session3.add_tag("api");
    session3.mark_resolved(Some(30));
    let path3 = session3.save()?;

    // Verify all sessions exist
    assert!(path1.exists());
    assert!(path2.exists());
    assert!(path3.exists());

    // Verify we can find related sessions by tag
    let api_sessions = LearningSession::find_by_tag("api")?;
    assert_eq!(api_sessions.len(), 2);

    Ok(())
}

#[test]
fn test_unresolved_session_tracking() -> Result<()> {
    let temp_dir = setup_test_env()?;
    std::env::set_var("HOME", temp_dir.path());

    // Create unresolved session
    let mut session = LearningSession::new("debug", "Intermittent timeout issue");
    session.add_tag("investigation");
    session.add_step("Enabled verbose logging");
    session.add_step("Monitoring for pattern");

    // Don't mark as resolved
    let path = session.save()?;

    // Verify it's saved as unresolved
    let loaded = LearningSession::load(&session.id)?;
    assert!(!loaded.resolved);
    assert!(loaded.duration_minutes.is_none());

    Ok(())
}

#[test]
fn test_workflow_with_config_integration() -> Result<()> {
    let temp_dir = setup_test_env()?;
    let config_path = temp_dir.path().join(".config/cldev/config.toml");

    // Load config
    let config = Config::load(Some(config_path.clone()))?;

    // Verify session recording is enabled by default
    assert!(config.dev.session_recording);

    // Simulate workflow respecting config
    if config.dev.session_recording {
        std::env::set_var("HOME", temp_dir.path());

        let session = LearningSession::new("feature", "New feature implementation");
        let path = session.save()?;
        assert!(path.exists());
    }

    Ok(())
}

#[test]
fn test_session_query_by_type() -> Result<()> {
    let temp_dir = setup_test_env()?;
    std::env::set_var("HOME", temp_dir.path());

    // Create multiple sessions of different types
    LearningSession::new("urgent", "Issue 1").save()?;
    std::thread::sleep(std::time::Duration::from_millis(2));

    LearningSession::new("urgent", "Issue 2").save()?;
    std::thread::sleep(std::time::Duration::from_millis(2));

    LearningSession::new("debug", "Investigation").save()?;
    std::thread::sleep(std::time::Duration::from_millis(2));

    LearningSession::new("fix", "Bug fix").save()?;

    // Query by type
    let urgent_sessions = LearningSession::find_by_type("urgent")?;
    assert_eq!(urgent_sessions.len(), 2);

    let debug_sessions = LearningSession::find_by_type("debug")?;
    assert_eq!(debug_sessions.len(), 1);

    Ok(())
}

#[test]
fn test_complete_incident_lifecycle() -> Result<()> {
    let temp_dir = setup_test_env()?;
    std::env::set_var("HOME", temp_dir.path());

    // Complete incident lifecycle from detection to resolution
    let mut session = LearningSession::new("urgent", "Service degradation - high response times");

    // Initial response (0-5 minutes)
    session.add_tag("production");
    session.add_tag("performance");
    session.add_tag("P1-High");
    session.add_step("Checked monitoring dashboards");
    session.add_step("Alerted team");

    // Investigation (5-30 minutes)
    session.add_step("Analyzed slow query logs");
    session.add_step("Identified missing database index");
    session.add_file("migrations/add_index.sql");

    // Root cause (30 minutes)
    session.set_root_cause("Missing index on frequently queried column");

    // Mitigation (30-45 minutes)
    session.add_step("Created database index");
    session.add_step("Verified query performance improved");

    // Resolution
    session.set_solution("Added index on user_id column");
    session.add_learning("Always analyze query patterns before deploying");
    session.add_learning("Monitor slow query logs proactively");
    session.mark_resolved(Some(45));

    // Metadata
    session.add_metadata("impact", "50% of users affected");
    session.add_metadata("downtime", "0 minutes");

    // Save and verify complete record
    let path = session.save()?;
    let loaded = LearningSession::load(&session.id)?;

    assert!(loaded.resolved);
    assert_eq!(loaded.steps_taken.len(), 6);
    assert_eq!(loaded.learnings.len(), 2);
    assert!(loaded.root_cause.is_some());
    assert!(loaded.solution.is_some());

    Ok(())
}
