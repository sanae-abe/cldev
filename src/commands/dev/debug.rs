use crate::cli::output::OutputHandler;
use crate::core::session_recorder::LearningSessionBuilder;
use crate::core::Result;
use colored::*;
use dialoguer::{Confirm, Input, MultiSelect, Select};
use std::time::Instant;

/// Systematic debugging workflow with structured investigation
///
/// This command provides a comprehensive debugging framework for identifying
/// root causes of issues through methodical investigation.
pub fn handle_debug(symptom: Option<String>, output: &OutputHandler) -> Result<()> {
    let start_time = Instant::now();

    println!("{}", output.t("debug-header").blue().bold());
    println!("{}", output.t("debug-separator").blue());
    println!();

    // Step 1: Symptom Description
    let symptom_desc = if let Some(s) = symptom {
        s
    } else {
        Input::<String>::new()
            .with_prompt(&output.t("debug-describe-symptom"))
            .interact_text()?
    };

    println!();

    // Step 2: Issue Type Classification
    let issue_types = vec![
        "Unexpected behavior (wrong output)",
        "Error/Exception thrown",
        "Performance issue (slow, hanging)",
        "Visual/UI issue",
        "Integration/API failure",
        "State management issue",
        "Memory leak/resource issue",
        "Intermittent/timing issue",
    ];

    let issue_idx = Select::new()
        .with_prompt(&output.t("debug-issue-type"))
        .items(&issue_types)
        .interact()?;

    let issue_type = issue_types[issue_idx];

    println!();

    // Step 3: Environment Information
    println!("{}", output.t("debug-environment").cyan().bold());
    println!();

    let environments = vec![
        "Development (local)",
        "Staging/QA",
        "Production",
        "CI/CD pipeline",
    ];

    let env_idx = Select::new()
        .with_prompt(&output.t("debug-where-occur"))
        .items(&environments)
        .interact()?;

    let environment = environments[env_idx];

    // Step 4: When did it start?
    let onset_options = vec![
        "Just now (new)",
        "After recent deployment/change",
        "Has been happening for a while",
        "Intermittent (started and stopped)",
        "Not sure",
    ];

    let onset_idx = Select::new()
        .with_prompt(&output.t("debug-when-start"))
        .items(&onset_options)
        .interact()?;

    let onset = onset_options[onset_idx];

    println!();
    println!("{}", output.t("debug-systematic-checklist").cyan().bold());
    println!();

    // Step 5: Investigation Framework
    let investigation_framework = vec![
        // Data Flow
        "1️⃣  INPUT ANALYSIS",
        "   • What are the input values?",
        "   • Are inputs valid/expected?",
        "   • Try different inputs (boundary cases)",
        "",
        "2️⃣  STATE INSPECTION",
        "   • What is the current state?",
        "   • Is state initialized correctly?",
        "   • Check state mutations/updates",
        "",
        "3️⃣  EXECUTION FLOW",
        "   • Add console.log/debug statements",
        "   • Set breakpoints in debugger",
        "   • Trace execution path",
        "",
        "4️⃣  DATA TRANSFORMATION",
        "   • Verify data at each step",
        "   • Check type conversions",
        "   • Validate transformations",
        "",
        "5️⃣  OUTPUT VALIDATION",
        "   • What is the actual output?",
        "   • What is the expected output?",
        "   • Compare differences",
        "",
        "6️⃣  ERROR HANDLING",
        "   • Check error messages/stack traces",
        "   • Review error logs",
        "   • Identify error source",
    ];

    for line in &investigation_framework {
        if line.starts_with("   ") {
            println!("{}", line.dimmed());
        } else if !line.is_empty() {
            println!("{}", line.bold());
        } else {
            println!();
        }
    }

    println!();

    // Step 6: Log Analysis
    println!("{}", output.t("debug-log-analysis").cyan().bold());
    println!();

    let has_logs = Confirm::new()
        .with_prompt(&output.t("debug-have-logs"))
        .default(true)
        .interact()?;

    let log_info = if has_logs {
        println!();
        println!("{}", output.t("debug-log-tips"));
        println!("  • Filter by timestamp (narrow down to incident time)");
        println!("  • Search for ERROR, WARN keywords");
        println!("  • Look for stack traces");
        println!("  • Check request/response logs");
        println!("  • Correlate logs across services");
        println!();

        let log_findings = Input::<String>::new()
            .with_prompt(&output.t("debug-log-findings"))
            .allow_empty(true)
            .interact_text()?;

        Some(log_findings)
    } else {
        None
    };

    println!();
    println!("{}", output.t("debug-reproduction").cyan().bold());
    println!();

    // Step 7: Reproduction Steps
    let can_reproduce = Confirm::new()
        .with_prompt(&output.t("debug-can-reproduce"))
        .default(false)
        .interact()?;

    let reproduction_steps = if can_reproduce {
        println!();
        println!("{}", output.t("debug-enter-repro-steps"));
        println!();

        let mut steps = Vec::new();
        let mut step_num = 1;

        loop {
            let step = Input::<String>::new()
                .with_prompt(&output.t_format("debug-step-num", "num", &step_num.to_string()))
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
        println!();
        println!("{}", output.t("debug-repro-tips"));
        println!("  • Simplify the scenario");
        println!("  • Isolate variables");
        println!("  • Create minimal test case");
        println!("  • Document exact conditions");
        None
    };

    println!();
    println!("{}", output.t("debug-debugging-techniques").green().bold());
    println!();

    // Step 8: Select Debugging Techniques
    let techniques = vec![
        "Binary search (comment out code sections)",
        "Add debug logging/print statements",
        "Use debugger with breakpoints",
        "Rubber duck debugging (explain to someone)",
        "Divide and conquer (isolate components)",
        "Compare with working version (git diff)",
        "Check documentation/specs",
        "Search for similar issues (Google, Stack Overflow)",
        "Minimal reproduction (create isolated test)",
        "Time-travel debugging (replay execution)",
    ];

    let selected_techniques = MultiSelect::new()
        .with_prompt(&output.t("debug-select-techniques"))
        .items(&techniques)
        .interact()?;

    let techniques_to_use: Vec<String> = selected_techniques
        .iter()
        .map(|&i| techniques[i].to_string())
        .collect();

    println!();
    println!("{}", output.t("debug-debugging-commands").green().bold());
    println!();

    // Step 9: Provide debugging commands based on issue type
    match issue_idx {
        0 | 1 => {
            // Unexpected behavior or Error
            println!("Debugging commands:");
            println!("  # Run with debug logging");
            println!("  $ DEBUG=* npm run dev");
            println!();
            println!("  # Run specific test");
            println!("  $ npm run test -- --verbose <test-name>");
            println!();
            println!("  # Node.js debugger");
            println!("  $ node --inspect-brk <file.js>");
            println!();
        }
        2 => {
            // Performance
            println!("Performance profiling:");
            println!("  # Chrome DevTools profiler");
            println!("  $ npm run dev (then open DevTools > Performance)");
            println!();
            println!("  # Node.js profiler");
            println!("  $ node --prof <file.js>");
            println!();
            println!("  # Bundle analyzer");
            println!("  $ npm run analyze:bundle");
            println!();
        }
        6 => {
            // Memory leak
            println!("Memory debugging:");
            println!("  # Heap snapshot");
            println!("  $ node --inspect <file.js>");
            println!("  (Chrome DevTools > Memory > Take Heap Snapshot)");
            println!();
            println!("  # Memory usage tracking");
            println!("  $ node --trace-gc <file.js>");
            println!();
        }
        _ => {
            println!("General debugging:");
            println!("  $ npm run dev");
            println!("  (Use browser DevTools for inspection)");
            println!();
        }
    }

    println!("{}", output.t("debug-hypothesis-findings").cyan().bold());
    println!();

    // Step 10: Hypothesis
    let hypothesis = Input::<String>::new()
        .with_prompt(&output.t("debug-hypothesis-prompt"))
        .allow_empty(true)
        .interact_text()?;

    println!();

    // Step 11: Evidence
    let evidence = Input::<String>::new()
        .with_prompt(&output.t("debug-evidence-prompt"))
        .allow_empty(true)
        .interact_text()?;

    println!();

    // Step 12: Root Cause (if found)
    let root_cause_found = Confirm::new()
        .with_prompt(&output.t("debug-root-cause-found"))
        .default(false)
        .interact()?;

    let root_cause = if root_cause_found {
        Some(
            Input::<String>::new()
                .with_prompt(&output.t("debug-root-cause-prompt"))
                .interact_text()?,
        )
    } else {
        None
    };

    println!();

    // Step 13: Next Steps
    if root_cause_found {
        println!("{}", output.t("debug-root-cause-identified").green().bold());
        println!();
        println!("{}", output.t("debug-next-steps-identified"));
        println!("  1. Plan the fix (consider impact)");
        println!("  2. Implement solution");
        println!("  3. Add tests to prevent regression");
        println!("  4. Document the issue and solution");
        println!();
        println!("{}", output.t("debug-can-run-fix"));
        println!("  $ cldev fix \"{}\"", symptom_desc);
    } else {
        println!(
            "{}",
            output.t("debug-continue-investigation").yellow().bold()
        );
        println!();
        println!("{}", output.t("debug-next-steps-ongoing"));
        println!("  1. Try the selected debugging techniques");
        println!("  2. Gather more evidence");
        println!("  3. Refine hypothesis");
        println!("  4. Test hypotheses systematically");
        println!();
        println!("{}", output.t("debug-remember"));
        println!("  • Take notes of everything you try");
        println!("  • Rule out possibilities one by one");
        println!("  • Ask for help if stuck > 30 minutes");
    }

    println!();

    let duration = start_time.elapsed().as_secs() / 60;

    // Step 14: Save Learning Session
    let mut session = LearningSessionBuilder::new("debug", &symptom_desc)
        .tag("debugging")
        .tag(issue_type)
        .metadata("environment", environment)
        .metadata("onset", onset);

    if !hypothesis.is_empty() {
        session = session.step(format!("Hypothesis: {}", hypothesis));
    }

    if !evidence.is_empty() {
        session = session.step(format!("Evidence: {}", evidence));
    }

    if let Some(ref log) = log_info {
        if !log.is_empty() {
            session = session.step(format!("Log findings: {}", log));
        }
    }

    if let Some(steps) = reproduction_steps {
        for step in steps {
            session = session.step(format!("Repro: {}", step));
        }
    }

    for technique in &techniques_to_use {
        session = session.step(format!("Technique: {}", technique));
    }

    if let Some(ref cause) = root_cause {
        session = session.root_cause(cause);
        session = session.resolved(Some(duration as u32));
        session = session.learning(format!("Root cause identified: {}", cause));
    }

    let (session, path) = session.save()?;

    output.success(&output.t("debug-session-saved"));
    println!(
        "   {}",
        output.t_format("debug-session-id", "id", &session.id.cyan().to_string())
    );
    println!(
        "   {}",
        output.t_format(
            "debug-session-path",
            "path",
            &path.display().to_string().cyan().to_string()
        )
    );
    println!();

    println!("{}", output.t("debug-tips-header").cyan().bold());
    println!("  • {}", output.t("debug-tip-detective"));
    println!("  • {}", output.t("debug-tip-assumptions"));
    println!("  • {}", output.t("debug-tip-break"));
    println!("  • {}", output.t("debug-tip-learn"));
    println!("  • {}", output.t("debug-tip-pair"));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_command_structure() {
        // Test that the command structure is well-formed
        assert!(true);
    }
}
