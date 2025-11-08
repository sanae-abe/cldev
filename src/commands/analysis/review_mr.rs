use crate::cli::output::OutputHandler;
use crate::core::error::Result;
use git2::{DiffOptions, Repository};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// MR/PR Review result
#[derive(Debug, Serialize, Deserialize)]
pub struct ReviewResult {
    pub mr_number: u32,
    pub timestamp: String,
    pub summary: ReviewSummary,
    pub security_issues: Vec<SecurityIssue>,
    pub performance_issues: Vec<PerformanceIssue>,
    pub quality_issues: Vec<QualityIssue>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReviewSummary {
    pub files_changed: usize,
    pub lines_added: usize,
    pub lines_removed: usize,
    pub commits: usize,
    pub overall_risk: RiskLevel,
    pub approval_status: ApprovalStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ApprovalStatus {
    Approved,
    ApprovedWithComments,
    ChangesRequested,
    Blocked,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityIssue {
    pub file: String,
    pub line: usize,
    pub severity: String,
    pub category: SecurityCategory,
    pub description: String,
    pub recommendation: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SecurityCategory {
    Injection,
    Authentication,
    Authorization,
    CryptographicFailure,
    InsecureDeserialization,
    XSS,
    CSRF,
    Logging,
    SecretExposure,
    Other,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PerformanceIssue {
    pub file: String,
    pub line: usize,
    pub severity: String,
    pub issue_type: String,
    pub description: String,
    pub suggestion: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualityIssue {
    pub file: String,
    pub line: usize,
    pub severity: String,
    pub issue_type: String,
    pub description: String,
}

/// Review a merge request/pull request
pub fn review_merge_request(
    number: u32,
    detailed: bool,
    security_focus: bool,
    performance_focus: bool,
    output: &OutputHandler,
) -> Result<()> {
    output.info(&format!("Reviewing MR/PR #{}...", number));

    let current_dir = std::env::current_dir()?;
    let repo = Repository::open(&current_dir)?;

    // Get the changes
    let review = perform_review(&repo, number, detailed, security_focus, performance_focus)?;

    // Display results
    display_review(&review, detailed, output);

    // Determine approval status
    let status = determine_approval(&review);
    display_approval(status, output);

    Ok(())
}

fn perform_review(
    repo: &Repository,
    number: u32,
    _detailed: bool,
    security_focus: bool,
    performance_focus: bool,
) -> Result<ReviewResult> {
    // Get HEAD commit
    let head = repo.head()?;
    let head_commit = head.peel_to_commit()?;

    // Get base commit (for simplicity, using HEAD~1, in production would fetch actual MR base)
    let base_commit = head_commit.parent(0)?;

    // Calculate diff
    let mut diff_opts = DiffOptions::new();
    let diff = repo.diff_tree_to_tree(
        Some(&base_commit.tree()?),
        Some(&head_commit.tree()?),
        Some(&mut diff_opts),
    )?;

    // Collect statistics
    let stats = diff.stats()?;
    let files_changed = stats.files_changed();
    let lines_added = stats.insertions();
    let lines_removed = stats.deletions();

    // Analyze changes
    let mut security_issues = Vec::new();
    let mut performance_issues = Vec::new();
    let mut quality_issues = Vec::new();

    // Scan diff for issues
    diff.foreach(
        &mut |delta, _progress| {
            if let Some(new_file) = delta.new_file().path() {
                if let Some(path_str) = new_file.to_str() {
                    // Read file content and analyze
                    if let Ok(content) = std::fs::read_to_string(new_file) {
                        if security_focus || !performance_focus {
                            security_issues.extend(scan_security_issues(path_str, &content));
                        }

                        if performance_focus || !security_focus {
                            performance_issues.extend(scan_performance_issues(path_str, &content));
                        }

                        quality_issues.extend(scan_quality_issues(path_str, &content));
                    }
                }
            }
            true
        },
        None,
        None,
        None,
    )?;

    // Calculate overall risk
    let overall_risk = calculate_risk(&security_issues, &performance_issues, &quality_issues);

    // Generate recommendations
    let recommendations =
        generate_recommendations(&security_issues, &performance_issues, &quality_issues);

    Ok(ReviewResult {
        mr_number: number,
        timestamp: chrono::Utc::now().to_rfc3339(),
        summary: ReviewSummary {
            files_changed,
            lines_added,
            lines_removed,
            commits: 1, // Simplified, would count actual commits in production
            overall_risk,
            approval_status: ApprovalStatus::Approved, // Determined later
        },
        security_issues,
        performance_issues,
        quality_issues,
        recommendations,
    })
}

fn scan_security_issues(file: &str, content: &str) -> Vec<SecurityIssue> {
    let mut issues = Vec::new();

    for (i, line) in content.lines().enumerate() {
        // OWASP Top 10 checks

        // 1. Injection (SQL, Command, etc.)
        if line.contains("format!")
            && (line.contains("SELECT") || line.contains("INSERT") || line.contains("UPDATE"))
        {
            issues.push(SecurityIssue {
                file: file.to_string(),
                line: i + 1,
                severity: "high".to_string(),
                category: SecurityCategory::Injection,
                description: "Potential SQL injection vulnerability".to_string(),
                recommendation: "Use parameterized queries or ORM".to_string(),
            });
        }

        if line.contains("Command::new") && line.contains("format!") {
            issues.push(SecurityIssue {
                file: file.to_string(),
                line: i + 1,
                severity: "high".to_string(),
                category: SecurityCategory::Injection,
                description: "Potential command injection".to_string(),
                recommendation: "Sanitize input and avoid shell execution with user input"
                    .to_string(),
            });
        }

        // 2. Authentication
        if line.contains("password") && !line.contains("hash") && !line.contains("bcrypt") {
            issues.push(SecurityIssue {
                file: file.to_string(),
                line: i + 1,
                severity: "medium".to_string(),
                category: SecurityCategory::Authentication,
                description: "Password handling without hashing".to_string(),
                recommendation: "Use bcrypt or argon2 for password hashing".to_string(),
            });
        }

        // 3. Cryptographic failures
        if line.contains("MD5") || line.contains("SHA1") {
            issues.push(SecurityIssue {
                file: file.to_string(),
                line: i + 1,
                severity: "high".to_string(),
                category: SecurityCategory::CryptographicFailure,
                description: "Weak cryptographic algorithm".to_string(),
                recommendation: "Use SHA-256 or stronger algorithms".to_string(),
            });
        }

        // 4. Insecure deserialization
        if line.contains("serde_json::from_str") && !line.contains("validate") {
            issues.push(SecurityIssue {
                file: file.to_string(),
                line: i + 1,
                severity: "medium".to_string(),
                category: SecurityCategory::InsecureDeserialization,
                description: "Deserialization without validation".to_string(),
                recommendation: "Validate deserialized data before use".to_string(),
            });
        }

        // 5. XSS (for web contexts)
        if line.contains("innerHTML") || (line.contains("html") && line.contains("&")) {
            issues.push(SecurityIssue {
                file: file.to_string(),
                line: i + 1,
                severity: "high".to_string(),
                category: SecurityCategory::XSS,
                description: "Potential XSS vulnerability".to_string(),
                recommendation: "Sanitize HTML output and use proper escaping".to_string(),
            });
        }

        // 6. Secret exposure
        if (line.contains("API_KEY") || line.contains("SECRET") || line.contains("PASSWORD"))
            && !line.contains("env::var")
            && !line.contains("get_env")
        {
            issues.push(SecurityIssue {
                file: file.to_string(),
                line: i + 1,
                severity: "critical".to_string(),
                category: SecurityCategory::SecretExposure,
                description: "Hardcoded secret detected".to_string(),
                recommendation: "Use environment variables or secure vault".to_string(),
            });
        }

        // 7. Logging sensitive data
        if (line.contains("log") || line.contains("println!"))
            && (line.contains("password") || line.contains("token") || line.contains("secret"))
        {
            issues.push(SecurityIssue {
                file: file.to_string(),
                line: i + 1,
                severity: "high".to_string(),
                category: SecurityCategory::Logging,
                description: "Logging sensitive information".to_string(),
                recommendation: "Remove sensitive data from logs".to_string(),
            });
        }

        // 8. Unwrap/expect without error handling
        if line.contains("unwrap()") || line.contains("expect(") {
            issues.push(SecurityIssue {
                file: file.to_string(),
                line: i + 1,
                severity: "low".to_string(),
                category: SecurityCategory::Other,
                description: "Panic-inducing code may cause DoS".to_string(),
                recommendation: "Use proper error handling with Result".to_string(),
            });
        }
    }

    issues
}

fn scan_performance_issues(file: &str, content: &str) -> Vec<PerformanceIssue> {
    let mut issues = Vec::new();

    for (i, line) in content.lines().enumerate() {
        // Clone detection
        if line.matches(".clone()").count() > 1 {
            issues.push(PerformanceIssue {
                file: file.to_string(),
                line: i + 1,
                severity: "medium".to_string(),
                issue_type: "Excessive cloning".to_string(),
                description: "Multiple clone operations in single line".to_string(),
                suggestion: "Use references or Arc for shared data".to_string(),
            });
        }

        // String allocations
        if line.contains("String::from") && line.contains("loop") {
            issues.push(PerformanceIssue {
                file: file.to_string(),
                line: i + 1,
                severity: "high".to_string(),
                issue_type: "Allocation in loop".to_string(),
                description: "String allocation inside loop".to_string(),
                suggestion: "Pre-allocate or use string builder pattern".to_string(),
            });
        }

        // Synchronous I/O in async context
        if line.contains("async fn") {
            let next_lines = content
                .lines()
                .skip(i)
                .take(10)
                .collect::<Vec<_>>()
                .join("\n");
            if next_lines.contains("std::fs::read") {
                issues.push(PerformanceIssue {
                    file: file.to_string(),
                    line: i + 1,
                    severity: "high".to_string(),
                    issue_type: "Blocking I/O in async".to_string(),
                    description: "Synchronous I/O in async function".to_string(),
                    suggestion: "Use tokio::fs for async I/O operations".to_string(),
                });
            }
        }

        // Unoptimized loops
        if line.contains("for")
            && line.contains("..")
            && line.contains("len()")
            && line.contains("vec[")
        {
            issues.push(PerformanceIssue {
                file: file.to_string(),
                line: i + 1,
                severity: "low".to_string(),
                issue_type: "Iterator optimization".to_string(),
                description: "Index-based iteration".to_string(),
                suggestion: "Use iterator methods for better performance".to_string(),
            });
        }

        // Large capacity allocations
        if line.contains("Vec::with_capacity") {
            if let Some(cap_str) = line.split('(').nth(1) {
                if let Ok(cap) = cap_str
                    .split(')')
                    .next()
                    .unwrap_or("0")
                    .trim()
                    .parse::<usize>()
                {
                    if cap > 100000 {
                        issues.push(PerformanceIssue {
                            file: file.to_string(),
                            line: i + 1,
                            severity: "medium".to_string(),
                            issue_type: "Large allocation".to_string(),
                            description: format!("Large capacity allocation: {}", cap),
                            suggestion: "Consider streaming or chunking for large datasets"
                                .to_string(),
                        });
                    }
                }
            }
        }

        // Recursive without tail call optimization
        if line.contains("fn ") {
            let fn_name = extract_function_name(line);
            if content.contains(&format!("{}(", fn_name))
                && content
                    .lines()
                    .skip(i)
                    .take(20)
                    .any(|l| l.contains(&fn_name))
                && !line.contains("tail")
            {
                issues.push(PerformanceIssue {
                    file: file.to_string(),
                    line: i + 1,
                    severity: "low".to_string(),
                    issue_type: "Recursion".to_string(),
                    description: "Recursive function without tail call optimization".to_string(),
                    suggestion: "Consider iterative approach or ensure tail recursion".to_string(),
                });
            }
        }
    }

    issues
}

fn scan_quality_issues(file: &str, content: &str) -> Vec<QualityIssue> {
    let mut issues = Vec::new();

    for (i, line) in content.lines().enumerate() {
        // Long lines
        if line.len() > 120 {
            issues.push(QualityIssue {
                file: file.to_string(),
                line: i + 1,
                severity: "low".to_string(),
                issue_type: "Code style".to_string(),
                description: format!("Line too long: {} characters", line.len()),
            });
        }

        // TODO/FIXME comments
        if line.contains("TODO") || line.contains("FIXME") {
            issues.push(QualityIssue {
                file: file.to_string(),
                line: i + 1,
                severity: "low".to_string(),
                issue_type: "Technical debt".to_string(),
                description: "Unresolved TODO/FIXME comment".to_string(),
            });
        }

        // Magic numbers
        if line.contains("== ") || line.contains("!= ") {
            for word in line.split_whitespace() {
                if let Ok(num) = word.trim_matches(|c: char| !c.is_numeric()).parse::<i32>() {
                    if num > 10 && num != 100 && num != 1000 {
                        issues.push(QualityIssue {
                            file: file.to_string(),
                            line: i + 1,
                            severity: "low".to_string(),
                            issue_type: "Magic number".to_string(),
                            description: format!("Magic number {} should be a named constant", num),
                        });
                        break;
                    }
                }
            }
        }

        // Missing error handling
        if line.contains("?") && !line.contains("Result") && line.contains("fn ") {
            issues.push(QualityIssue {
                file: file.to_string(),
                line: i + 1,
                severity: "medium".to_string(),
                issue_type: "Error handling".to_string(),
                description: "Function using ? operator should return Result".to_string(),
            });
        }
    }

    issues
}

fn calculate_risk(
    security: &[SecurityIssue],
    performance: &[PerformanceIssue],
    quality: &[QualityIssue],
) -> RiskLevel {
    let critical_security = security.iter().filter(|i| i.severity == "critical").count();
    let high_security = security.iter().filter(|i| i.severity == "high").count();
    let high_performance = performance.iter().filter(|i| i.severity == "high").count();

    if critical_security > 0 {
        RiskLevel::Critical
    } else if high_security > 2 || high_performance > 5 {
        RiskLevel::High
    } else if high_security > 0 || high_performance > 2 || quality.len() > 20 {
        RiskLevel::Medium
    } else {
        RiskLevel::Low
    }
}

fn generate_recommendations(
    security: &[SecurityIssue],
    performance: &[PerformanceIssue],
    quality: &[QualityIssue],
) -> Vec<String> {
    let mut recommendations = Vec::new();

    // Security recommendations
    if !security.is_empty() {
        recommendations.push(format!(
            "Address {} security issues before merging",
            security.len()
        ));

        let categories: HashMap<String, usize> =
            security.iter().fold(HashMap::new(), |mut acc, issue| {
                *acc.entry(format!("{:?}", issue.category)).or_insert(0) += 1;
                acc
            });

        for (category, count) in categories {
            recommendations.push(format!("  - {} {} issues", count, category));
        }
    }

    // Performance recommendations
    if !performance.is_empty() {
        let high_perf = performance.iter().filter(|i| i.severity == "high").count();
        if high_perf > 0 {
            recommendations.push(format!(
                "Fix {} high-severity performance issues",
                high_perf
            ));
        }

        recommendations.push("Consider profiling with criterion for benchmarking".to_string());
    }

    // Quality recommendations
    if quality.len() > 10 {
        recommendations.push(format!("Address {} quality issues", quality.len()));
        recommendations.push("Run cargo fmt and cargo clippy".to_string());
    }

    // General recommendations
    recommendations.push("Add tests for new functionality".to_string());
    recommendations.push("Update documentation if API changes".to_string());

    recommendations
}

fn determine_approval(review: &ReviewResult) -> ApprovalStatus {
    let critical_security = review
        .security_issues
        .iter()
        .filter(|i| i.severity == "critical")
        .count();
    let high_security = review
        .security_issues
        .iter()
        .filter(|i| i.severity == "high")
        .count();

    if critical_security > 0 {
        ApprovalStatus::Blocked
    } else if high_security > 2 {
        ApprovalStatus::ChangesRequested
    } else if !review.security_issues.is_empty() || !review.performance_issues.is_empty() {
        ApprovalStatus::ApprovedWithComments
    } else {
        ApprovalStatus::Approved
    }
}

fn display_review(review: &ReviewResult, detailed: bool, output: &OutputHandler) {
    output.info(&format!("\n=== MR/PR #{} Review ===\n", review.mr_number));

    // Summary
    output.info("--- Summary ---");
    output.info(&format!("Files changed: {}", review.summary.files_changed));
    output.info(&format!("Lines added: +{}", review.summary.lines_added));
    output.info(&format!("Lines removed: -{}", review.summary.lines_removed));
    output.info(&format!("Overall risk: {:?}", review.summary.overall_risk));

    // Security issues
    if !review.security_issues.is_empty() {
        output.warning(&format!(
            "\n--- Security Issues ({}) ---",
            review.security_issues.len()
        ));
        for issue in &review.security_issues {
            output.list_item(&format!(
                "[{}] {:?} at {}:{} - {}",
                issue.severity, issue.category, issue.file, issue.line, issue.description
            ));
            if detailed {
                output.info(&format!("    💡 {}", issue.recommendation));
            }
        }
    }

    // Performance issues
    if !review.performance_issues.is_empty() {
        output.warning(&format!(
            "\n--- Performance Issues ({}) ---",
            review.performance_issues.len()
        ));
        for issue in &review.performance_issues {
            output.list_item(&format!(
                "[{}] {} at {}:{} - {}",
                issue.severity, issue.issue_type, issue.file, issue.line, issue.description
            ));
            if detailed {
                output.info(&format!("    💡 {}", issue.suggestion));
            }
        }
    }

    // Quality issues
    if !review.quality_issues.is_empty() && detailed {
        output.info(&format!(
            "\n--- Quality Issues ({}) ---",
            review.quality_issues.len()
        ));
        for issue in review.quality_issues.iter().take(10) {
            output.list_item(&format!(
                "[{}] {} at {}:{} - {}",
                issue.severity, issue.issue_type, issue.file, issue.line, issue.description
            ));
        }
        if review.quality_issues.len() > 10 {
            output.info(&format!(
                "    ... and {} more",
                review.quality_issues.len() - 10
            ));
        }
    }

    // Recommendations
    if !review.recommendations.is_empty() {
        output.info("\n--- Recommendations ---");
        for rec in &review.recommendations {
            output.list_item(rec);
        }
    }
}

fn display_approval(status: ApprovalStatus, output: &OutputHandler) {
    output.info("\n--- Review Decision ---");

    match status {
        ApprovalStatus::Approved => {
            output.success("✅ APPROVED - No blocking issues found");
        }
        ApprovalStatus::ApprovedWithComments => {
            output.warning("⚠️  APPROVED WITH COMMENTS - Please address comments before merge");
        }
        ApprovalStatus::ChangesRequested => {
            output.error("❌ CHANGES REQUESTED - Critical issues must be fixed");
        }
        ApprovalStatus::Blocked => {
            output.error("🚫 BLOCKED - Critical security issues detected, do not merge");
        }
    }
}

fn extract_function_name(line: &str) -> String {
    if let Some(start) = line.find("fn ") {
        let after_fn = &line[start + 3..];
        if let Some(end) = after_fn.find(['(', '<']) {
            return after_fn[..end].trim().to_string();
        }
    }
    String::new()
}
