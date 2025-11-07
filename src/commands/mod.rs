/// Command implementations
pub mod analysis;
pub mod config;
pub mod dev;
pub mod git;
pub mod lr;
pub mod ops;
pub mod quality;
pub mod tech;
pub mod todo;

pub use analysis::{analyze_project, explain_target, review_merge_request, run_serena};
pub use config::{check_config, edit_config, list_commands};
pub use dev::{
    handle_debug, handle_feature, handle_fix, handle_optimize, handle_refactor, handle_research,
    handle_urgent,
};
pub use git::{create_branch, create_commit, create_merge_request, show_status};
pub use lr::{handle_find, handle_new, handle_problems, handle_stats};
pub use ops::{handle_build, handle_deploy};
pub use quality::{format_code, run_lint, run_tests};
pub use tech::handle_start;
pub use todo::handle_manage;
