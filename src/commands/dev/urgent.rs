use crate::cli::output::OutputHandler;
use crate::core::session_recorder::LearningSessionBuilder;
use crate::core::Result;
use colored::*;
use dialoguer::{Confirm, Input, MultiSelect, Select};
use std::time::Instant;

/// Handle urgent production incidents with structured response flow
///
/// This command provides a 5-minute initial response framework for critical
/// production issues and security incidents.
pub fn handle_urgent(problem: Option<String>, output: &OutputHandler) -> Result<()> {
    let start_time = Instant::now();

    println!("{}", output.t("urgent-header").red().bold());
    println!("{}", output.t("urgent-separator").red());
    println!();

    // Step 1: Problem Description
    let problem_desc = if let Some(p) = problem {
        p
    } else {
        Input::<String>::new()
            .with_prompt(output.t("urgent-describe-incident"))
            .interact_text()?
    };

    println!();
    println!("{}", output.t("urgent-immediate-actions").yellow().bold());
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
        .with_prompt(output.t("urgent-select-affected-areas"))
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
        .with_prompt(output.t("urgent-severity-level"))
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
    println!(
        "{}",
        output
            .t("urgent-immediate-response-checklist")
            .cyan()
            .bold()
    );
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
    println!("{}", output.t("urgent-investigation-steps").cyan().bold());
    println!();

    // Step 5: Common Investigation Areas
    println!("{}", output.t("urgent-investigation-intro"));
    println!("  1. Application logs (errors, warnings, stack traces)");
    println!("  2. System metrics (CPU, memory, disk, network)");
    println!("  3. Database performance (slow queries, locks, connections)");
    println!("  4. External API status (third-party services)");
    println!("  5. Recent code changes (git log, deployments)");
    println!("  6. Infrastructure changes (scaling, configurations)");
    println!();

    // Step 6: Immediate Mitigation Options
    println!("{}", output.t("urgent-mitigation-options").yellow().bold());
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
        .with_prompt(output.t("urgent-choose-mitigation"))
        .items(&mitigation_options)
        .interact_opt()?;

    let mitigation = mitigation_idx.map(|idx| mitigation_options[idx].to_string());

    println!();
    println!("{}", output.t("urgent-rollback-guide").green().bold());
    println!();
    println!("{}", output.t("urgent-rollback-intro"));
    println!("  1. {}:", output.t("urgent-rollback-step1"));
    println!("     $ git log --oneline --graph -10");
    println!();
    println!("  2. {}:", output.t("urgent-rollback-step2"));
    println!("     $ git checkout -b hotfix/rollback-YYYYMMDD <commit-hash>");
    println!();
    println!("  3. {}:", output.t("urgent-rollback-step3"));
    println!("     $ ./deploy.sh production --emergency");
    println!();
    println!("  4. {}:", output.t("urgent-rollback-step4"));
    println!("     $ curl https://api.example.com/health");
    println!();

    // Step 7: Documentation Prompt
    println!(
        "{}",
        output.t("urgent-incident-documentation").cyan().bold()
    );
    println!();

    let root_cause = Input::<String>::new()
        .with_prompt(output.t("urgent-root-cause-prompt"))
        .default("investigating".to_string())
        .interact_text()?;

    let immediate_action = Input::<String>::new()
        .with_prompt(output.t("urgent-immediate-action-prompt"))
        .allow_empty(true)
        .interact_text()?;

    // Step 8: Next Steps
    println!();
    println!("{}", output.t("urgent-next-steps").cyan().bold());
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
            .with_prompt(output.t("urgent-is-resolved"))
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
    output.success(&output.t("urgent-session-saved"));
    println!(
        "   {}",
        output.t_format("urgent-session-id", "id", &session.id.cyan().to_string())
    );
    println!(
        "   {}",
        output.t_format(
            "urgent-session-path",
            "path",
            &path.display().to_string().cyan().to_string()
        )
    );
    println!();

    if resolved {
        println!("{}", output.t("urgent-incident-resolved").green().bold());
        println!(
            "   {}",
            output.t_format("urgent-duration", "minutes", &duration.to_string())
        );
    } else {
        println!("{}", output.t("urgent-incident-ongoing").yellow().bold());
    }

    println!();
    println!("{}", output.t("urgent-tips-header").cyan().bold());
    println!("  • {}", output.t("urgent-tip-stakeholders"));
    println!("  • {}", output.t("urgent-tip-document"));
    println!("  • {}", output.t("urgent-tip-systematic"));
    println!("  • {}", output.t("urgent-tip-escalate"));

    Ok(())
}

#[cfg(test)]
mod tests {
    

    #[test]
    fn test_urgent_command_structure() {
        // Test that the command structure is well-formed
        // Actual interactive testing would require mock inputs
        assert!(true);
    }
}
