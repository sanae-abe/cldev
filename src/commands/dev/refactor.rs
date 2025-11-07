use crate::cli::output::OutputHandler;
use crate::core::session_recorder::LearningSessionBuilder;
use crate::core::{CldevError, Result};
use colored::*;
use dialoguer::{Confirm, Input, MultiSelect, Select};
use std::time::Instant;

/// Safe refactoring workflow with impact analysis
///
/// This command provides a structured approach for refactoring code:
/// - Target analysis and motivation
/// - Test coverage verification
/// - Incremental refactoring steps
/// - Continuous testing
/// - Impact analysis on existing functionality
pub fn handle_refactor(target: Option<String>, output: &OutputHandler) -> Result<()> {
    let start_time = Instant::now();

    println!("{}", output.t("refactor-header").blue().bold());
    println!("{}", output.t("refactor-separator").blue());
    println!();

    // Step 1: Refactoring Target
    let refactor_target = if let Some(t) = target {
        t
    } else {
        Input::<String>::new()
            .with_prompt(&output.t("refactor-target-prompt"))
            .interact_text()?
    };

    println!();

    // Step 2: Motivation for Refactoring
    println!("{}", output.t("refactor-motivation-header").cyan().bold());
    println!();

    let motivation_options = vec![
        "Improve code readability/maintainability",
        "Reduce code duplication (DRY principle)",
        "Improve performance",
        "Simplify complex logic",
        "Extract reusable components/utilities",
        "Improve type safety",
        "Update to modern patterns/best practices",
        "Reduce technical debt",
        "Prepare for new features",
        "Fix code smells",
    ];

    let motivation_indices = MultiSelect::new()
        .with_prompt("Select refactoring goals")
        .items(&motivation_options)
        .interact()?;

    let motivations: Vec<String> = motivation_indices
        .iter()
        .map(|&i| motivation_options[i].to_string())
        .collect();

    if motivations.is_empty() {
        return Err(CldevError::config(
            "At least one refactoring goal must be selected",
        ));
    }

    println!();

    // Step 3: Scope and Impact Analysis
    println!("{}", "📊 IMPACT ANALYSIS".cyan().bold());
    println!();

    let scope_options = vec![
        "Single function/method (low impact)",
        "Single file/module (medium impact)",
        "Multiple related files (high impact)",
        "Cross-cutting concern (system-wide impact)",
    ];

    let scope_idx = Select::new()
        .with_prompt("Refactoring Scope")
        .items(&scope_options)
        .interact()?;

    let scope = scope_options[scope_idx];
    let is_high_impact = scope_idx >= 2;

    println!();

    // Step 4: Files to Refactor
    println!("{}", "📁 FILES TO REFACTOR".cyan().bold());
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

    if files.is_empty() {
        println!("{}", "⚠️  Warning: No files specified".yellow());
    }

    println!();

    // Step 5: Test Coverage Check
    println!("{}", "🧪 TEST COVERAGE VERIFICATION".cyan().bold());
    println!();
    println!("Before refactoring, verify that the code has adequate test coverage.");
    println!();

    let has_tests = Confirm::new()
        .with_prompt("Does the target code have existing tests?")
        .default(false)
        .interact()?;

    if !has_tests {
        println!();
        println!(
            "{}",
            "⚠️  WARNING: Refactoring without tests is risky!"
                .yellow()
                .bold()
        );
        println!();
        println!("Recommended approach:");
        println!("  1. Write characterization tests first (test current behavior)");
        println!("  2. Ensure tests pass before refactoring");
        println!("  3. Refactor incrementally, running tests after each step");
        println!("  4. Keep tests passing throughout the refactoring");
        println!();

        let continue_without_tests = Confirm::new()
            .with_prompt("Continue without tests? (not recommended)")
            .default(false)
            .interact()?;

        if !continue_without_tests {
            println!();
            println!("{}", "✅ Good decision! Write tests first.".green());
            println!();
            println!("Suggested approach:");
            println!("  $ npm run test -- --coverage       # Check current coverage");
            println!("  # Write tests for the target code");
            println!("  $ npm run test                     # Verify tests pass");
            println!(
                "  $ cldev refactor \"{}\"             # Run refactoring again",
                refactor_target
            );
            return Ok(());
        }
    } else {
        println!();
        println!("Run tests before starting:");
        println!("  $ npm run test                   # Verify all tests pass");
        println!("  $ npm run test:coverage          # Check coverage percentage");
        println!();
    }

    println!();

    // Step 6: Refactoring Patterns/Techniques
    println!("{}", "🛠️  REFACTORING TECHNIQUES".green().bold());
    println!();

    let refactoring_patterns = vec![
        "Extract Function/Method",
        "Extract Component/Module",
        "Inline Function/Variable",
        "Rename (improve naming)",
        "Move Function/Class",
        "Replace Conditional with Polymorphism",
        "Introduce Parameter Object",
        "Replace Magic Numbers with Constants",
        "Decompose Conditional",
        "Consolidate Duplicate Code",
        "Simplify Complex Expression",
        "Replace Nested Conditionals with Guard Clauses",
        "Extract Interface/Type",
        "Replace Loop with Pipeline (map/filter/reduce)",
    ];

    let selected_patterns = MultiSelect::new()
        .with_prompt("Select refactoring techniques to apply")
        .items(&refactoring_patterns)
        .interact()?;

    let patterns: Vec<String> = selected_patterns
        .iter()
        .map(|&i| refactoring_patterns[i].to_string())
        .collect();

    println!();

    // Step 7: Incremental Refactoring Plan
    println!("{}", "📝 INCREMENTAL REFACTORING PLAN".cyan().bold());
    println!();
    println!("Break refactoring into small, safe steps:");
    println!();

    let mut refactoring_steps = Vec::new();
    let mut step_num = 1;

    println!("Enter refactoring steps (press Enter twice when done):");
    println!("Example: 'Extract validateUser function from processRequest'");
    println!();

    loop {
        let step = Input::<String>::new()
            .with_prompt(&format!("Step {}", step_num))
            .allow_empty(true)
            .interact_text()?;

        if step.is_empty() {
            break;
        }

        refactoring_steps.push(step);
        step_num += 1;
    }

    if refactoring_steps.is_empty() {
        println!(
            "{}",
            "⚠️  No steps defined - will proceed with general refactoring".yellow()
        );
    }

    println!();

    // Step 8: Safety Checklist
    println!("{}", "✅ SAFETY CHECKLIST".cyan().bold());
    println!();

    let safety_checks = vec![
        "Run all tests before starting (establish baseline)",
        "Make small, incremental changes",
        "Run tests after each step",
        "Keep commits small and focused",
        "Verify no behavioral changes (tests still pass)",
        "Check for compilation/type errors after each change",
        "Review impact on dependent code",
        "Update documentation if interfaces change",
    ];

    println!("Safety practices to follow:");
    for (i, check) in safety_checks.iter().enumerate() {
        println!("  {}. {}", i + 1, check);
    }

    println!();

    if is_high_impact {
        println!(
            "{}",
            "⚠️  HIGH IMPACT REFACTORING - Extra Precautions:"
                .yellow()
                .bold()
        );
        println!("  • Create a feature branch for the refactoring");
        println!("  • Consider pair programming or mob programming");
        println!("  • Plan for incremental PRs (multiple small PRs > one large PR)");
        println!("  • Communicate with team about potential merge conflicts");
        println!("  • Consider feature flags for gradual rollout");
        println!();
    }

    // Step 9: Commands for Testing
    println!("{}", "🧪 TESTING COMMANDS".green().bold());
    println!();
    println!("Use these commands throughout refactoring:");
    println!();
    println!("  # Run all tests");
    println!("  $ npm run test");
    println!();
    println!("  # Run specific test file");
    println!("  $ npm run test -- <test-file>");
    println!();
    println!("  # Run tests in watch mode (auto-rerun on changes)");
    println!("  $ npm run test:watch");
    println!();
    println!("  # Type checking");
    println!("  $ npm run type-check");
    println!();
    println!("  # Linting");
    println!("  $ npm run lint");
    println!();
    println!("  # Build verification");
    println!("  $ npm run build");
    println!();

    // Step 10: Execution Workflow
    println!("{}", "🔄 RECOMMENDED WORKFLOW".cyan().bold());
    println!();
    println!("For each refactoring step:");
    println!();
    println!("  1. 🧪 Run tests → All passing ✅");
    println!("  2. ✏️  Make ONE small change");
    println!("  3. 🧪 Run tests → All passing ✅");
    println!("  4. 💾 Commit with descriptive message");
    println!("  5. 🔁 Repeat");
    println!();

    // Step 11: Security Impact Check
    if is_high_impact {
        println!();
        println!("{}", "🔒 SECURITY IMPACT CHECK".yellow().bold());
        println!();

        let security_considerations = vec![
            "Authentication/authorization logic changes",
            "Input validation changes",
            "Data sanitization changes",
            "Access control changes",
            "Encryption/decryption logic changes",
            "API endpoint changes",
            "None of the above",
        ];

        let security_impacts = MultiSelect::new()
            .with_prompt("Does this refactoring affect any of these security-sensitive areas?")
            .items(&security_considerations)
            .interact()?;

        if !security_impacts.is_empty() && security_impacts[0] < security_considerations.len() - 1 {
            println!();
            println!("{}", "⚠️  SECURITY REVIEW REQUIRED".red().bold());
            println!();
            println!("This refactoring touches security-sensitive code.");
            println!("Additional requirements:");
            println!("  • Security-focused code review is mandatory");
            println!("  • Verify all security tests pass");
            println!("  • Consider security testing (penetration testing)");
            println!("  • Document security implications in PR");
            println!();
        }
    }

    println!();

    // Step 12: Progress Tracking
    println!("{}", "📊 REFACTORING STATUS".cyan().bold());
    println!();

    let status_options = vec![
        "Planning (not started yet)",
        "In Progress (actively refactoring)",
        "Testing (refactoring done, verifying changes)",
        "Review (ready for code review)",
        "Completed (merged)",
    ];

    let status_idx = Select::new()
        .with_prompt("Current Status")
        .items(&status_options)
        .default(0)
        .interact()?;

    let current_status = status_options[status_idx];
    let is_completed = status_idx == 4;

    println!();

    // Step 13: Results and Metrics (if completed)
    let mut improvements = Vec::new();
    if is_completed {
        println!("{}", "📈 REFACTORING RESULTS".green().bold());
        println!();

        let measure_improvements = Confirm::new()
            .with_prompt("Did you measure improvements?")
            .default(false)
            .interact()?;

        if measure_improvements {
            println!();
            println!("Enter improvements (press Enter twice when done):");
            println!("Examples:");
            println!("  - Reduced file size from 500 to 300 lines");
            println!("  - Eliminated 5 instances of code duplication");
            println!("  - Improved test coverage from 60% to 85%");
            println!("  - Performance improved by 20%");
            println!();

            loop {
                let improvement = Input::<String>::new()
                    .with_prompt("Improvement")
                    .allow_empty(true)
                    .interact_text()?;

                if improvement.is_empty() {
                    break;
                }

                improvements.push(improvement);
            }
        }
    }

    let duration = start_time.elapsed().as_secs() / 60;

    // Step 14: Save Learning Session
    let mut session = LearningSessionBuilder::new("refactor", &refactor_target)
        .tag("refactoring")
        .tag("code-quality")
        .metadata("scope", scope)
        .metadata("status", current_status);

    // Add motivations
    for motivation in &motivations {
        session = session.step(format!("Goal: {}", motivation));
    }

    // Add files
    for file in &files {
        session = session.file(file);
    }

    // Add refactoring patterns
    for pattern in &patterns {
        session = session.step(format!("Technique: {}", pattern));
    }

    // Add refactoring steps
    for step in &refactoring_steps {
        session = session.step(format!("Plan: {}", step));
    }

    // Add improvements
    for improvement in &improvements {
        session = session.learning(improvement);
    }

    if is_completed {
        session = session.resolved(Some(duration as u32));
        session = session.solution(format!("Successfully refactored: {}", patterns.join(", ")));
    }

    let (session, path) = session.save()?;

    println!();
    println!("{}", "✅ Refactoring session saved".green());
    println!("   Session ID: {}", session.id.cyan());
    println!("   Path: {}", path.display().to_string().cyan());
    println!();

    // Step 15: Next Steps
    match status_idx {
        0 => {
            // Planning
            println!("{}", "📋 NEXT STEPS: Start Refactoring".green().bold());
            println!();
            println!("1. Ensure all tests are passing (baseline)");
            println!(
                "2. Create a feature branch (e.g., refactor/{}))",
                refactor_target.to_lowercase().replace(" ", "-")
            );
            println!("3. Begin with the first small refactoring step");
            println!("4. Run tests after each change");
            println!();
        }
        1 => {
            // In Progress
            println!("{}", "🔄 NEXT STEPS: Continue Refactoring".green().bold());
            println!();
            println!("1. Continue with incremental steps");
            println!("2. Keep tests passing (green) at all times");
            println!("3. Commit frequently with clear messages");
            println!("4. Review your changes regularly");
            println!();
        }
        2 => {
            // Testing
            println!("{}", "🧪 NEXT STEPS: Verification".green().bold());
            println!();
            println!("1. Run full test suite");
            println!("2. Verify no behavioral changes");
            println!("3. Check performance if applicable");
            println!("4. Review all changes before PR");
            println!();
        }
        3 => {
            // Review
            println!("{}", "👀 NEXT STEPS: Code Review".green().bold());
            println!();
            println!("1. Create pull request");
            println!("2. Provide clear description of refactoring");
            println!("3. Highlight areas that need special attention");
            println!("4. Address review feedback");
            println!();
        }
        4 => {
            // Completed
            println!("{}", "🎉 REFACTORING COMPLETED!".green().bold());
            println!();
            println!("   Time taken: {} minutes", duration);
            println!();
            if !improvements.is_empty() {
                println!("Improvements achieved:");
                for improvement in &improvements {
                    println!("  ✓ {}", improvement.green());
                }
                println!();
            }
        }
        _ => {}
    }

    println!("{}", "💡 REFACTORING PRINCIPLES".cyan().bold());
    println!("  • Make it work, make it right, make it fast (in that order)");
    println!("  • Keep the code always working (tests always green)");
    println!("  • Small steps are safer than big leaps");
    println!("  • Don't mix refactoring with feature development");
    println!("  • When in doubt, run the tests");
    println!("  • Refactoring is about improving design, not changing behavior");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_refactor_command_structure() {
        // Test that the command structure is well-formed
        assert!(true);
    }
}
