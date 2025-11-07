//! Integration tests module
//!
//! This module contains integration tests that verify the interaction
//! between multiple components of the cldev system.

#[path = "integration_tests/config_integration_test.rs"]
mod config_integration_test;

#[path = "integration_tests/command_integration_test.rs"]
mod command_integration_test;

#[path = "integration_tests/git_integration_test.rs"]
mod git_integration_test;

#[path = "integration_tests/session_integration_test.rs"]
mod session_integration_test;
