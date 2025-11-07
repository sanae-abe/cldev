/// Development workflow commands
///
/// This module contains commands for managing critical development workflows:
/// - urgent: Production incident response (5-minute initial response)
/// - fix: Important bug resolution (same-day target)
/// - debug: Systematic debugging framework
/// - feature: New feature implementation workflow
/// - refactor: Safe code refactoring with impact analysis
/// - optimize: Performance optimization with benchmarking
/// - research: Technical research and learning sessions
///
/// All commands integrate with the learning session recorder to capture
/// insights and build knowledge over time.
pub mod debug;
pub mod feature;
pub mod fix;
pub mod optimize;
pub mod refactor;
pub mod research;
pub mod urgent;

pub use debug::handle_debug;
pub use feature::handle_feature;
pub use fix::handle_fix;
pub use optimize::handle_optimize;
pub use refactor::handle_refactor;
pub use research::handle_research;
pub use urgent::handle_urgent;
