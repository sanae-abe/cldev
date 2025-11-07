//! E2E tests for feature development workflow
//!
//! Tests the complete /feature command workflow including
//! branch creation, implementation, testing, and documentation.

use cldev::core::config::Config;
use cldev::core::error::Result;
use cldev::core::git_utils::GitUtils;
use cldev::core::session_recorder::LearningSession;
use git2::Repository;
use std::fs;
use tempfile::TempDir;

fn setup_feature_env() -> Result<TempDir> {
    let temp_dir = TempDir::new()?;
    let repo_path = temp_dir.path();

    // Initialize Git repository
    let repo = Repository::init(repo_path)?;
    let mut config = repo.config()?;
    config.set_str("user.name", "Test User")?;
    config.set_str("user.email", "test@example.com")?;

    // Create initial commit
    let file_path = repo_path.join("README.md");
    fs::write(&file_path, "# Feature Test Project\n")?;

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
    cldev_config.dev.auto_create_branch = true;
    cldev_config.dev.branch_prefix = "feature".to_string();
    cldev_config.save(Some(config_dir.join("config.toml")))?;

    Ok(temp_dir)
}

#[test]
fn test_feature_workflow_complete() -> Result<()> {
    let temp_dir = setup_feature_env()?;
    std::env::set_var("HOME", temp_dir.path());

    let git_utils = GitUtils::open(temp_dir.path())?;

    // Step 1: Create feature branch
    git_utils.create_branch("feature/user-authentication")?;
    assert_eq!(git_utils.current_branch()?, "feature/user-authentication");

    // Step 2: Start learning session
    let mut session = LearningSession::new("feature", "Implement user authentication system");
    session.add_tag("authentication");
    session.add_tag("security");

    // Step 3: Implementation phase
    session.add_step("Designed authentication flow");
    session.add_step("Implemented JWT token generation");
    session.add_step("Added password hashing with bcrypt");

    // Step 4: File tracking
    session.add_file("src/auth/mod.rs");
    session.add_file("src/auth/jwt.rs");
    session.add_file("src/auth/password.rs");
    session.add_file("tests/auth/integration_tests.rs");

    // Step 5: Learnings during implementation
    session.add_learning("JWT tokens should have expiration");
    session.add_learning("Use bcrypt for password hashing");
    session.add_learning("Always validate user input");

    // Step 6: Mark complete
    session.set_solution("Implemented secure authentication with JWT and bcrypt");
    session.mark_resolved(Some(240)); // 4 hours

    // Step 7: Save session
    let path = session.save()?;
    assert!(path.exists());

    // Verify session
    let loaded = LearningSession::load(&session.id)?;
    assert_eq!(loaded.session_type, "feature");
    assert!(loaded.resolved);
    assert_eq!(loaded.files_affected.len(), 4);

    Ok(())
}

#[test]
fn test_feature_with_config_branch_naming() -> Result<()> {
    let temp_dir = setup_feature_env()?;
    let config_path = temp_dir.path().join(".config/cldev/config.toml");

    // Load config
    let config = Config::load(Some(config_path))?;

    // Verify auto_create_branch is enabled
    assert!(config.dev.auto_create_branch);
    assert_eq!(config.dev.branch_prefix, "feature");

    // Create branch using config
    let git_utils = GitUtils::open(temp_dir.path())?;
    let branch_name = format!("{}/new-feature", config.dev.branch_prefix);
    git_utils.create_branch(&branch_name)?;

    assert_eq!(git_utils.current_branch()?, branch_name);

    Ok(())
}

#[test]
fn test_feature_incremental_development() -> Result<()> {
    let temp_dir = setup_feature_env()?;
    std::env::set_var("HOME", temp_dir.path());

    // Simulate incremental feature development
    let mut session = LearningSession::new("feature", "API endpoints for user management");

    // Phase 1: User registration
    session.add_step("Implemented POST /api/register endpoint");
    session.add_file("src/api/register.rs");
    session.add_learning("Validate email format before registration");

    // Phase 2: User login
    session.add_step("Implemented POST /api/login endpoint");
    session.add_file("src/api/login.rs");
    session.add_learning("Return JWT token on successful login");

    // Phase 3: User profile
    session.add_step("Implemented GET /api/profile endpoint");
    session.add_file("src/api/profile.rs");
    session.add_learning("Protect profile endpoint with JWT auth");

    // Phase 4: Testing
    session.add_step("Added integration tests for all endpoints");
    session.add_file("tests/api/user_tests.rs");

    // Complete
    session.mark_resolved(Some(180));
    session.save()?;

    // Verify all phases documented
    let loaded = LearningSession::load(&session.id)?;
    assert_eq!(loaded.steps_taken.len(), 4);
    assert_eq!(loaded.files_affected.len(), 4);
    assert_eq!(loaded.learnings.len(), 3);

    Ok(())
}

#[test]
fn test_feature_with_testing_workflow() -> Result<()> {
    let temp_dir = setup_feature_env()?;
    std::env::set_var("HOME", temp_dir.path());

    let mut session = LearningSession::new("feature", "Payment processing integration");

    // Implementation
    session.add_step("Implemented payment API client");
    session.add_file("src/payment/client.rs");

    // Unit testing
    session.add_step("Wrote unit tests for payment client");
    session.add_file("tests/payment/client_tests.rs");
    session.add_learning("Mock external APIs in unit tests");

    // Integration testing
    session.add_step("Added integration tests with test API");
    session.add_file("tests/payment/integration_tests.rs");
    session.add_learning("Use test mode for payment API");

    // Documentation
    session.add_step("Documented payment flow in README");
    session.add_file("docs/payment_integration.md");

    session.mark_resolved(Some(300));
    session.save()?;

    let loaded = LearningSession::load(&session.id)?;
    assert_eq!(loaded.files_affected.len(), 4);
    assert!(loaded
        .files_affected
        .contains(&"docs/payment_integration.md".to_string()));

    Ok(())
}

