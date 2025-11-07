/// Quality management commands
///
/// This module provides commands for code quality assurance including:
/// - Linting (lint.rs)
/// - Code formatting (format.rs)
/// - Testing (test.rs)
mod format;
mod lint;
mod test;

pub use format::format_code;
pub use lint::run_lint;
pub use test::run_tests;
