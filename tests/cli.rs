//! CLI tests module
//!
//! This module contains CLI E2E tests using assert_cmd
//! to verify command-line interface behavior.

#[path = "cli_tests/completion_test.rs"]
mod completion_test;

#[path = "cli_tests/config_commands_test.rs"]
mod config_commands_test;

#[path = "cli_tests/help_test.rs"]
mod help_test;

#[path = "cli_tests/version_test.rs"]
mod version_test;
