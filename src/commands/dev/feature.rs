use crate::cli::output::OutputHandler;
use crate::core::session_recorder::LearningSessionBuilder;
use crate::core::{GitUtils, Result};
use colored::*;
use dialoguer::{Confirm, Input, MultiSelect, Select};
use std::time::Instant;

/// New feature implementation workflow
///
/// This command provides a structured approach for implementing new features:
/// - Requirements gathering and clarification
/// - Design planning
/// - Git branch creation (feature/name)
/// - Step-by-step implementation with TodoWrite integration
/// - Testing and documentation
pub fn handle_feature(name: Option<String>, output: &OutputHandler) -> Result<()> {
    let start_time = Instant::now();

    println!("{}", output.t("feature-header").green().bold());
    println!("{}", output.t("feature-separator").green());
    println!();

    // Step 1: Feature Name and Description
    let feature_name = if let Some(n) = name {
        n
    } else {
        Input::<String>::new()
            .with_prompt(output.t("feature-name-prompt"))
            .interact_text()?
    };

    println!();

    let feature_desc = Input::<String>::new()
        .with_prompt(output.t("feature-description-prompt"))
        .interact_text()?;

    println!();

    // Step 2: Requirements Gathering
    println!("{}", output.t("feature-requirements-header").cyan().bold());
    println!();

    let requirements_items = [
        "What problem does this solve?",
        "Who are the users/stakeholders?",
        "What are the acceptance criteria?",
        "Are there any constraints or dependencies?",
        "What is the expected timeline?",
    ];

    println!("Key questions to answer:");
    for (i, item) in requirements_items.iter().enumerate() {
        println!("  {}. {}", i + 1, item);
    }

    println!();

    let problem_statement = Input::<String>::new()
        .with_prompt(output.t("feature-problem-prompt"))
        .interact_text()?;

    println!();

    let target_users = Input::<String>::new()
        .with_prompt(output.t("feature-users-prompt"))
        .interact_text()?;

    println!();

    // Step 3: Acceptance Criteria
    println!("{}", "✅ ACCEPTANCE CRITERIA".cyan().bold());
    println!("Enter acceptance criteria (press Enter twice when done):");
    println!();

    let mut acceptance_criteria = Vec::new();
    let mut criterion_num = 1;

    loop {
        let criterion = Input::<String>::new()
            .with_prompt(
                output
                    .t("feature-criterion-prompt")
                    .replace("{num}", &criterion_num.to_string()),
            )
            .allow_empty(true)
            .interact_text()?;

        if criterion.is_empty() {
            break;
        }

        acceptance_criteria.push(criterion);
        criterion_num += 1;
    }

    if acceptance_criteria.is_empty() {
        println!("{}", "⚠️  Warning: No acceptance criteria defined".yellow());
        println!();
    }

    // Step 4: Feature Type Classification
    println!();
    println!("{}", "🏷️  FEATURE CLASSIFICATION".cyan().bold());
    println!();

    let feature_types = vec![
        output.t("feature-type-ui"),
        output.t("feature-type-api"),
        output.t("feature-type-data"),
        output.t("feature-type-algorithm"),
        output.t("feature-type-integration"),
        output.t("feature-type-performance"),
        output.t("feature-type-tooling"),
        output.t("feature-type-security"),
        output.t("feature-type-accessibility"),
        output.t("feature-type-other"),
    ];

    let type_idx = Select::new()
        .with_prompt(output.t("feature-type-prompt"))
        .items(&feature_types)
        .interact()?;

    let feature_type = feature_types[type_idx].clone();

    println!();

    // Step 5: Complexity Estimation
    let complexity_levels = vec![
        output.t("feature-complexity-small"),
        output.t("feature-complexity-medium"),
        output.t("feature-complexity-large"),
        output.t("feature-complexity-xlarge"),
    ];

    let complexity_idx = Select::new()
        .with_prompt(output.t("feature-complexity-prompt"))
        .items(&complexity_levels)
        .interact()?;

    let complexity = complexity_levels[complexity_idx].clone();

    println!();

    // Step 6: Git Branch Creation
    println!("{}", "🌿 GIT BRANCH SETUP".cyan().bold());
    println!();

    let create_branch = Confirm::new()
        .with_prompt(output.t("feature-branch-create"))
        .default(true)
        .interact()?;

    let branch_name = if create_branch {
        let suggested_branch = format!("feature/{}", feature_name.to_lowercase().replace(" ", "-"));

        let branch_name = Input::<String>::new()
            .with_prompt(output.t("feature-branch-name"))
            .default(suggested_branch)
            .interact_text()?;

        // Check if we're in a git repository
        if let Ok(git_utils) = GitUtils::open_current() {
            match git_utils.create_branch(&branch_name) {
                Ok(_) => {
                    println!(
                        "{}",
                        format!("✅ Created and switched to branch: {}", branch_name).green()
                    );
                    Some(branch_name)
                }
                Err(e) => {
                    println!("{}", format!("⚠️  Failed to create branch: {}", e).yellow());
                    println!(
                        "   You can create it manually: git checkout -b {}",
                        branch_name
                    );
                    None
                }
            }
        } else {
            println!("{}", "⚠️  Not in a git repository".yellow());
            println!("   Branch will not be created");
            None
        }
    } else {
        None
    };

    println!();

    // Step 7: Design Planning
    println!("{}", "🎨 DESIGN PLANNING".cyan().bold());
    println!();

    let design_considerations = vec![
        output.t("feature-design-ui"),
        output.t("feature-design-schema"),
        output.t("feature-design-api"),
        output.t("feature-design-state"),
        output.t("feature-design-architecture"),
        output.t("feature-design-security"),
        output.t("feature-design-performance"),
        output.t("feature-design-testing"),
        output.t("feature-design-docs"),
        output.t("feature-design-accessibility"),
    ];

    let selected_design_items = MultiSelect::new()
        .with_prompt(output.t("feature-design-prompt"))
        .items(&design_considerations)
        .interact()?;

    let design_items: Vec<String> = selected_design_items
        .iter()
        .map(|&i| design_considerations[i].to_string())
        .collect();

    println!();

    // Step 8: Implementation Plan - Files to Create/Modify
    println!("{}", "📁 FILES TO CREATE/MODIFY".cyan().bold());
    println!("Enter file paths (one per line, press Enter twice when done):");
    println!();

    let mut files = Vec::new();
    loop {
        let file = Input::<String>::new()
            .with_prompt(output.t("feature-file-prompt"))
            .allow_empty(true)
            .interact_text()?;

        if file.is_empty() {
            break;
        }

        files.push(file);
    }

    println!();

    // Step 9: Dependencies
    println!("{}", "📦 DEPENDENCIES".cyan().bold());
    println!();

    let has_new_deps = Confirm::new()
        .with_prompt(output.t("feature-dependencies-question"))
        .default(false)
        .interact()?;

    let mut dependencies = Vec::new();
    if has_new_deps {
        println!();
        println!("Enter dependencies (package names, press Enter twice when done):");
        println!();

        loop {
            let dep = Input::<String>::new()
                .with_prompt(output.t("feature-dependency-prompt"))
                .allow_empty(true)
                .interact_text()?;

            if dep.is_empty() {
                break;
            }

            dependencies.push(dep);
        }
    }

    println!();

    // Step 10: Implementation Phases
    println!("{}", "📝 IMPLEMENTATION TODO LIST".green().bold());
    println!();

    let mut todo_items = Vec::new();

    // Add design phase if needed
    if !design_items.is_empty() {
        todo_items.push("Complete design phase (wireframes, API contracts, etc.)".to_string());
    }

    // Add dependencies installation
    if !dependencies.is_empty() {
        todo_items.push(format!("Install dependencies: {}", dependencies.join(", ")));
    }

    // Add file creation/modification
    for file in &files {
        todo_items.push(format!("Implement: {}", file));
    }

    // Add standard phases
    todo_items.push("Write unit tests for new functionality".to_string());
    todo_items.push("Write integration tests (if applicable)".to_string());
    todo_items.push("Update documentation (README, API docs, etc.)".to_string());
    todo_items.push("Perform manual testing against acceptance criteria".to_string());
    todo_items.push("Code review preparation (self-review)".to_string());

    println!("Generated TODO list:");
    for (i, item) in todo_items.iter().enumerate() {
        println!("  {}. {}", i + 1, item.dimmed());
    }

    println!();
    println!(
        "{}",
        "💡 Use TodoWrite in Claude Code to track these tasks!".cyan()
    );
    println!();

    // Step 11: Testing Strategy
    println!("{}", "🧪 TESTING STRATEGY".cyan().bold());
    println!();

    let test_types = vec![
        output.t("feature-test-unit"),
        output.t("feature-test-integration"),
        output.t("feature-test-e2e"),
        output.t("feature-test-visual"),
        output.t("feature-test-performance"),
        output.t("feature-test-accessibility"),
        output.t("feature-test-security"),
        output.t("feature-test-manual"),
    ];

    let selected_tests = MultiSelect::new()
        .with_prompt(output.t("feature-test-prompt"))
        .items(&test_types)
        .interact()?;

    let tests_required: Vec<String> = selected_tests
        .iter()
        .map(|&i| test_types[i].to_string())
        .collect();

    println!();

    // Step 12: Documentation Requirements
    println!("{}", "📚 DOCUMENTATION".cyan().bold());
    println!();

    let doc_types = vec![
        output.t("feature-doc-readme"),
        output.t("feature-doc-api"),
        output.t("feature-doc-comments"),
        output.t("feature-doc-guide"),
        output.t("feature-doc-adr"),
        output.t("feature-doc-migration"),
    ];

    let selected_docs = MultiSelect::new()
        .with_prompt(output.t("feature-doc-prompt"))
        .items(&doc_types)
        .interact()?;

    let docs_required: Vec<String> = selected_docs
        .iter()
        .map(|&i| doc_types[i].to_string())
        .collect();

    println!();

    // Step 13: Implementation Progress
    println!("{}", "🔄 IMPLEMENTATION STATUS".cyan().bold());
    println!();

    let status_options = vec![
        output.t("feature-status-planning"),
        output.t("feature-status-progress"),
        output.t("feature-status-testing"),
        output.t("feature-status-review"),
        output.t("feature-status-completed"),
    ];

    let status_idx = Select::new()
        .with_prompt(output.t("feature-status-prompt"))
        .items(&status_options)
        .default(0)
        .interact()?;

    let current_status = status_options[status_idx].clone();

    let is_completed = status_idx == 4;
    let duration = start_time.elapsed().as_secs() / 60;

    println!();

    // Step 14: Key Learnings
    let user_learning = if is_completed {
        println!("{}", "💡 KEY LEARNINGS".cyan().bold());
        println!();

        let learning = Input::<String>::new()
            .with_prompt(output.t("feature-learning-prompt"))
            .allow_empty(true)
            .interact_text()?;

        println!();
        learning
    } else {
        String::new()
    };

    // Step 15: Save Learning Session
    let mut session = LearningSessionBuilder::new("feature", &feature_name)
        .tag("feature-development")
        .tag(feature_type)
        .metadata("complexity", complexity)
        .metadata("status", current_status)
        .metadata("problem", &problem_statement)
        .metadata("users", &target_users);

    if let Some(branch) = &branch_name {
        session = session.metadata("branch", branch);
    }

    // Add description as a step
    session = session.step(format!("Feature: {}", feature_desc));

    // Add acceptance criteria
    for criterion in &acceptance_criteria {
        session = session.step(format!("Acceptance: {}", criterion));
    }

    // Add design considerations
    for design in &design_items {
        session = session.step(format!("Design: {}", design));
    }

    // Add files
    for file in &files {
        session = session.file(file);
    }

    // Add testing requirements
    for test in &tests_required {
        session = session.step(format!("Test: {}", test));
    }

    // Add documentation requirements
    for doc in &docs_required {
        session = session.step(format!("Doc: {}", doc));
    }

    if is_completed {
        session = session.resolved(Some(duration as u32));
        session = session.solution(&feature_desc);
        if status_idx == 4 {
            let learning_text = if user_learning.is_empty() {
                "Feature successfully completed and deployed".to_string()
            } else {
                user_learning
            };
            session = session.learning(&learning_text);
        }
    }

    let (session, path) = session.save()?;

    println!("{}", "✅ Feature session saved".green());
    println!("   Session ID: {}", session.id.cyan());
    println!("   Path: {}", path.display().to_string().cyan());
    println!();

    // Step 16: Next Steps Guidance
    match status_idx {
        0 => {
            // Planning
            println!(
                "{}",
                "📋 NEXT STEPS: Planning → Implementation".green().bold()
            );
            println!();
            println!("1. Review and finalize the design considerations");
            println!("2. Set up the Git branch (if not done)");
            println!("3. Start implementing the first file");
            println!("4. Use TodoWrite to track progress through the implementation");
            println!();
            println!("Suggested commands:");
            println!("  $ npm run dev          # Start development server");
            println!("  $ npm run type-check   # Verify TypeScript types");
        }
        1 => {
            // In Progress
            println!(
                "{}",
                "🛠️  NEXT STEPS: Continue Implementation".green().bold()
            );
            println!();
            println!("1. Complete remaining file implementations");
            println!("2. Write tests as you go (TDD approach)");
            println!("3. Regularly test against acceptance criteria");
            println!("4. Update documentation incrementally");
            println!();
            println!("Suggested commands:");
            println!("  $ npm run test         # Run tests");
            println!("  $ npm run lint:fix     # Fix linting issues");
        }
        2 => {
            // Testing
            println!("{}", "🧪 NEXT STEPS: Testing → Review".green().bold());
            println!();
            println!("1. Complete all test types selected");
            println!("2. Verify all acceptance criteria are met");
            println!("3. Run full test suite");
            println!("4. Prepare for code review");
            println!();
            println!("Suggested commands:");
            println!("  $ npm run test:coverage  # Check test coverage");
            println!("  $ npm run lint           # Final lint check");
            println!("  $ npm run build          # Verify build succeeds");
        }
        3 => {
            // Review
            println!("{}", "👀 NEXT STEPS: Code Review → Merge".green().bold());
            println!();
            println!("1. Create pull request");
            println!("2. Request code review from team");
            println!("3. Address review feedback");
            println!("4. Prepare for merge and deployment");
            println!();
            println!("Suggested commands:");
            println!(
                "  $ git push -u origin {}    # Push branch",
                branch_name.as_deref().unwrap_or("feature-branch")
            );
            println!("  $ cldev pr                 # Create PR (if available)");
        }
        4 => {
            // Completed
            println!("{}", "🎉 FEATURE COMPLETED!".green().bold());
            println!();
            println!("   Time taken: {} minutes", duration);
            println!();
            println!("Next steps:");
            println!("  1. Monitor feature in production");
            println!("  2. Gather user feedback");
            println!("  3. Document lessons learned");
            println!("  4. Plan follow-up improvements");
        }
        _ => {}
    }

    println!();
    println!("{}", "💡 BEST PRACTICES".cyan().bold());
    println!("  • Break large features into smaller, reviewable PRs");
    println!("  • Write tests alongside implementation (TDD)");
    println!("  • Commit frequently with clear messages");
    println!("  • Document decisions and trade-offs");
    println!("  • Seek early feedback through demos or WIP PRs");
    println!("  • Keep acceptance criteria visible and refer to them often");

    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_feature_command_structure() {
        // Test that the command structure is well-formed
    }
}
