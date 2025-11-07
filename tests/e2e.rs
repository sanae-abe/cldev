//! End-to-end tests module
//!
//! This module contains E2E tests that simulate complete user workflows
//! and verify the system works correctly from end to end.

#[path = "e2e_tests/dev_workflow_test.rs"]
mod dev_workflow_test;

#[path = "e2e_tests/feature_workflow_test.rs"]
mod feature_workflow_test;

#[path = "e2e_tests/git_workflow_test.rs"]
mod git_workflow_test;

#[path = "e2e_tests/quality_workflow_test.rs"]
mod quality_workflow_test;
