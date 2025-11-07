//! Git command implementations
//!
//! This module provides Git-related commands including:
//! - Conventional commits with emoji support
//! - Conventional branch naming
//! - GitHub/GitLab merge request creation
//! - Enhanced Git status with recommendations

mod branch;
mod commit;
mod merge_request;
mod status;

pub use branch::create_branch;
pub use commit::create_commit;
pub use merge_request::create_merge_request;
pub use status::show_status;