#[test]
fn test_feature_refactoring_during_development() -> Result<()> {
    let temp_dir = setup_feature_env()?;
    std::env::set_var("HOME", temp_dir.path());

    let mut session = LearningSession::new("feature", "Search functionality");

    // Initial implementation
    session.add_step("Implemented basic search");
    session.add_file("src/search/mod.rs");

    // Refactoring
    session.add_step("Refactored to use async search");
    session.add_file("src/search/async_search.rs");
    session.add_learning("Async search improves performance");

    // Optimization
    session.add_step("Added search result caching");
    session.add_file("src/search/cache.rs");
    session.add_learning("Cache frequent searches");

    session.mark_resolved(Some(120));
    session.save()?;

    let loaded = LearningSession::load(&session.id)?;
    assert_eq!(loaded.learnings.len(), 2);

    Ok(())
}

#[test]
fn test_feature_with_documentation_updates() -> Result<()> {
    let temp_dir = setup_feature_env()?;
    std::env::set_var("HOME", temp_dir.path());

    let mut session = LearningSession::new("feature", "REST API endpoints");

    // Implementation
    session.add_file("src/api/endpoints.rs");

    // Documentation
    session.add_step("Created API documentation");
    session.add_file("docs/api/README.md");
    session.add_file("docs/api/authentication.md");
    session.add_file("docs/api/endpoints.md");

    // Examples
    session.add_step("Added code examples");
    session.add_file("examples/api_usage.rs");

    session.add_learning("Always document public APIs");
    session.mark_resolved(Some(90));
    session.save()?;

    let loaded = LearningSession::load(&session.id)?;
    assert!(loaded.files_affected.len() >= 5);

    Ok(())
}

#[test]
fn test_feature_multi_developer_simulation() -> Result<()> {
    let temp_dir = setup_feature_env()?;
    std::env::set_var("HOME", temp_dir.path());

    // Developer 1: Backend
    let mut backend_session = LearningSession::new("feature", "Backend API");
    backend_session.add_tag("backend");
    backend_session.add_file("src/api/backend.rs");
    backend_session.save()?;

    // Developer 2: Frontend
    let mut frontend_session = LearningSession::new("feature", "Frontend UI");
    frontend_session.add_tag("frontend");
    frontend_session.add_file("src/ui/components.rs");
    frontend_session.save()?;

    // Verify both sessions exist
    let all_sessions = LearningSession::list_all()?;
    assert!(all_sessions.len() >= 2);

    Ok(())
}

#[test]
fn test_feature_with_technical_debt_tracking() -> Result<()> {
    let temp_dir = setup_feature_env()?;
    std::env::set_var("HOME", temp_dir.path());

    let mut session = LearningSession::new("feature", "Quick feature implementation");

    session.add_step("Implemented feature quickly");
    session.add_file("src/quick_feature.rs");

    // Track technical debt
    session.add_learning("TODO: Refactor to use better error handling");
    session.add_learning("TODO: Add comprehensive tests");
    session.add_learning("TODO: Optimize performance");

    session.add_metadata("tech_debt", "High");
    session.mark_resolved(Some(30));
    session.save()?;

    let loaded = LearningSession::load(&session.id)?;
    assert_eq!(loaded.metadata.get("tech_debt"), Some(&"High".to_string()));

    Ok(())
}

#[test]
fn test_feature_dependency_tracking() -> Result<()> {
    let temp_dir = setup_feature_env()?;
    std::env::set_var("HOME", temp_dir.path());

    let mut session = LearningSession::new("feature", "New library integration");

    session.add_step("Added dependencies to Cargo.toml");
    session.add_file("Cargo.toml");

    session.add_step("Integrated library in code");
    session.add_file("src/integration/mod.rs");

    session.add_learning("New dependency: tokio 1.0");
    session.add_learning("New dependency: serde 1.0");

    session.add_metadata("dependencies", "tokio, serde");
    session.mark_resolved(Some(60));
    session.save()?;

    let loaded = LearningSession::load(&session.id)?;
    assert!(loaded.metadata.contains_key("dependencies"));

    Ok(())
}

#[test]
fn test_feature_rollback_scenario() -> Result<()> {
    let temp_dir = setup_feature_env()?;
    std::env::set_var("HOME", temp_dir.path());

    let mut session = LearningSession::new("feature", "Failed feature attempt");

    session.add_step("Started implementation");
    session.add_step("Encountered major architectural issues");
    session.add_step("Decided to rollback and redesign");

    session.add_learning("Need to validate architecture before implementation");
    session.add_learning("Prototype complex features first");

    // Mark as not resolved (abandoned)
    session.add_metadata("status", "abandoned");
    session.save()?;

    let loaded = LearningSession::load(&session.id)?;
    assert!(!loaded.resolved);
    assert_eq!(
        loaded.metadata.get("status"),
        Some(&"abandoned".to_string())
    );

    Ok(())
}
