use crate::cli::output::OutputHandler;
use crate::core::session_recorder::LearningSessionBuilder;
use crate::core::{CldevError, Result};
use colored::*;
use dialoguer::{Confirm, Input, MultiSelect, Select};
use std::time::Instant;

/// Technical research and learning session workflow
///
/// This command provides a structured approach for technical research:
/// - Topic definition and research goals
/// - Systematic investigation
/// - Experimentation and hands-on learning
/// - Knowledge capture and documentation
/// - Automatic learning session recording
pub fn handle_research(topic: Option<String>, output: &OutputHandler) -> Result<()> {
    let start_time = Instant::now();

    println!(
        "{}",
        "🔬 RESEARCH: Technical Investigation & Learning"
            .cyan()
            .bold()
    );
    println!("{}", "━".repeat(60).cyan());
    println!();

    // Step 1: Research Topic
    let research_topic = if let Some(t) = topic {
        t
    } else {
        Input::<String>::new()
            .with_prompt(output.t("research-topic-prompt"))
            .interact_text()?
    };

    println!();

    // Step 2: Research Motivation
    println!("{}", "❓ RESEARCH MOTIVATION".cyan().bold());
    println!();

    let motivations = vec![
        output.t("research-motivation-learn"),
        output.t("research-motivation-solve"),
        output.t("research-motivation-evaluate"),
        output.t("research-motivation-best-practices"),
        output.t("research-motivation-performance"),
        output.t("research-motivation-architecture"),
        output.t("research-motivation-security"),
        output.t("research-motivation-compatibility"),
        output.t("research-motivation-trends"),
        output.t("research-motivation-personal"),
    ];

    let motivation_idx = Select::new()
        .with_prompt(output.t("research-motivation-prompt"))
        .items(&motivations)
        .interact()?;

    let motivation = motivations[motivation_idx].clone();

    println!();

    let context = Input::<String>::new()
        .with_prompt(output.t("research-context-prompt"))
        .interact_text()?;

    println!();

    // Step 3: Research Questions
    println!("{}", "❓ RESEARCH QUESTIONS".cyan().bold());
    println!("What specific questions do you want to answer?");
    println!("(Press Enter twice when done)");
    println!();

    let mut research_questions = Vec::new();
    let mut question_num = 1;

    loop {
        let question = Input::<String>::new()
            .with_prompt(output.t("research-question-prompt").replace("{num}", &question_num.to_string()))
            .allow_empty(true)
            .interact_text()?;

        if question.is_empty() {
            break;
        }

        research_questions.push(question);
        question_num += 1;
    }

    if research_questions.is_empty() {
        println!(
            "{}",
            "⚠️  No specific questions - general exploration".yellow()
        );
    }

    println!();

    // Step 4: Research Scope
    println!("{}", "🎯 RESEARCH SCOPE".cyan().bold());
    println!();

    let scope_options = vec![
        output.t("research-scope-quick"),
        output.t("research-scope-moderate"),
        output.t("research-scope-deep"),
        output.t("research-scope-extended"),
    ];

    let scope_idx = Select::new()
        .with_prompt(output.t("research-scope-prompt"))
        .items(&scope_options)
        .interact()?;

    let scope = scope_options[scope_idx].clone();

    println!();

    // Step 5: Research Activities
    println!("{}", "📋 RESEARCH ACTIVITIES".cyan().bold());
    println!();

    let activities = vec![
        output.t("research-activity-docs"),
        output.t("research-activity-blogs"),
        output.t("research-activity-videos"),
        output.t("research-activity-source"),
        output.t("research-activity-experiment"),
        output.t("research-activity-poc"),
        output.t("research-activity-benchmark"),
        output.t("research-activity-security"),
        output.t("research-activity-community"),
        output.t("research-activity-compare"),
        output.t("research-activity-papers"),
        output.t("research-activity-experts"),
    ];

    let selected_activities = MultiSelect::new()
        .with_prompt(output.t("research-activity-prompt"))
        .items(&activities)
        .interact()?;

    let activity_list: Vec<String> = selected_activities
        .iter()
        .map(|&i| activities[i].to_string())
        .collect();

    if activity_list.is_empty() {
        return Err(CldevError::config(
            "At least one research activity must be selected",
        ));
    }

    println!();

    // Step 6: Key Resources
    println!("{}", "🔗 KEY RESOURCES".cyan().bold());
    println!("Enter important resources/URLs (press Enter twice when done):");
    println!();

    let mut resources = Vec::new();
    loop {
        let resource = Input::<String>::new()
            .with_prompt(output.t("research-resource-prompt"))
            .allow_empty(true)
            .interact_text()?;

        if resource.is_empty() {
            break;
        }

        resources.push(resource);
    }

    println!();

    // Step 7: Experimentation Plan
    let will_experiment = Confirm::new()
        .with_prompt(output.t("research-experiment-question"))
        .default(true)
        .interact()?;

    let mut experiments = Vec::new();

    if will_experiment {
        println!();
        println!("{}", "🧪 EXPERIMENTATION PLAN".green().bold());
        println!("Enter experiments/POCs to build (press Enter twice when done):");
        println!("Examples:");
        println!("  - Create minimal React app with new library");
        println!("  - Benchmark algorithm performance");
        println!("  - Test integration with external API");
        println!();

        loop {
            let experiment = Input::<String>::new()
                .with_prompt(output.t("research-experiment-prompt"))
                .allow_empty(true)
                .interact_text()?;

            if experiment.is_empty() {
                break;
            }

            experiments.push(experiment);
        }
    }

    println!();

    // Step 8: Evaluation Criteria (if comparing alternatives)
    let mut evaluation_criteria = Vec::new();

    if motivation.contains("Evaluate alternatives") || motivation.contains("decision") {
        println!("{}", "📊 EVALUATION CRITERIA".cyan().bold());
        println!("Enter criteria for comparison (press Enter twice when done):");
        println!("Examples:");
        println!("  - Performance");
        println!("  - Learning curve");
        println!("  - Community support");
        println!("  - License/cost");
        println!("  - Bundle size");
        println!();

        loop {
            let criterion = Input::<String>::new()
                .with_prompt("Criterion (Space to select, Enter to confirm)")
                .allow_empty(true)
                .interact_text()?;

            if criterion.is_empty() {
                break;
            }

            evaluation_criteria.push(criterion);
        }
    }

    println!();

    // Step 9: Research Progress Tracking
    println!("{}", "📝 RESEARCH TRACKER".cyan().bold());
    println!();
    println!("Track your progress through the research:");
    println!();

    for (i, activity) in activity_list.iter().enumerate() {
        println!("  {}. {}", i + 1, activity.dimmed());
    }

    println!();

    // Step 10: Findings and Insights
    println!("{}", "💡 FINDINGS & INSIGHTS".cyan().bold());
    println!();

    let has_findings = Confirm::new()
        .with_prompt("Have you completed the research and gathered findings? (Space to select, Enter to confirm)")
        .default(false)
        .interact()?;

    let mut key_findings = Vec::new();
    let mut learnings = Vec::new();
    let mut recommendations = String::new();

    if has_findings {
        println!();
        println!("Enter key findings (press Enter twice when done):");
        println!();

        loop {
            let finding = Input::<String>::new()
                .with_prompt("Finding (Space to select, Enter to confirm)")
                .allow_empty(true)
                .interact_text()?;

            if finding.is_empty() {
                break;
            }

            key_findings.push(finding);
        }

        println!();
        println!("Enter key learnings (press Enter twice when done):");
        println!();

        loop {
            let learning = Input::<String>::new()
                .with_prompt("Learning (Space to select, Enter to confirm)")
                .allow_empty(true)
                .interact_text()?;

            if learning.is_empty() {
                break;
            }

            learnings.push(learning);
        }

        println!();

        recommendations = Input::<String>::new()
            .with_prompt("💡 Recommendations (what should be done based on this research?) (Space to select, Enter to confirm)")
            .allow_empty(true)
            .interact_text()?;

        println!();
    }

    // Step 11: Answered Questions
    if !research_questions.is_empty() && has_findings {
        println!("{}", "✅ QUESTIONS ANSWERED".green().bold());
        println!();

        let mut answers = Vec::new();

        for (i, question) in research_questions.iter().enumerate() {
            println!("Question {}: {}", i + 1, question.cyan());

            let answer = Input::<String>::new()
                .with_prompt("Answer (Space to select, Enter to confirm)")
                .allow_empty(true)
                .interact_text()?;

            if !answer.is_empty() {
                answers.push((question.clone(), answer));
            }

            println!();
        }
    }

    // Step 12: Next Actions
    println!("{}", "🎯 NEXT ACTIONS".cyan().bold());
    println!();

    let next_action_options = vec![
        "Apply findings to current project",
        "Create POC/prototype",
        "Share findings with team",
        "Write documentation/blog post",
        "Plan implementation/migration",
        "Further research needed",
        "Archive for future reference",
    ];

    let action_indices = MultiSelect::new()
        .with_prompt("Select next actions (Space to select, Enter to confirm)")
        .items(&next_action_options)
        .interact()?;

    let next_actions: Vec<String> = action_indices
        .iter()
        .map(|&i| next_action_options[i].to_string())
        .collect();

    println!();

    // Step 13: Research Status
    let status_options = vec![
        output.t("research-status-planning"),
        output.t("research-status-progress"),
        output.t("research-status-experimenting"),
        output.t("research-status-analyzing"),
        output.t("research-status-completed"),
    ];

    let status_idx = Select::new()
        .with_prompt(output.t("research-status-prompt"))
        .items(&status_options)
        .default(if has_findings { 4 } else { 1 })
        .interact()?;

    let current_status = status_options[status_idx].clone();
    let is_completed = status_idx == 4;

    let duration = start_time.elapsed().as_secs() / 60;

    // Step 14: Save Learning Session
    let mut session = LearningSessionBuilder::new("research", &research_topic)
        .tag("research")
        .tag("learning")
        .metadata("motivation", motivation)
        .metadata("scope", scope)
        .metadata("status", current_status)
        .metadata("context", &context);

    // Add research questions
    for question in &research_questions {
        session = session.step(format!("Question: {}", question));
    }

    // Add activities
    for activity in &activity_list {
        session = session.step(format!("Activity: {}", activity));
    }

    // Add resources
    for resource in &resources {
        session = session.step(format!("Resource: {}", resource));
    }

    // Add experiments
    for experiment in &experiments {
        session = session.step(format!("Experiment: {}", experiment));
    }

    // Add evaluation criteria
    for criterion in &evaluation_criteria {
        session = session.step(format!("Criterion: {}", criterion));
    }

    // Add findings
    for finding in &key_findings {
        session = session.step(format!("Finding: {}", finding));
    }

    // Add learnings
    for learning in &learnings {
        session = session.learning(learning);
    }

    // Add recommendations
    if !recommendations.is_empty() {
        session = session.solution(&recommendations);
    }

    // Add next actions
    for action in &next_actions {
        session = session.step(format!("Next: {}", action));
    }

    if is_completed {
        session = session.resolved(Some(duration as u32));
    }

    let (session, path) = session.save()?;

    println!();
    println!("{}", "✅ Research session saved".green());
    println!("   Session ID: {}", session.id.cyan());
    println!("   Path: {}", path.display().to_string().cyan());
    println!();

    // Step 15: Summary and Next Steps
    if is_completed {
        println!("{}", "🎓 RESEARCH SUMMARY".green().bold());
        println!("{}", "━".repeat(60).green());
        println!();

        println!("{}", format!("Topic: {}", research_topic).bold());
        println!("Time spent: {} minutes", duration);
        println!();

        if !key_findings.is_empty() {
            println!("{}", "Key Findings:".bold());
            for finding in &key_findings {
                println!("  • {}", finding);
            }
            println!();
        }

        if !learnings.is_empty() {
            println!("{}", "Key Learnings:".bold());
            for learning in &learnings {
                println!("  • {}", learning.green());
            }
            println!();
        }

        if !recommendations.is_empty() {
            println!("{}", "Recommendations:".bold());
            println!("  {}", recommendations);
            println!();
        }

        if !next_actions.is_empty() {
            println!("{}", "Next Actions:".bold());
            for (i, action) in next_actions.iter().enumerate() {
                println!("  {}. {}", i + 1, action);
            }
            println!();
        }

        println!("{}", "🎉 Research completed!".green().bold());
        println!();
        println!("Learning session saved to:");
        println!("  {}", path.display().to_string().cyan());
        println!();
    } else {
        println!("{}", "🔄 RESEARCH IN PROGRESS".cyan().bold());
        println!();

        match status_idx {
            0 => {
                println!("Next steps:");
                println!("  1. Review research questions");
                println!("  2. Gather initial resources");
                println!("  3. Start with documentation/articles");
                println!("  4. Plan hands-on experiments");
            }
            1 => {
                println!("Keep researching:");
                println!("  • Take notes as you go");
                println!("  • Bookmark useful resources");
                println!("  • Try to answer research questions systematically");
                println!("  • Don't get lost in rabbit holes (time-box!)");
            }
            2 => {
                println!("Experimentation tips:");
                println!("  • Start with minimal examples");
                println!("  • Document unexpected behaviors");
                println!("  • Compare with alternatives if applicable");
                println!("  • Save code snippets for future reference");
            }
            3 => {
                println!("Synthesize findings:");
                println!("  • Organize notes and findings");
                println!("  • Answer original research questions");
                println!("  • Identify key takeaways");
                println!("  • Formulate recommendations");
            }
            _ => {}
        }
        println!();

        println!("Re-run this command when you have findings:");
        println!("  $ cldev research \"{}\"", research_topic);
    }

    println!();
    println!("{}", "💡 RESEARCH BEST PRACTICES".cyan().bold());
    println!("  • Start with official documentation");
    println!("  • Verify information from multiple sources");
    println!("  • Hands-on experimentation beats reading alone");
    println!("  • Document as you go (don't rely on memory)");
    println!("  • Share findings with team (knowledge multiplier)");
    println!("  • Save useful resources for future reference");
    println!("  • Time-box research to avoid analysis paralysis");
    println!("  • Focus on answering specific questions");

    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_research_command_structure() {
        // Test that the command structure is well-formed
    }
}
