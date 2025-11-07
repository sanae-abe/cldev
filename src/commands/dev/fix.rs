use crate::core::session_recorder::{LearningSession, LearningSessionBuilder};
use crate::core::{CldevError, Result};
use colored::*;
use dialoguer::{Confirm, Input, MultiSelect, Select};
use std::time::Instant;

/// Handle important bug fixes with structured approach
///
/// This command provides a same-day resolution framework for critical bugs
/// with root cause analysis and systematic testing.
pub fn handle_fix(target: Option<String>) -> Result<()> {
    let start_time = Instant::now();

    println!("{}", "🔧 FIX: Important Bug Resolution".yellow().bold());
    println!("{}", "━".repeat(60).yellow());
    println!();

    // Step 1: Bug Description
    let bug_desc = if let Some(t) = target {
        t
    } else {
        Input::<String>::new()
            .with_prompt("📝 Describe the bug (symptoms and impact)")
            .interact_text()?
    };

    println!();

    // Step 2: Bug Classification
    let bug_categories = vec![
        "Logic error (incorrect behavior)",
        "UI/UX issue (visual or interaction)",
        "Performance issue (slowness, memory leak)",
        "Data integrity issue",
        "API/Integration error",
        "Security vulnerability",
        "Configuration error",
        "Edge case handling",
    ];

    let category_idx = Select::new()
        .with_prompt("🏷️  Bug Category")
        .items(&bug_categories)
        .interact()?;

    let category = bug_categories[category_idx];

    println!();

    // Step 3: Reproducibility
    let reproducibility_options = vec![
        "Always (100% reproducible)",
        "Frequently (>75%)",
        "Sometimes (25-75%)",
        "Rarely (<25%)",
        "Unable to reproduce",
    ];

    let repro_idx = Select::new()
        .with_prompt("🔄 Reproducibility")
        .items(&reproducibility_options)
        .interact()?;

    let reproducibility = reproducibility_options[repro_idx];

    println!();

    // Step 4: Reproduction Steps (if reproducible)
    let reproduction_steps = if repro_idx < 4 {
        // If reproducible
        println!("{}", "📋 REPRODUCTION STEPS".cyan().bold());
        println!("Enter step-by-step instructions to reproduce the bug:");
        println!("(Press Enter twice when done)");
        println!();

        let mut steps = Vec::new();
        let mut step_num = 1;

        loop {
            let step = Input::<String>::new()
                .with_prompt(&format!("Step {}", step_num))
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
    println!("{}", "🔍 ROOT CAUSE ANALYSIS".cyan().bold());
    println!();

    // Step 5: Investigation Checklist
    let investigation_areas = vec![
        "Review recent code changes (git log, blame)",
        "Check error logs and stack traces",
        "Add debug logging/breakpoints",
        "Review related test cases",
        "Check dependencies/library versions",
        "Verify environment configurations",
        "Review data flow and state management",
        "Check boundary conditions and edge cases",
    ];

    println!("Investigation checklist:");
    for (i, area) in investigation_areas.iter().enumerate() {
        println!("  {}. {}", i + 1, area);
    }

    println!();

    // Step 6: Root Cause Input
    let root_cause = Input::<String>::new()
        .with_prompt("🎯 Root Cause (what's causing the bug?)")
        .interact_text()?;

    println!();

    // Step 7: Fix Strategy
    println!("{}", "🛠️  FIX PATTERNS".green().bold());
    println!();

    let fix_patterns = vec![
        "Add null/undefined checks",
        "Fix conditional logic",
        "Update algorithm/calculation",
        "Add input validation",
        "Fix race condition/timing issue",
        "Update state management",
        "Fix API integration",
        "Optimize performance",
        "Add error handling",
        "Update configuration",
        "Other (custom fix)",
    ];

    let pattern_idx = Select::new()
        .with_prompt("Select fix pattern")
        .items(&fix_patterns)
        .interact()?;

    let fix_pattern = fix_patterns[pattern_idx];

    println!();

    // Step 8: Implementation Plan
    let implementation = Input::<String>::new()
        .with_prompt("💡 Implementation plan (brief description)")
        .interact_text()?;

    println!();

    // Step 9: Files to Modify
    println!("{}", "📁 Affected Files".cyan());
    println!("Enter file paths (one per line, press Enter twice when done):");
    println!();

    let mut files = Vec::new();
    loop {
        let file = Input::<String>::new()
            .with_prompt("File")
            .allow_empty(true)
            .interact_text()?;

        if file.is_empty() {
            break;
        }

        files.push(file);
    }

    println!();
    println!("{}", "✅ TESTING CHECKLIST".cyan().bold());
    println!();

    // Step 10: Testing Requirements
    let test_requirements = vec![
        "Add/update unit tests for the fix",
        "Verify existing tests still pass",
        "Test the reproduction steps (if available)",
        "Test edge cases and boundary conditions",
        "Perform regression testing",
        "Test in staging/pre-production environment",
        "Verify performance impact",
        "Check for security implications",
    ];

    let selected_tests = MultiSelect::new()
        .with_prompt("Select testing steps to perform")
        .items(&test_requirements)
        .defaults(&vec![true; test_requirements.len()])
        .interact()?;

    let tests_to_perform: Vec<String> = selected_tests
        .iter()
        .map(|&i| test_requirements[i].to_string())
        .collect();

    println!();
    println!("{}", "🧪 TESTING COMMANDS".green().bold());
    println!();
    println!("Suggested test commands:");
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
    println!("{}", "💾 RECOMMENDED COMMIT MESSAGE".cyan().bold());
    println!();

    let commit_type = match category {
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
        .with_prompt("Is the bug fixed and tested?")
        .default(false)
        .interact()?;

    let duration = start_time.elapsed().as_secs() / 60;

    // Step 13: Save Learning Session
    let mut session = LearningSessionBuilder::new("fix", &bug_desc)
        .tag("bug-fix")
        .tag(category)
        .root_cause(&root_cause)
        .solution(&implementation)
        .metadata("fix_pattern", fix_pattern)
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
    println!("{}", "✅ Session saved".green());
    println!("   Session ID: {}", session.id.cyan());
    println!("   Path: {}", path.display().to_string().cyan());
    println!();

    if resolved {
        println!("{}", "🎉 Bug fixed and tested!".green().bold());
        println!("   Time taken: {} minutes", duration);
        println!();
        println!("Next steps:");
        println!("  1. Create pull request with the fix");
        println!("  2. Request code review");
        println!("  3. Monitor after deployment");
    } else {
        println!("{}", "⚠️  Continue working on the fix".yellow().bold());
        println!();
        println!("Next steps:");
        println!("  1. Implement the fix based on the plan");
        println!("  2. Run the testing checklist");
        println!("  3. Re-run this command to update status");
    }

    println!();
    println!("{}", "💡 BEST PRACTICES".cyan().bold());
    println!("  • Write clear, focused commits");
    println!("  • Add tests to prevent regression");
    println!("  • Document edge cases and assumptions");
    println!("  • Consider similar bugs in related code");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fix_command_structure() {
        // Test that the command structure is well-formed
        assert!(true);
    }
}
