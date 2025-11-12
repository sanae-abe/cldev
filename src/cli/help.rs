//! I18n support for clap help messages
//!
//! Provides internationalized help strings for all CLI commands, options, and arguments.
//! This module bridges clap's help system with the i18n message catalog.

#![allow(dead_code)]

use crate::core::i18n::{I18n, Language};
use std::sync::OnceLock;

/// Global I18n instance for help messages
static HELP_I18N: OnceLock<I18n> = OnceLock::new();

/// Initialize the help system with a specific language
pub fn init_help_i18n(language: Language) {
    HELP_I18N.get_or_init(|| I18n::with_language(language));
}

/// Get the I18n instance
/// This should be called only after init_help_i18n() has been called
fn get_i18n() -> &'static I18n {
    HELP_I18N
        .get()
        .expect("Help I18n not initialized - call init_help_i18n() first")
}

/// Get a help message by key
pub fn help(key: &str) -> &'static str {
    // We need to leak the string to get a 'static lifetime for clap
    Box::leak(get_i18n().get(key).into_boxed_str())
}

// ============================================================================
// Main CLI Help Strings
// ============================================================================

pub fn app_about() -> &'static str {
    help("app-about")
}

pub fn verbose_help() -> &'static str {
    help("opt-verbose")
}

pub fn quiet_help() -> &'static str {
    help("opt-quiet")
}

pub fn no_color_help() -> &'static str {
    help("opt-no-color")
}

pub fn lang_help() -> &'static str {
    help("opt-lang")
}

// ============================================================================
// Configuration Commands
// ============================================================================

pub fn config_about() -> &'static str {
    help("cmd-cat-configuration-desc")
}

pub fn config_init_about() -> &'static str {
    help("cmd-config-init-desc")
}

pub fn config_init_defaults_help() -> &'static str {
    help("opt-config-init-defaults")
}

pub fn config_init_force_help() -> &'static str {
    help("opt-config-init-force")
}

pub fn config_check_about() -> &'static str {
    help("cmd-config-check-desc")
}

pub fn config_check_detailed_help() -> &'static str {
    help("opt-config-check-detailed")
}

pub fn config_check_fix_help() -> &'static str {
    help("opt-config-check-fix")
}

pub fn config_edit_about() -> &'static str {
    help("cmd-config-edit-desc")
}

pub fn config_edit_target_help() -> &'static str {
    help("opt-config-edit-target")
}

pub fn config_list_about() -> &'static str {
    help("cmd-config-list-desc")
}

pub fn config_list_detailed_help() -> &'static str {
    help("opt-config-list-detailed")
}

pub fn config_list_filter_help() -> &'static str {
    help("opt-config-list-filter")
}

pub fn config_maintain_about() -> &'static str {
    help("cmd-config-maintain-desc")
}

pub fn config_maintain_backup_help() -> &'static str {
    help("opt-config-maintain-backup")
}

pub fn config_maintain_cleanup_help() -> &'static str {
    help("opt-config-maintain-cleanup")
}

pub fn config_maintain_archive_help() -> &'static str {
    help("opt-config-maintain-archive")
}

pub fn config_maintain_retention_days_help() -> &'static str {
    help("opt-config-maintain-retention-days")
}

pub fn config_update_docs_about() -> &'static str {
    help("cmd-config-update-docs-desc")
}

pub fn config_update_docs_type_help() -> &'static str {
    help("opt-config-update-docs-type")
}

pub fn config_update_docs_validate_help() -> &'static str {
    help("opt-config-update-docs-validate")
}

// ============================================================================
// Development Commands
// ============================================================================

pub fn dev_about() -> &'static str {
    help("cmd-cat-development-desc")
}

pub fn dev_urgent_about() -> &'static str {
    help("cmd-dev-urgent-desc")
}

pub fn dev_urgent_problem_help() -> &'static str {
    help("arg-dev-urgent-problem")
}

pub fn dev_urgent_yes_help() -> &'static str {
    help("opt-dev-urgent-yes")
}

