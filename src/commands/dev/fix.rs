use crate::cli::output::OutputHandler;
use crate::core::session_recorder::LearningSessionBuilder;
use crate::core::Result;
use colored::*;
use dialoguer::{Confirm, Input, MultiSelect, Select};
use std::time::Instant;

/// Handle important bug fixes with structured approach
///
/// This command provides a same-day resolution framework for critical bugs
/// with root cause analysis and systematic testing.
pub fn handle_fix(target: Option<String>, output: &OutputHandler) -> Result<()> {
    let start_time = Instant::now();

    println!("{}", output.t("fix-header").yellow().bold());
    println!("{}", output.t("fix-separator").yellow());
    println!();

    // Step 1: Bug Description
    let bug_desc = if let Some(t) = target {
        t
    } else {
        Input::<String>::new()
            .with_prompt(output.t("fix-describe-bug"))
            .interact_text()?
    };

    println!();

    // Step 2: Bug Classification
    let bug_categories = vec![
        output.t("fix-category-logic"),
        output.t("fix-category-ui"),
        output.t("fix-category-performance"),
        output.t("fix-category-data"),
        output.t("fix-category-api"),
        output.t("fix-category-security"),
        output.t("fix-category-config"),
        output.t("fix-category-edge-case"),
    ];

    let category_idx = Select::new()
        .with_prompt(output.t("fix-bug-category"))
        .items(&bug_categories)
        .interact()?;

    let category = bug_categories[category_idx].clone();

    println!();

    // Step 3: Reproducibility
    let reproducibility_options = vec![
        output.t("fix-repro-always"),
        output.t("fix-repro-frequently"),
        output.t("fix-repro-sometimes"),
        output.t("fix-repro-rarely"),
        output.t("fix-repro-unable"),
    ];

    let repro_idx = Select::new()
        .with_prompt(output.t("fix-reproducibility"))
        .items(&reproducibility_options)
        .interact()?;

    let reproducibility = reproducibility_options[repro_idx].clone();

    println!();

    // Step 4: Reproduction Steps (if reproducible)
    let reproduction_steps = if repro_idx < 4 {
        // If reproducible
        println!("{}", output.t("fix-reproduction-steps").cyan().bold());
        println!("{}", output.t("fix-enter-steps"));
        println!("{}", output.t("fix-press-enter-twice"));
        println!();

        let mut steps = Vec::new();
        let mut step_num = 1;

        loop {
            let step = Input::<String>::new()
                .with_prompt(output.t_format("fix-step-num", "num", &step_num.to_string()))
                .allow_empty(true)
                .interact_text()?;

            if step.is_empty() {
                break;
            }

            steps.push(step);
            step_num += 1;
        }

        Some(steps)
    } else {
        None
    };

    println!();
    println!("{}", output.t("fix-root-cause-analysis").cyan().bold());
    println!();

    // Step 5: Investigation Checklist
    let investigation_areas = [
        output.t("fix-investigation-git"),
        output.t("fix-investigation-logs"),
        output.t("fix-investigation-debug"),
        output.t("fix-investigation-tests"),
        output.t("fix-investigation-deps"),
        output.t("fix-investigation-env"),
        output.t("fix-investigation-data-flow"),
        output.t("fix-investigation-edge-cases"),
    ];

    println!("{}", output.t("fix-investigation-checklist"));
    for (i, area) in investigation_areas.iter().enumerate() {
        println!("  {}. {}", i + 1, area);
    }

    println!();

    // Step 6: Root Cause Input
    let root_cause = Input::<String>::new()
        .with_prompt(output.t("fix-root-cause-prompt"))
        .interact_text()?;

    println!();

    // Step 7: Fix Strategy
    println!("{}", output.t("fix-fix-patterns").green().bold());
    println!();

    let fix_patterns = vec![
        output.t("fix-pattern-null-check"),
        output.t("fix-pattern-conditional"),
        output.t("fix-pattern-algorithm"),
        output.t("fix-pattern-validation"),
        output.t("fix-pattern-race"),
        output.t("fix-pattern-state"),
        output.t("fix-pattern-api"),
        output.t("fix-pattern-performance"),
        output.t("fix-pattern-error-handling"),
        output.t("fix-pattern-config"),
        output.t("fix-pattern-other"),
    ];

    let pattern_idx = Select::new()
        .with_prompt(output.t("fix-select-pattern"))
        .items(&fix_patterns)
        .interact()?;

    let fix_pattern = fix_patterns[pattern_idx].clone();

    println!();

    // Step 8: Implementation Plan
    let implementation = Input::<String>::new()
        .with_prompt(output.t("fix-implementation-plan"))
        .interact_text()?;

    println!();

    // Step 9: Files to Modify
    println!("{}", output.t("fix-affected-files").cyan());
    println!("{}", output.t("fix-enter-files"));
    println!();

    let mut files = Vec::new();
    loop {
        let file = Input::<String>::new()
            .with_prompt(output.t("fix-file-prompt"))
            .allow_empty(true)
            .interact_text()?;

        if file.is_empty() {
            break;
        }

        files.push(file);
    }

    println!();
    println!("{}", output.t("fix-testing-checklist").cyan().bold());
    println!();

    // Step 10: Testing Requirements
    let test_requirements = vec![
        output.t("fix-test-unit"),
        output.t("fix-test-existing"),
        output.t("fix-test-repro"),
        output.t("fix-test-edge-cases"),
        output.t("fix-test-regression"),
        output.t("fix-test-staging"),
        output.t("fix-test-performance-impact"),
        output.t("fix-test-security"),
    ];

    let selected_tests = MultiSelect::new()
        .with_prompt(output.t("fix-select-tests"))
        .items(&test_requirements)
        .defaults(&vec![true; test_requirements.len()])
        .interact()?;

    let tests_to_perform: Vec<String> = selected_tests
        .iter()
        .map(|&i| test_requirements[i].to_string())
        .collect();

    println!();
    println!("{}", output.t("fix-testing-commands").green().bold());
    println!();
    println!("{}", output.t("fix-suggested-commands"));
    println!("  # Run unit tests");
    println!("  $ npm run test");
    println!();
    println!("  # Run specific test file");
    println!("  $ npm run test -- <test-file>");
    println!();
    println!("  # Run with coverage");
    println!("  $ npm run test:coverage");
    println!();
    println!("  # Type checking");
    println!("  $ npm run type-check");
    println!();
    println!("  # Linting");
    println!("  $ npm run lint");
    println!();

    // Step 11: Commit Message Suggestion
    println!("{}", output.t("fix-commit-message").cyan().bold());
    println!();

    let commit_type = match category.as_str() {
        c if c.contains("Security") => "fix(security)",
        c if c.contains("Performance") => "perf",
        c if c.contains("UI/UX") => "fix(ui)",
        _ => "fix",
    };

    let commit_msg = format!(
        "{}: {}\n\nRoot cause: {}\nFix: {}\n\nAffected files:\n{}\n\nTested:\n{}",
        commit_type,
        bug_desc.lines().next().unwrap_or(""),
        root_cause,
        implementation,
        files
            .iter()
            .map(|f| format!("- {}", f))
            .collect::<Vec<_>>()
            .join("\n"),
        tests_to_perform
            .iter()
            .map(|t| format!("- {}", t))
            .collect::<Vec<_>>()
            .join("\n")
    );

    println!("{}", commit_msg.cyan());
    println!();

    // Step 12: Mark as Resolved
    let resolved = Confirm::new()
        .with_prompt(output.t("fix-is-resolved"))
        .default(false)
        .interact()?;

    let duration = start_time.elapsed().as_secs() / 60;

    // Step 13: Save Learning Session
    let mut session = LearningSessionBuilder::new("fix", &bug_desc)
        .tag("bug-fix")
        .tag(&category)
        .root_cause(&root_cause)
        .solution(&implementation)
        .metadata("fix_pattern", &fix_pattern)
        .metadata("reproducibility", reproducibility);

    for file in &files {
        session = session.file(file);
    }

    if let Some(steps) = reproduction_steps {
        for step in steps {
            session = session.step(format!("Repro: {}", step));
        }
    }

    session = session.step(format!("Fix strategy: {}", fix_pattern));
    session = session.step(format!("Implementation: {}", implementation));

    for test in &tests_to_perform {
        session = session.step(format!("Test: {}", test));
    }

    if resolved {
        session = session.resolved(Some(duration as u32));
        session = session.learning(format!("Successfully fixed: {} - {}", category, root_cause));
    }

    let (session, path) = session.save()?;

    println!();
    output.success(&output.t("fix-session-saved"));
    println!(
        "   {}",
        output.t_format("fix-session-id", "id", &session.id.cyan().to_string())
    );
    println!(
        "   {}",
        output.t_format(
            "fix-session-path",
            "path",
            &path.display().to_string().cyan().to_string()
        )
    );
    println!();

    if resolved {
        println!("{}", output.t("fix-bug-resolved").green().bold());
        println!(
            "   {}",
            output.t_format("fix-duration", "minutes", &duration.to_string())
        );
        println!();
        println!("{}", output.t("fix-next-steps-resolved"));
        println!("  1. {}", output.t("fix-next-pr"));
        println!("  2. {}", output.t("fix-next-review"));
        println!("  3. {}", output.t("fix-next-monitor"));
    } else {
        println!("{}", output.t("fix-continue-working").yellow().bold());
        println!();
        println!("{}", output.t("fix-next-steps-ongoing"));
        println!("  1. {}", output.t("fix-next-implement"));
        println!("  2. {}", output.t("fix-next-run-tests"));
        println!("  3. {}", output.t("fix-next-update"));
    }

    println!();
    println!("{}", output.t("fix-best-practices").cyan().bold());
    println!("  • {}", output.t("fix-tip-clear-commits"));
    println!("  • {}", output.t("fix-tip-add-tests"));
    println!("  • {}", output.t("fix-tip-document"));
    println!("  • {}", output.t("fix-tip-similar-bugs"));

    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_fix_command_structure() {
        // Test that the command structure is well-formed
    }
}
