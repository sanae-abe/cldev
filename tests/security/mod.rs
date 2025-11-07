//! Security integration tests
//!
//! This module contains comprehensive security tests covering:
//! - Path traversal prevention
//! - Command injection prevention
//! - File permission validation
//!
//! These tests ensure the security module provides robust protection
//! against common attack vectors.

mod command_injection_tests;
mod file_permissions_tests;
mod path_traversal_tests;