pub fn dev_fix_about() -> &'static str {
    help("cmd-dev-fix-desc")
}

pub fn dev_debug_about() -> &'static str {
    help("cmd-dev-debug-desc")
}

pub fn dev_feature_about() -> &'static str {
    help("cmd-dev-feature-desc")
}

pub fn dev_refactor_about() -> &'static str {
    help("cmd-dev-refactor-desc")
}

pub fn dev_optimize_about() -> &'static str {
    help("cmd-dev-optimize-desc")
}

pub fn dev_research_about() -> &'static str {
    help("cmd-dev-research-desc")
}

pub fn dev_fix_target_help() -> &'static str {
    help("arg-dev-fix-target")
}

pub fn dev_fix_branch_help() -> &'static str {
    help("opt-dev-fix-branch")
}

pub fn dev_debug_symptom_help() -> &'static str {
    help("arg-dev-debug-symptom")
}

pub fn dev_debug_verbose_help() -> &'static str {
    help("opt-dev-debug-verbose")
}

pub fn dev_feature_name_help() -> &'static str {
    help("arg-dev-feature-name")
}

pub fn dev_feature_skip_confirm_help() -> &'static str {
    help("opt-dev-feature-skip-confirm")
}

pub fn dev_refactor_target_help() -> &'static str {
    help("arg-dev-refactor-target")
}

pub fn dev_refactor_scope_help() -> &'static str {
    help("opt-dev-refactor-scope")
}

pub fn dev_optimize_target_help() -> &'static str {
    help("arg-dev-optimize-target")
}

pub fn dev_optimize_focus_help() -> &'static str {
    help("opt-dev-optimize-focus")
}

pub fn dev_research_topic_help() -> &'static str {
    help("arg-dev-research-topic")
}

pub fn dev_research_format_help() -> &'static str {
    help("opt-dev-research-format")
}

// ============================================================================
// Git Commands
// ============================================================================

pub fn git_about() -> &'static str {
    help("cmd-cat-git-desc")
}

pub fn git_commit_about() -> &'static str {
    help("cmd-git-commit-desc")
}

pub fn git_commit_message_help() -> &'static str {
    help("arg-git-commit-message")
}

pub fn git_commit_no_verify_help() -> &'static str {
    help("opt-git-commit-no-verify")
}

pub fn git_commit_amend_help() -> &'static str {
    help("opt-git-commit-amend")
}

pub fn git_branch_about() -> &'static str {
    help("cmd-git-branch-desc")
}

pub fn git_branch_name_help() -> &'static str {
    help("arg-git-branch-name")
}

pub fn git_branch_type_help() -> &'static str {
    help("opt-git-branch-type")
}

pub fn git_merge_request_about() -> &'static str {
    help("cmd-git-merge-request-desc")
}

pub fn git_merge_request_target_help() -> &'static str {
    help("opt-git-mr-target")
}

pub fn git_merge_request_title_help() -> &'static str {
    help("arg-git-mr-title")
}

pub fn git_merge_request_detailed_help() -> &'static str {
    help("opt-git-mr-detailed")
}

pub fn git_status_about() -> &'static str {
    help("cmd-git-status-desc")
}

pub fn git_status_detailed_help() -> &'static str {
    help("opt-git-status-detailed")
}

// ============================================================================
// Quality Commands
// ============================================================================

pub fn quality_about() -> &'static str {
    help("cmd-cat-quality-desc")
}

pub fn quality_lint_about() -> &'static str {
    help("cmd-quality-lint-desc")
}

pub fn quality_lint_fix_help() -> &'static str {
    help("opt-quality-lint-fix")
}

pub fn quality_lint_paths_help() -> &'static str {
    help("arg-quality-lint-paths")
}

pub fn quality_format_about() -> &'static str {
    help("cmd-quality-format-desc")
}

pub fn quality_format_check_help() -> &'static str {
    help("opt-quality-format-check")
}

