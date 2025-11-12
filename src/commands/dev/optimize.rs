use crate::cli::output::OutputHandler;
use crate::core::session_recorder::LearningSessionBuilder;
use crate::core::{CldevError, Result};
use colored::*;
use dialoguer::{Confirm, Input, MultiSelect, Select};
use std::time::Instant;

/// Performance optimization workflow with scientific measurement
///
/// This command provides a data-driven approach to optimization:
/// - Baseline measurement (before optimization)
/// - Bottleneck identification
/// - Optimization implementation
/// - Performance verification (after optimization)
/// - Before/after comparison
pub fn handle_optimize(target: Option<String>, output: &OutputHandler) -> Result<()> {
    let start_time = Instant::now();

    println!("{}", output.t("optimize-header").magenta().bold());
    println!("{}", output.t("optimize-separator").magenta());
    println!();

    // Step 1: Optimization Target
    let optimize_target = if let Some(t) = target {
        t
    } else {
        Input::<String>::new()
            .with_prompt(output.t("optimize-target-prompt"))
            .interact_text()?
    };

    println!();

    // Step 2: Performance Issue Type
    println!(
        "{}",
        output.t("optimize-issue-classification").cyan().bold()
    );
    println!();

    let issue_types = vec![
        output.t("optimize-issue-page-load"),
        output.t("optimize-issue-api"),
        output.t("optimize-issue-database"),
        output.t("optimize-issue-memory"),
        output.t("optimize-issue-cpu"),
        output.t("optimize-issue-network"),
        output.t("optimize-issue-bundle"),
        output.t("optimize-issue-image"),
        output.t("optimize-issue-animation"),
        output.t("optimize-issue-search"),
    ];

    let issue_indices = MultiSelect::new()
        .with_prompt(output.t("optimize-issue-prompt"))
        .items(&issue_types)
        .interact()?;

    let performance_issues: Vec<String> = issue_indices
        .iter()
        .map(|&i| issue_types[i].to_string())
        .collect();

    if performance_issues.is_empty() {
        return Err(CldevError::config(
            "At least one performance issue must be selected",
        ));
    }

    println!();

    // Step 3: Baseline Measurement
    println!("{}", "📊 BASELINE MEASUREMENT".cyan().bold());
    println!();
    println!("Performance optimization must be data-driven!");
    println!("Measure current performance before making changes.");
    println!();

    let has_baseline = Confirm::new()
        .with_prompt(output.t("optimize-baseline-question"))
        .default(false)
        .interact()?;

    let mut baseline_metrics = Vec::new();

    if !has_baseline {
        println!();
        println!(
            "{}",
            "⚠️  IMPORTANT: Measure baseline first!".yellow().bold()
        );
        println!();
        println!("Recommended measurement tools:");
        println!();

        // Provide tool recommendations based on issue type
        if performance_issues
            .iter()
            .any(|i| i.contains("page load") || i.contains("rendering"))
        {
            println!("  Frontend Performance:");
            println!("    • Chrome DevTools Performance tab");
            println!("    • Lighthouse (Web Vitals: LCP, FID, CLS)");
            println!("    • WebPageTest.org");
            println!("    $ npm run analyze:lighthouse");
            println!();
        }

        if performance_issues
            .iter()
            .any(|i| i.contains("API") || i.contains("query"))
        {
            println!("  Backend Performance:");
            println!("    • API response time metrics");
            println!("    • Database query EXPLAIN plans");
            println!("    • APM tools (New Relic, Datadog, etc.)");
            println!("    • Server-side profiling");
            println!();
        }

        if performance_issues.iter().any(|i| i.contains("Bundle size")) {
            println!("  Bundle Analysis:");
            println!("    • webpack-bundle-analyzer");
            println!("    • source-map-explorer");
            println!("    $ npm run analyze:bundle");
            println!();
        }

        if performance_issues.iter().any(|i| i.contains("Memory")) {
            println!("  Memory Profiling:");
            println!("    • Chrome DevTools Memory tab (Heap snapshots)");
            println!("    • Performance Monitor");
            println!("    • Node.js --inspect for backend");
            println!();
        }

        println!("Run measurements and return to this command.");
        println!();

        let continue_anyway = Confirm::new()
            .with_prompt(output.t("optimize-continue-no-baseline"))
            .default(false)
            .interact()?;

        if !continue_anyway {
            println!();
            println!("{}", "✅ Good! Measure first, optimize second.".green());
            println!();
            println!("Next steps:");
            println!("  1. Run appropriate measurement tools");
            println!("  2. Record baseline metrics");
            println!("  3. Re-run: cldev optimize \"{}\"", optimize_target);
            return Ok(());
        }
    } else {
        println!();
        println!("Enter baseline metrics (press Enter twice when done):");
        println!("Examples:");
        println!("  - Page load time: 3.2s");
        println!("  - API response: 450ms");
        println!("  - Bundle size: 850KB");
        println!("  - Memory usage: 120MB");
        println!("  - LCP: 2.8s");
        println!();

        loop {
            let metric = Input::<String>::new()
                .with_prompt(output.t("optimize-baseline-prompt"))
                .allow_empty(true)
                .interact_text()?;

            if metric.is_empty() {
                break;
            }

            baseline_metrics.push(metric);
        }

        if baseline_metrics.is_empty() {
            println!("{}", "⚠️  Warning: No baseline metrics recorded".yellow());
        }
    }

    println!();

    // Step 4: Bottleneck Identification
    println!("{}", "🔬 BOTTLENECK ANALYSIS".cyan().bold());
    println!();

    let bottleneck_sources = vec![
        output.t("optimize-bottleneck-images"),
        output.t("optimize-bottleneck-javascript"),
        output.t("optimize-bottleneck-rerenders"),
        output.t("optimize-bottleneck-blocking"),
        output.t("optimize-bottleneck-n-plus-one"),
        output.t("optimize-bottleneck-indexes"),
        output.t("optimize-bottleneck-data"),
        output.t("optimize-bottleneck-algorithms"),
        output.t("optimize-bottleneck-memory-leaks"),
        output.t("optimize-bottleneck-requests"),
        output.t("optimize-bottleneck-libraries"),
        output.t("optimize-bottleneck-css"),
    ];

    let bottleneck_indices = MultiSelect::new()
        .with_prompt(output.t("optimize-bottleneck-prompt"))
        .items(&bottleneck_sources)
        .interact()?;

    let bottlenecks: Vec<String> = bottleneck_indices
        .iter()
        .map(|&i| bottleneck_sources[i].to_string())
        .collect();

    println!();

    // Step 5: Optimization Strategy
    println!("{}", "🛠️  OPTIMIZATION TECHNIQUES".green().bold());
    println!();

    let optimization_techniques = vec![
        // Frontend
        output.t("optimize-technique-code-splitting"),
        output.t("optimize-technique-images"),
        output.t("optimize-technique-memoization"),
        output.t("optimize-technique-virtualization"),
        output.t("optimize-technique-debounce"),
        output.t("optimize-technique-bundle-reduction"),
        output.t("optimize-technique-css"),
        output.t("optimize-technique-web-workers"),
        output.t("optimize-technique-service-worker"),
        // Backend
        output.t("optimize-technique-query"),
        output.t("optimize-technique-caching"),
        output.t("optimize-technique-pooling"),
        output.t("optimize-technique-async"),
        output.t("optimize-technique-batch"),
        output.t("optimize-technique-cdn"),
        // General
        output.t("optimize-technique-algorithm"),
        output.t("optimize-technique-parallel"),
        output.t("optimize-technique-complexity"),
        output.t("optimize-technique-resource-reuse"),
    ];

    let selected_techniques = MultiSelect::new()
        .with_prompt(output.t("optimize-technique-prompt"))
        .items(&optimization_techniques)
        .interact()?;

    let techniques: Vec<String> = selected_techniques
        .iter()
        .map(|&i| optimization_techniques[i].to_string())
        .collect();

    if techniques.is_empty() {
        return Err(CldevError::config(
            "At least one optimization technique must be selected",
        ));
    }

    println!();

    // Step 6: Implementation Plan
    println!("{}", "📝 OPTIMIZATION IMPLEMENTATION PLAN".cyan().bold());
    println!();
    println!("Enter optimization steps (press Enter twice when done):");
    println!();

    let mut optimization_steps = Vec::new();
    let mut step_num = 1;

    loop {
        let step = Input::<String>::new()
            .with_prompt(
                output
                    .t("optimize-step-prompt")
                    .replace("{num}", &step_num.to_string()),
            )
            .allow_empty(true)
            .interact_text()?;

        if step.is_empty() {
            break;
        }

        optimization_steps.push(step);
        step_num += 1;
    }

    println!();

    // Step 7: Files to Modify
    println!("{}", "📁 FILES TO OPTIMIZE".cyan().bold());
    println!("Enter file paths (one per line, press Enter twice when done):");
    println!();

    let mut files = Vec::new();
    loop {
        let file = Input::<String>::new()
            .with_prompt(output.t("optimize-file-prompt"))
            .allow_empty(true)
            .interact_text()?;

        if file.is_empty() {
            break;
        }

        files.push(file);
    }

    println!();

    // Step 8: Testing and Validation
    println!("{}", "✅ VALIDATION CHECKLIST".cyan().bold());
    println!();

    let validation_checks = [
        "Run existing tests (ensure no breakage)",
        "Measure performance after optimization",
        "Compare before/after metrics",
        "Test with realistic data volumes",
        "Verify functionality remains correct",
        "Check for memory leaks",
        "Test on different devices/browsers (if frontend)",
        "Load testing / stress testing",
    ];

    println!("Validation requirements:");
    for (i, check) in validation_checks.iter().enumerate() {
        println!("  {}. {}", i + 1, check);
    }

    println!();

    // Step 9: Performance Budget
    println!("{}", "🎯 PERFORMANCE TARGET".cyan().bold());
    println!();

    let target_improvement = Input::<String>::new()
        .with_prompt(output.t("optimize-target-prompt"))
        .allow_empty(true)
        .interact_text()?;

    println!();

    // Step 10: Benchmark Commands
    println!("{}", "🧪 BENCHMARKING COMMANDS".green().bold());
    println!();
    println!("Use these commands to measure performance:");
    println!();

    if performance_issues
        .iter()
        .any(|i| i.contains("page load") || i.contains("rendering"))
    {
        println!("  Frontend Performance:");
        println!("    $ npm run analyze:lighthouse    # Web Vitals");
        println!("    (Chrome DevTools > Performance > Record)");
        println!();
    }

    if performance_issues.iter().any(|i| i.contains("Bundle")) {
        println!("  Bundle Size:");
        println!("    $ npm run analyze:bundle        # Visualize bundle composition");
        println!("    $ npm run build -- --stats      # Generate stats.json");
        println!();
    }

    if performance_issues
        .iter()
        .any(|i| i.contains("API") || i.contains("Backend"))
    {
        println!("  Backend Performance:");
        println!("    # Use curl with timing");
        println!("    $ curl -w \"@curl-format.txt\" -o /dev/null -s <url>");
        println!("    # Or use Apache Bench");
        println!("    $ ab -n 1000 -c 10 <url>");
        println!();
    }

    println!("  General:");
    println!("    $ npm run test:performance       # Performance tests");
    println!("    $ npm run build                  # Verify build succeeds");
    println!();

    // Step 11: Progress Status
    println!("{}", "📊 OPTIMIZATION STATUS".cyan().bold());
    println!();

    let status_options = vec![
        output.t("optimize-status-planning"),
        output.t("optimize-status-progress"),
        output.t("optimize-status-measuring"),
        output.t("optimize-status-completed"),
    ];

    let status_idx = Select::new()
        .with_prompt(output.t("optimize-status-prompt"))
        .items(&status_options)
        .default(0)
        .interact()?;

    let current_status = status_options[status_idx].clone();
    let is_completed = status_idx == 3;

    println!();

    // Step 12: After Metrics (if completed)
    let mut after_metrics = Vec::new();
    let mut improvements = Vec::new();

    if is_completed {
        println!("{}", "📈 OPTIMIZATION RESULTS".green().bold());
        println!();
        println!("Enter after-optimization metrics (press Enter twice when done):");
        println!();

        loop {
            let metric = Input::<String>::new()
                .with_prompt(output.t("optimize-after-prompt"))
                .allow_empty(true)
                .interact_text()?;

            if metric.is_empty() {
                break;
            }

            after_metrics.push(metric);
        }

        println!();

        if !baseline_metrics.is_empty() && !after_metrics.is_empty() {
            println!("{}", "📊 BEFORE/AFTER COMPARISON".cyan().bold());
            println!();
            println!("  BEFORE:");
            for metric in &baseline_metrics {
                println!("    • {}", metric.dimmed());
            }
            println!();
            println!("  AFTER:");
            for metric in &after_metrics {
                println!("    • {}", metric.green());
            }
            println!();
        }

        println!("Enter measured improvements (press Enter twice when done):");
        println!("Examples:");
        println!("  - Load time improved by 60% (3.2s → 1.3s)");
        println!("  - Bundle size reduced by 40% (850KB → 510KB)");
        println!("  - API response 3x faster (450ms → 150ms)");
        println!();

        loop {
            let improvement = Input::<String>::new()
                .with_prompt(output.t("optimize-improvement-prompt"))
                .allow_empty(true)
                .interact_text()?;

            if improvement.is_empty() {
                break;
            }

            improvements.push(improvement);
        }
    }

    let duration = start_time.elapsed().as_secs() / 60;

    // Step 13: Save Learning Session
    let mut session = LearningSessionBuilder::new("optimize", &optimize_target)
        .tag("performance")
        .tag("optimization")
        .metadata("status", current_status);

    if !target_improvement.is_empty() {
        session = session.metadata("target", &target_improvement);
    }

    // Add performance issues
    for issue in &performance_issues {
        session = session.step(format!("Issue: {}", issue));
    }

    // Add bottlenecks
    for bottleneck in &bottlenecks {
        session = session.step(format!("Bottleneck: {}", bottleneck));
    }

    // Add techniques
    for technique in &techniques {
        session = session.step(format!("Technique: {}", technique));
    }

    // Add baseline metrics
    for metric in &baseline_metrics {
        session = session.step(format!("Baseline: {}", metric));
    }

    // Add optimization steps
    for step in &optimization_steps {
        session = session.step(format!("Plan: {}", step));
    }

    // Add files
    for file in &files {
        session = session.file(file);
    }

    // Add after metrics
    for metric in &after_metrics {
        session = session.step(format!("After: {}", metric));
    }

    // Add improvements
    for improvement in &improvements {
        session = session.learning(improvement);
    }

    if is_completed {
        session = session.resolved(Some(duration as u32));
        session = session.solution(format!(
            "Performance optimized using: {}",
            techniques.join(", ")
        ));
    }

    let (session, path) = session.save()?;

    println!();
    println!("{}", "✅ Optimization session saved".green());
    println!("   Session ID: {}", session.id.cyan());
    println!("   Path: {}", path.display().to_string().cyan());
    println!();

    // Step 14: Next Steps
    match status_idx {
        0 => {
            // Planning
            println!("{}", "📋 NEXT STEPS: Start Optimization".green().bold());
            println!();
            println!("1. Ensure baseline metrics are recorded");
            println!("2. Start with the highest-impact optimization");
            println!("3. Optimize incrementally (one technique at a time)");
            println!("4. Measure after each change");
            println!();
        }
        1 => {
            // In Progress
            println!("{}", "⚡ NEXT STEPS: Continue Optimization".green().bold());
            println!();
            println!("1. Continue implementing optimization techniques");
            println!("2. Test functionality after each change");
            println!("3. Profile/measure intermediate improvements");
            println!("4. Document trade-offs and decisions");
            println!();
        }
        2 => {
            // Measuring
            println!("{}", "📊 NEXT STEPS: Verify Results".green().bold());
            println!();
            println!("1. Run comprehensive performance benchmarks");
            println!("2. Compare with baseline metrics");
            println!("3. Test under realistic load");
            println!("4. Verify no regressions in functionality");
            println!();
        }
        3 => {
            // Completed
            println!("{}", "🎉 OPTIMIZATION COMPLETED!".green().bold());
            println!();
            println!("   Time taken: {} minutes", duration);
            println!();

            if !improvements.is_empty() {
                println!("Verified improvements:");
                for improvement in &improvements {
                    println!("  ✓ {}", improvement.green());
                }
                println!();
            }

            println!("Next steps:");
            println!("  1. Create PR with optimization changes");
            println!("  2. Include before/after metrics in PR description");
            println!("  3. Monitor performance in production");
            println!("  4. Document optimization techniques used");
        }
        _ => {}
    }

    println!();
    println!("{}", "💡 OPTIMIZATION PRINCIPLES".cyan().bold());
    println!("  • Measure first, optimize second (avoid premature optimization)");
    println!("  • Focus on bottlenecks (80/20 rule)");
    println!("  • One optimization at a time (isolate impact)");
    println!("  • Always compare before/after metrics");
    println!("  • Don't sacrifice readability for micro-optimizations");
    println!("  • Test with realistic data and load");
    println!("  • Performance is a feature - monitor it continuously");

    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_optimize_command_structure() {
        // Test that the command structure is well-formed
    }
}
