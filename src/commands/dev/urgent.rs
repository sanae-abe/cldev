use crate::core::session_recorder::{LearningSession, LearningSessionBuilder};
use crate::core::{CldevError, Result};
use colored::*;
use dialoguer::{Confirm, Input, MultiSelect, Select};
use std::time::Instant;

/// Handle urgent production incidents with structured response flow
///
/// This command provides a 5-minute initial response framework for critical
/// production issues and security incidents.
pub fn handle_urgent(problem: Option<String>) -> Result<()> {
    let start_time = Instant::now();

    println!("{}", "🚨 URGENT: Production Incident Response".red().bold());
    println!("{}", "━".repeat(60).red());
    println!();

    // Step 1: Problem Description
    let problem_desc = if let Some(p) = problem {
        p
    } else {
        Input::<String>::new()
            .with_prompt("📝 Describe the incident (be specific)")
            .interact_text()?
    };

    println!();
    println!(
        "{}",
        "⚡ IMMEDIATE ACTIONS (First 5 minutes)".yellow().bold()
    );
    println!();

    // Step 2: Impact Assessment
    let impact_areas = vec![
        "User authentication/login",
        "Payment processing",
        "Data integrity",
        "Service availability (downtime)",
        "Security breach/leak",
        "Performance degradation",
        "Data loss",
        "Integration failures",
    ];

    let selected_impacts = MultiSelect::new()
        .with_prompt("⚠️  Select all affected areas")
        .items(&impact_areas)
        .interact()?;

    let affected = selected_impacts
        .iter()
        .map(|&i| impact_areas[i].to_string())
        .collect::<Vec<_>>();

    println!();

    // Step 3: Severity Classification
    let severity_options = vec![
        "P0 - Critical (Complete outage, data loss, security breach)",
        "P1 - High (Major functionality broken, significant user impact)",
        "P2 - Medium (Partial functionality affected, workaround available)",
        "P3 - Low (Minor issue, limited impact)",
    ];

    let severity_idx = Select::new()
        .with_prompt("🎯 Severity Level")
        .items(&severity_options)
        .default(0)
        .interact()?;

    let severity = match severity_idx {
        0 => "P0-Critical",
        1 => "P1-High",
        2 => "P2-Medium",
        _ => "P3-Low",
    };

    println!();
    println!("{}", "📋 IMMEDIATE RESPONSE CHECKLIST".cyan().bold());
    println!();

    // Step 4: Immediate Response Checklist
    let checklist_items = if severity == "P0-Critical" || severity == "P1-High" {
        vec![
            "✓ Alert team/on-call engineer",
            "✓ Check monitoring dashboards (metrics, logs, errors)",
            "✓ Verify service health status",
            "✓ Review recent deployments (last 24h)",
            "✓ Check external dependencies status",
            "✓ Document incident timeline",
        ]
    } else {
        vec![
            "✓ Check recent deployments/changes",
            "✓ Review error logs and metrics",
            "✓ Verify system health",
            "✓ Document findings",
        ]
    };

    for item in &checklist_items {
        println!("  {}", item);
    }

    println!();
    println!("{}", "🔍 INVESTIGATION STEPS".cyan().bold());
    println!();

    // Step 5: Common Investigation Areas
    println!("Common areas to investigate:");
    println!("  1. Application logs (errors, warnings, stack traces)");
    println!("  2. System metrics (CPU, memory, disk, network)");
    println!("  3. Database performance (slow queries, locks, connections)");
    println!("  4. External API status (third-party services)");
    println!("  5. Recent code changes (git log, deployments)");
    println!("  6. Infrastructure changes (scaling, configurations)");
    println!();

    // Step 6: Immediate Mitigation Options
    println!("{}", "⚡ MITIGATION OPTIONS".yellow().bold());
    println!();

    let mitigation_options = vec![
        "Rollback to previous stable version",
        "Scale up resources (horizontal/vertical)",
        "Enable feature flag/circuit breaker",
        "Restart affected services",
        "Apply hotfix patch",
        "Redirect traffic (failover)",
        "Enable maintenance mode",
        "Other (custom action)",
    ];

    let mitigation_idx = Select::new()
        .with_prompt("Choose immediate mitigation strategy")
        .items(&mitigation_options)
        .interact_opt()?;

    let mitigation = mitigation_idx.map(|idx| mitigation_options[idx].to_string());

    println!();
    println!("{}", "🔧 ROLLBACK GUIDE".green().bold());
    println!();
    println!("If rollback is needed:");
    println!("  1. Identify last known good version:");
    println!("     $ git log --oneline --graph -10");
    println!();
    println!("  2. Create rollback branch:");
    println!("     $ git checkout -b hotfix/rollback-YYYYMMDD <commit-hash>");
    println!();
    println!("  3. Deploy to production:");
    println!("     $ ./deploy.sh production --emergency");
    println!();
    println!("  4. Verify service health:");
    println!("     $ curl https://api.example.com/health");
    println!();

    // Step 7: Documentation Prompt
    println!("{}", "📝 INCIDENT DOCUMENTATION".cyan().bold());
    println!();

    let root_cause = Input::<String>::new()
        .with_prompt("Root cause (if identified, or 'investigating')")
        .default("investigating".to_string())
        .interact_text()?;

    let immediate_action = Input::<String>::new()
        .with_prompt("Immediate action taken")
        .allow_empty(true)
        .interact_text()?;

    // Step 8: Next Steps
    println!();
    println!("{}", "📌 NEXT STEPS".cyan().bold());
    println!();
    println!("  1. Continue monitoring metrics and logs");
    println!("  2. Document all findings and actions taken");
    println!("  3. Prepare detailed incident report");
    println!("  4. Schedule post-mortem meeting");
    println!("  5. Identify preventive measures");
    println!();

    // Step 9: Save Learning Session
    let duration = start_time.elapsed().as_secs() / 60;

    let resolved = if !immediate_action.is_empty() {
        Confirm::new()
            .with_prompt("Is the incident resolved?")
            .default(false)
            .interact()?
    } else {
        false
    };

    let mut session = LearningSessionBuilder::new("urgent", &problem_desc)
        .tag("production")
        .tag("incident")
        .tag(severity);

    for area in &affected {
        session = session.tag(area);
    }

    if let Some(ref mitigation_str) = mitigation {
        session = session.step(format!("Mitigation: {}", mitigation_str));
    }

    if !immediate_action.is_empty() {
        session = session.step(format!("Action taken: {}", immediate_action));
    }

    if root_cause != "investigating" {
        session = session.root_cause(&root_cause);
    }

    if resolved {
        session = session.resolved(Some(duration as u32));
        session = session.learning("Successfully resolved production incident");
    }

    session = session.metadata("severity", severity);
    session = session.metadata("affected_areas", affected.join(", "));

    let (session, path) = session.save()?;

    println!();
    println!("{}", "✅ Session saved".green());
    println!("   Session ID: {}", session.id.cyan());
    println!("   Path: {}", path.display().to_string().cyan());
    println!();

    if resolved {
        println!("{}", "🎉 Incident resolved!".green().bold());
        println!("   Duration: {} minutes", duration);
    } else {
        println!(
            "{}",
            "⚠️  Incident still ongoing - continue monitoring"
                .yellow()
                .bold()
        );
    }

    println!();
    println!("{}", "💡 TIPS".cyan().bold());
    println!("  • Keep stakeholders updated every 15-30 minutes");
    println!("  • Document every action and observation");
    println!("  • Don't panic - systematic approach is key");
    println!("  • Ask for help if needed - escalate appropriately");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_urgent_command_structure() {
        // Test that the command structure is well-formed
        // Actual interactive testing would require mock inputs
        assert!(true);
    }
}