pub fn quality_format_paths_help() -> &'static str {
    help("arg-quality-format-paths")
}

pub fn quality_test_about() -> &'static str {
    help("cmd-quality-test-desc")
}

pub fn quality_test_pattern_help() -> &'static str {
    help("arg-quality-test-pattern")
}

pub fn quality_test_coverage_help() -> &'static str {
    help("opt-quality-test-coverage")
}

pub fn quality_test_watch_help() -> &'static str {
    help("opt-quality-test-watch")
}

// ============================================================================
// Tech Stack Commands
// ============================================================================

pub fn tech_about() -> &'static str {
    help("cmd-cat-tech-stack-desc")
}

pub fn tech_start_about() -> &'static str {
    help("cmd-tech-start-desc")
}

pub fn tech_start_stack_help() -> &'static str {
    help("arg-tech-start-stack")
}

pub fn tech_start_port_help() -> &'static str {
    help("opt-tech-start-port")
}

pub fn tech_start_env_help() -> &'static str {
    help("opt-tech-start-env")
}

// ============================================================================
// Operations Commands
// ============================================================================

pub fn ops_about() -> &'static str {
    help("cmd-cat-operations-desc")
}

pub fn ops_build_about() -> &'static str {
    help("cmd-ops-build-desc")
}

pub fn ops_build_env_help() -> &'static str {
    help("opt-ops-build-env")
}

pub fn ops_build_analyze_help() -> &'static str {
    help("opt-ops-build-analyze")
}

pub fn ops_build_clean_help() -> &'static str {
    help("opt-ops-build-clean")
}

pub fn ops_deploy_about() -> &'static str {
    help("cmd-ops-deploy-desc")
}

pub fn ops_deploy_env_help() -> &'static str {
    help("arg-ops-deploy-env")
}

pub fn ops_deploy_yes_help() -> &'static str {
    help("opt-ops-deploy-yes")
}

pub fn ops_deploy_dry_run_help() -> &'static str {
    help("opt-ops-deploy-dry-run")
}

// ============================================================================
// Analysis Commands
// ============================================================================

pub fn analysis_about() -> &'static str {
    help("cmd-cat-analysis-desc")
}

pub fn analysis_analyze_about() -> &'static str {
    help("cmd-analysis-analyze-desc")
}

pub fn analysis_analyze_target_help() -> &'static str {
    help("arg-analysis-analyze-target")
}

pub fn analysis_analyze_format_help() -> &'static str {
    help("opt-analysis-analyze-format")
}

pub fn analysis_analyze_detailed_help() -> &'static str {
    help("opt-analysis-analyze-detailed")
}

pub fn analysis_explain_about() -> &'static str {
    help("cmd-analysis-explain-desc")
}

pub fn analysis_explain_target_help() -> &'static str {
    help("arg-analysis-explain-target")
}

pub fn analysis_explain_examples_help() -> &'static str {
    help("opt-analysis-explain-examples")
}

pub fn analysis_explain_detailed_help() -> &'static str {
    help("opt-analysis-explain-detailed")
}

pub fn analysis_review_mr_about() -> &'static str {
    help("cmd-analysis-review-mr-desc")
}

pub fn analysis_review_mr_number_help() -> &'static str {
    help("arg-analysis-review-mr-number")
}

pub fn analysis_review_mr_detailed_help() -> &'static str {
    help("opt-analysis-review-mr-detailed")
}

pub fn analysis_review_mr_security_focus_help() -> &'static str {
    help("opt-analysis-review-mr-security-focus")
}

pub fn analysis_review_mr_performance_focus_help() -> &'static str {
    help("opt-analysis-review-mr-performance-focus")
}

pub fn analysis_serena_about() -> &'static str {
    help("cmd-analysis-serena-desc")
}

pub fn analysis_serena_mode_help() -> &'static str {
    help("opt-analysis-serena-mode")
}

pub fn analysis_serena_targets_help() -> &'static str {
    help("arg-analysis-serena-targets")
}

// ============================================================================
// Learning Record Commands
// ============================================================================

pub fn lr_about() -> &'static str {
    help("cmd-cat-learning-desc")
}

pub fn lr_find_about() -> &'static str {
    help("cmd-lr-find-desc")
}

pub fn lr_find_query_help() -> &'static str {
    help("arg-lr-find-query")
}

pub fn lr_find_field_help() -> &'static str {
    help("opt-lr-find-field")
}

pub fn lr_find_limit_help() -> &'static str {
    help("opt-lr-find-limit")
}

pub fn lr_stats_about() -> &'static str {
    help("cmd-lr-stats-desc")
}

pub fn lr_stats_period_help() -> &'static str {
    help("opt-lr-stats-period")
}

pub fn lr_stats_detailed_help() -> &'static str {
    help("opt-lr-stats-detailed")
}

pub fn lr_problems_about() -> &'static str {
    help("cmd-lr-problems-desc")
}

pub fn lr_problems_priority_help() -> &'static str {
    help("opt-lr-problems-priority")
}

pub fn lr_problems_recent_help() -> &'static str {
    help("opt-lr-problems-recent")
}

pub fn lr_new_about() -> &'static str {
    help("cmd-lr-new-desc")
}

pub fn lr_new_topic_help() -> &'static str {
    help("arg-lr-new-topic")
}

pub fn lr_new_edit_help() -> &'static str {
    help("opt-lr-new-edit")
}

pub fn lr_check_file_about() -> &'static str {
    help("cmd-lr-check-file-desc")
}

pub fn lr_check_file_path_help() -> &'static str {
    help("arg-lr-check-file-path")
}

pub fn lr_suggest_about() -> &'static str {
    help("cmd-lr-suggest-desc")
}

pub fn lr_suggest_error_help() -> &'static str {
    help("arg-lr-suggest-error")
}

pub fn lr_suggest_threshold_help() -> &'static str {
    help("opt-lr-suggest-threshold")
}

pub fn lr_suggest_limit_help() -> &'static str {
    help("opt-lr-suggest-limit")
}

pub fn lr_similar_about() -> &'static str {
    help("cmd-lr-similar-desc")
}

pub fn lr_similar_session_id_help() -> &'static str {
    help("arg-lr-similar-session-id")
}

pub fn lr_similar_limit_help() -> &'static str {
    help("opt-lr-similar-limit")
}

// ============================================================================
// Todo Commands
// ============================================================================

pub fn todo_about() -> &'static str {
    help("cmd-cat-todo-desc")
}

// ============================================================================
// Session Commands
// ============================================================================

pub fn session_about() -> &'static str {
    help("cmd-cat-session-desc")
}

pub fn todo_add_about() -> &'static str {
    help("cmd-todo-add-desc")
}

pub fn todo_add_description_help() -> &'static str {
    help("arg-todo-add-description")
}

pub fn todo_list_about() -> &'static str {
    help("cmd-todo-list-desc")
}

pub fn todo_complete_about() -> &'static str {
    help("cmd-todo-complete-desc")
}

pub fn todo_sync_about() -> &'static str {
    help("cmd-todo-sync-desc")
}

pub fn todo_interactive_about() -> &'static str {
    help("cmd-todo-interactive-desc")
}

// ============================================================================
// Completions
// ============================================================================

pub fn completions_about() -> &'static str {
    help("cmd-completions-desc")
}

pub fn completions_shell_help() -> &'static str {
    help("arg-completions-shell")
}

pub fn completions_install_help() -> &'static str {
    help("opt-completions-install")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_help_initialization() {
        init_help_i18n(Language::English);
        let msg = app_about();
        assert!(!msg.is_empty());
    }

    #[test]
    fn test_help_functions() {
        // Initialize i18n before testing help functions
        init_help_i18n(Language::English);

        // Test a few representative help functions
        assert!(!app_about().is_empty());
        assert!(!config_about().is_empty());
        assert!(!dev_about().is_empty());
        assert!(!git_about().is_empty());
    }
}
