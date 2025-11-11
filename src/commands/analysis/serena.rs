use crate::cli::args::SerenaMode;
use crate::cli::output::OutputHandler;
use crate::core::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Serena semantic analysis result
#[derive(Debug, Serialize, Deserialize)]
pub struct SerenaResult {
    pub mode: String,
    pub timestamp: String,
    pub analysis: SemanticAnalysis,
    pub insights: Vec<Insight>,
    pub suggestions: Vec<Suggestion>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SemanticAnalysis {
    pub symbols: Vec<Symbol>,
    pub relationships: Vec<Relationship>,
    pub patterns: Vec<Pattern>,
    pub metrics: AnalysisMetrics,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub file: String,
    pub line: usize,
    pub scope: String,
    pub visibility: String,
    pub attributes: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SymbolKind {
    Function,
    Struct,
    Enum,
    Trait,
    Module,
    Constant,
    Variable,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Relationship {
    pub from: String,
    pub to: String,
    pub kind: RelationshipKind,
    pub strength: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RelationshipKind {
    Calls,
    Imports,
    Implements,
    Extends,
    Uses,
    Contains,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pattern {
    pub name: String,
    pub pattern_type: PatternType,
    pub locations: Vec<String>,
    pub confidence: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PatternType {
    DesignPattern,
    AntiPattern,
    Idiom,
    Architecture,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalysisMetrics {
    pub total_symbols: usize,
    pub total_relationships: usize,
    pub modularity_score: f32,
    pub coupling_score: f32,
    pub cohesion_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Insight {
    pub category: String,
    pub description: String,
    pub impact: String,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Suggestion {
    pub title: String,
    pub description: String,
    pub priority: String,
    pub effort: String,
    pub benefit: String,
}

/// Run Serena semantic code analysis
pub fn run_serena(mode: SerenaMode, targets: &[String], output: &OutputHandler) -> Result<()> {
    output.info(&output.t_format("serena-starting", "mode", &format!("{:?}", mode)));

    match mode {
        SerenaMode::Interactive => run_interactive_mode(targets, output),
        SerenaMode::Batch => run_batch_mode(targets, output),
        SerenaMode::Watch => run_watch_mode(targets, output),
    }
}

fn run_interactive_mode(targets: &[String], output: &OutputHandler) -> Result<()> {
    output.info(&output.t("serena-interactive-mode"));
    output.info(&format!("{}\n", output.t("serena-interactive-desc")));

    let current_dir = std::env::current_dir()?;
    let target_paths = if targets.is_empty() {
        vec![current_dir.to_string_lossy().to_string()]
    } else {
        targets.to_vec()
    };

    for target in &target_paths {
        let path = Path::new(target);
        let result = analyze_semantics(path, true)?;

        display_interactive_results(&result, output)?;
    }

    output.success(&format!("\n{}", output.t("serena-interactive-complete")));
    Ok(())
}

fn run_batch_mode(targets: &[String], output: &OutputHandler) -> Result<()> {
    output.info(&output.t("serena-batch-mode"));

    let current_dir = std::env::current_dir()?;
    let target_paths = if targets.is_empty() {
        vec![current_dir.to_string_lossy().to_string()]
    } else {
        targets.to_vec()
    };

    let mut all_results = Vec::new();

    for target in &target_paths {
        output.debug(&output.t_format("serena-analyzing", "target", target));
        let path = Path::new(target);
        let result = analyze_semantics(path, false)?;
        all_results.push(result);
    }

    // Generate batch report
    display_batch_results(&all_results, output)?;

    output.success(&format!(
        "\n{}",
        output.t_format(
            "serena-batch-complete",
            "count",
            &all_results.len().to_string()
        )
    ));
    Ok(())
}

fn run_watch_mode(targets: &[String], output: &OutputHandler) -> Result<()> {
    output.info(&output.t("serena-watch-mode"));
    output.warning(&output.t("serena-watch-simple-impl"));
    output.info(&format!("{}\n", output.t("serena-watch-simple-note")));

    let current_dir = std::env::current_dir()?;
    let target_paths = if targets.is_empty() {
        vec![current_dir.to_string_lossy().to_string()]
    } else {
        targets.to_vec()
    };

    // Initial analysis
    for target in &target_paths {
        output.info(&output.t_format("serena-watch-initial", "target", target));
        let path = Path::new(target);
        let result = analyze_semantics(path, false)?;
        display_summary(&result, output);
    }

    output.info(&format!("\n{}", output.t("serena-production-note")));
    output.info(&output.t("serena-production-stop"));

    Ok(())
}

fn analyze_semantics(path: &Path, _detailed: bool) -> Result<SerenaResult> {
    let mut symbols = Vec::new();
    let mut relationships = Vec::new();

    // Scan and extract symbols
    scan_symbols(path, &mut symbols)?;

    // Build relationship graph
    build_relationships(&symbols, &mut relationships);

    // Detect patterns
    let patterns = detect_patterns(&symbols, &relationships);

    // Calculate metrics
    let metrics = calculate_metrics(&symbols, &relationships);

    // Generate insights
    let insights = generate_insights(&symbols, &relationships, &patterns, &metrics);

    // Generate suggestions
    let suggestions = generate_suggestions(&insights, &metrics);

    Ok(SerenaResult {
        mode: "analysis".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        analysis: SemanticAnalysis {
            symbols,
            relationships,
            patterns,
            metrics,
        },
        insights,
        suggestions,
    })
}

fn scan_symbols(path: &Path, symbols: &mut Vec<Symbol>) -> Result<()> {
    if path.is_file() && is_source_file(path) {
        extract_symbols_from_file(path, symbols)?;
    } else if path.is_dir() && !is_ignored_dir(path) {
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                scan_symbols(&entry.path(), symbols)?;
            }
        }
    }

    Ok(())
}

fn extract_symbols_from_file(file_path: &Path, symbols: &mut Vec<Symbol>) -> Result<()> {
    // Skip files that are not valid UTF-8 (binaries, legacy encodings, etc.)
    let content = match std::fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(_) => return Ok(()), // Silently skip non-UTF-8 files
    };
    let file_str = file_path.to_string_lossy().to_string();

    for (i, line) in content.lines().enumerate() {
        let trimmed = line.trim();

        // Extract function symbols
        if trimmed.starts_with("pub fn ") || trimmed.starts_with("fn ") {
            if let Some(name) = extract_name_after_keyword(trimmed, "fn ") {
                symbols.push(Symbol {
                    name,
                    kind: SymbolKind::Function,
                    file: file_str.clone(),
                    line: i + 1,
                    scope: "module".to_string(),
                    visibility: if trimmed.starts_with("pub") {
                        "public"
                    } else {
                        "private"
                    }
                    .to_string(),
                    attributes: HashMap::new(),
                });
            }
        }

        // Extract struct symbols
        if trimmed.starts_with("pub struct ") || trimmed.starts_with("struct ") {
            if let Some(name) = extract_name_after_keyword(trimmed, "struct ") {
                symbols.push(Symbol {
                    name,
                    kind: SymbolKind::Struct,
                    file: file_str.clone(),
                    line: i + 1,
                    scope: "module".to_string(),
                    visibility: if trimmed.starts_with("pub") {
                        "public"
                    } else {
                        "private"
                    }
                    .to_string(),
                    attributes: HashMap::new(),
                });
            }
        }

        // Extract enum symbols
        if trimmed.starts_with("pub enum ") || trimmed.starts_with("enum ") {
            if let Some(name) = extract_name_after_keyword(trimmed, "enum ") {
                symbols.push(Symbol {
                    name,
                    kind: SymbolKind::Enum,
                    file: file_str.clone(),
                    line: i + 1,
                    scope: "module".to_string(),
                    visibility: if trimmed.starts_with("pub") {
                        "public"
                    } else {
                        "private"
                    }
                    .to_string(),
                    attributes: HashMap::new(),
                });
            }
        }

        // Extract trait symbols
        if trimmed.starts_with("pub trait ") || trimmed.starts_with("trait ") {
            if let Some(name) = extract_name_after_keyword(trimmed, "trait ") {
                symbols.push(Symbol {
                    name,
                    kind: SymbolKind::Trait,
                    file: file_str.clone(),
                    line: i + 1,
                    scope: "module".to_string(),
                    visibility: if trimmed.starts_with("pub") {
                        "public"
                    } else {
                        "private"
                    }
                    .to_string(),
                    attributes: HashMap::new(),
                });
            }
        }
    }

    Ok(())
}

fn build_relationships(symbols: &[Symbol], relationships: &mut Vec<Relationship>) {
    // Build simple relationship graph based on symbol usage
    for symbol in symbols {
        if let Ok(content) = std::fs::read_to_string(&symbol.file) {
            for other in symbols {
                if symbol.name != other.name && content.contains(&other.name) {
                    let kind = match symbol.kind {
                        SymbolKind::Function => RelationshipKind::Calls,
                        _ => RelationshipKind::Uses,
                    };

                    relationships.push(Relationship {
                        from: symbol.name.clone(),
                        to: other.name.clone(),
                        kind,
                        strength: 1.0,
                    });
                }
            }
        }
    }
}

fn detect_patterns(symbols: &[Symbol], _relationships: &[Relationship]) -> Vec<Pattern> {
    let mut patterns = Vec::new();

    // Detect builder pattern
    let builder_symbols: Vec<_> = symbols
        .iter()
        .filter(|s| s.name.ends_with("Builder"))
        .collect();

    if !builder_symbols.is_empty() {
        patterns.push(Pattern {
            name: "Builder Pattern".to_string(),
            pattern_type: PatternType::DesignPattern,
            locations: builder_symbols
                .iter()
                .map(|s| format!("{}:{}", s.file, s.line))
                .collect(),
            confidence: 0.8,
        });
    }

    // Detect factory pattern
    let factory_symbols: Vec<_> = symbols
        .iter()
        .filter(|s| s.name.contains("Factory") || s.name.starts_with("create_"))
        .collect();

    if !factory_symbols.is_empty() {
        patterns.push(Pattern {
            name: "Factory Pattern".to_string(),
            pattern_type: PatternType::DesignPattern,
            locations: factory_symbols
                .iter()
                .map(|s| format!("{}:{}", s.file, s.line))
                .collect(),
            confidence: 0.7,
        });
    }

    // Detect singleton (anti-pattern in Rust)
    let singleton_symbols: Vec<_> = symbols
        .iter()
        .filter(|s| s.name.contains("Singleton") || s.name == "instance")
        .collect();

    if !singleton_symbols.is_empty() {
        patterns.push(Pattern {
            name: "Singleton (consider alternatives)".to_string(),
            pattern_type: PatternType::AntiPattern,
            locations: singleton_symbols
                .iter()
                .map(|s| format!("{}:{}", s.file, s.line))
                .collect(),
            confidence: 0.6,
        });
    }

    patterns
}

fn calculate_metrics(symbols: &[Symbol], relationships: &[Relationship]) -> AnalysisMetrics {
    let total_symbols = symbols.len();
    let total_relationships = relationships.len();

    // Calculate modularity (ratio of internal to external relationships)
    let modularity_score = if total_symbols > 0 {
        (total_relationships as f32 / total_symbols as f32).min(1.0)
    } else {
        0.0
    };

    // Calculate coupling (average relationships per symbol)
    let coupling_score = if total_symbols > 0 {
        (total_relationships as f32 / total_symbols as f32 * 10.0).min(10.0)
    } else {
        0.0
    };

    // Calculate cohesion (simplified)
    let public_symbols = symbols.iter().filter(|s| s.visibility == "public").count();
    let cohesion_score = if total_symbols > 0 {
        (total_symbols - public_symbols) as f32 / total_symbols as f32 * 10.0
    } else {
        0.0
    };

    AnalysisMetrics {
        total_symbols,
        total_relationships,
        modularity_score,
        coupling_score,
        cohesion_score,
    }
}

fn generate_insights(
    symbols: &[Symbol],
    _relationships: &[Relationship],
    patterns: &[Pattern],
    metrics: &AnalysisMetrics,
) -> Vec<Insight> {
    let mut insights = Vec::new();

    // Modularity insight
    if metrics.modularity_score < 0.3 {
        insights.push(Insight {
            category: "Architecture".to_string(),
            description: "Low modularity detected - consider breaking down into smaller modules"
                .to_string(),
            impact: "Maintainability".to_string(),
            confidence: 0.8,
        });
    }

    // Coupling insight
    if metrics.coupling_score > 7.0 {
        insights.push(Insight {
            category: "Architecture".to_string(),
            description: "High coupling detected - reduce dependencies between modules".to_string(),
            impact: "Maintainability & Testability".to_string(),
            confidence: 0.9,
        });
    }

    // Pattern insights
    for pattern in patterns {
        if matches!(pattern.pattern_type, PatternType::AntiPattern) {
            insights.push(Insight {
                category: "Design Pattern".to_string(),
                description: format!("{} detected - consider refactoring", pattern.name),
                impact: "Code Quality".to_string(),
                confidence: pattern.confidence,
            });
        }
    }

    // Public API insight
    let public_count = symbols.iter().filter(|s| s.visibility == "public").count();
    if public_count as f32 / symbols.len() as f32 > 0.5 {
        insights.push(Insight {
            category: "API Design".to_string(),
            description: "Large public API surface - consider hiding internal details".to_string(),
            impact: "API Stability".to_string(),
            confidence: 0.7,
        });
    }

    insights
}

fn generate_suggestions(insights: &[Insight], metrics: &AnalysisMetrics) -> Vec<Suggestion> {
    let mut suggestions = Vec::new();

    for insight in insights {
        match insight.category.as_str() {
            "Architecture" => {
                suggestions.push(Suggestion {
                    title: "Refactor for modularity".to_string(),
                    description: insight.description.clone(),
                    priority: "high".to_string(),
                    effort: "medium".to_string(),
                    benefit: "Improved maintainability and testability".to_string(),
                });
            }
            "Design Pattern" => {
                suggestions.push(Suggestion {
                    title: "Review design patterns".to_string(),
                    description: insight.description.clone(),
                    priority: "medium".to_string(),
                    effort: "low".to_string(),
                    benefit: "Better code organization".to_string(),
                });
            }
            "API Design" => {
                suggestions.push(Suggestion {
                    title: "Reduce public API surface".to_string(),
                    description: insight.description.clone(),
                    priority: "medium".to_string(),
                    effort: "medium".to_string(),
                    benefit: "Better encapsulation and API stability".to_string(),
                });
            }
            _ => {}
        }
    }

    // Add general suggestions based on metrics
    if metrics.total_symbols > 100 {
        suggestions.push(Suggestion {
            title: "Consider code organization".to_string(),
            description: format!(
                "Project has {} symbols - consider splitting into multiple modules",
                metrics.total_symbols
            ),
            priority: "low".to_string(),
            effort: "high".to_string(),
            benefit: "Better code navigation and organization".to_string(),
        });
    }

    suggestions
}

fn display_interactive_results(result: &SerenaResult, output: &OutputHandler) -> Result<()> {
    output.info(&format!("\n{}\n", output.t("serena-results-header")));

    // Display metrics
    output.info(&output.t("serena-metrics-header"));
    output.info(&output.t_format(
        "serena-metrics-symbols",
        "count",
        &result.analysis.metrics.total_symbols.to_string(),
    ));
    output.info(&output.t_format(
        "serena-metrics-relationships",
        "count",
        &result.analysis.metrics.total_relationships.to_string(),
    ));
    output.info(&output.t_format(
        "serena-metrics-modularity",
        "score",
        &format!("{:.2}", result.analysis.metrics.modularity_score),
    ));
    output.info(&output.t_format(
        "serena-metrics-coupling",
        "score",
        &format!("{:.2}", result.analysis.metrics.coupling_score),
    ));
    output.info(&output.t_format(
        "serena-metrics-cohesion",
        "score",
        &format!("{:.2}", result.analysis.metrics.cohesion_score),
    ));

    // Display patterns
    if !result.analysis.patterns.is_empty() {
        output.info(&format!("\n{}", output.t("serena-patterns-header")));
        for pattern in &result.analysis.patterns {
            output.list_item(
                &output
                    .t("serena-patterns-item")
                    .replace("{name}", &pattern.name)
                    .replace("{type:?}", &format!("{:?}", pattern.pattern_type))
                    .replace(
                        "{confidence}",
                        &format!("{:.0}", pattern.confidence * 100.0),
                    ),
            );
        }
    }

    // Display insights
    if !result.insights.is_empty() {
        output.info(&format!("\n{}", output.t("serena-insights-header")));
        for insight in &result.insights {
            output.warning(
                &output
                    .t("serena-insights-item")
                    .replace("{category}", &insight.category)
                    .replace("{description}", &insight.description)
                    .replace(
                        "{confidence}",
                        &format!("{:.0}", insight.confidence * 100.0),
                    ),
            );
            output.info(&output.t_format("serena-insights-impact", "impact", &insight.impact));
        }
    }

    // Display suggestions
    if !result.suggestions.is_empty() {
        output.info(&format!("\n{}", output.t("serena-suggestions-header")));
        for suggestion in &result.suggestions {
            output.list_item(
                &output
                    .t("serena-suggestions-item")
                    .replace("{priority}", &suggestion.priority.to_uppercase())
                    .replace("{title}", &suggestion.title),
            );
            output.info(&output.t_format(
                "serena-suggestions-desc",
                "description",
                &suggestion.description,
            ));
            output.info(
                &output
                    .t("serena-suggestions-benefit")
                    .replace("{effort}", &suggestion.effort)
                    .replace("{benefit}", &suggestion.benefit),
            );
        }
    }

    Ok(())
}

fn display_batch_results(results: &[SerenaResult], output: &OutputHandler) -> Result<()> {
    output.info(&format!("\n{}\n", output.t("serena-report-header")));

    let total_symbols: usize = results
        .iter()
        .map(|r| r.analysis.metrics.total_symbols)
        .sum();
    let total_relationships: usize = results
        .iter()
        .map(|r| r.analysis.metrics.total_relationships)
        .sum();
    let avg_modularity: f32 = results
        .iter()
        .map(|r| r.analysis.metrics.modularity_score)
        .sum::<f32>()
        / results.len() as f32;

    output.info(&output.t("serena-report-overall-stats"));
    output.info(&output.t_format("serena-report-targets", "count", &results.len().to_string()));
    output.info(&output.t_format(
        "serena-report-total-symbols",
        "count",
        &total_symbols.to_string(),
    ));
    output.info(&output.t_format(
        "serena-report-total-relationships",
        "count",
        &total_relationships.to_string(),
    ));
    output.info(&output.t_format(
        "serena-report-avg-modularity",
        "score",
        &format!("{:.2}", avg_modularity),
    ));

    // Aggregate insights
    let mut all_insights = Vec::new();
    for result in results {
        all_insights.extend(result.insights.clone());
    }

    if !all_insights.is_empty() {
        output.info(&format!(
            "\n{}",
            output.t_format(
                "serena-report-aggregated-insights",
                "count",
                &all_insights.len().to_string()
            )
        ));
        for insight in all_insights.iter().take(10) {
            output.list_item(
                &output
                    .t("serena-report-insight-item")
                    .replace("{category}", &insight.category)
                    .replace("{description}", &insight.description),
            );
        }
    }

    Ok(())
}

fn display_summary(result: &SerenaResult, output: &OutputHandler) {
    output.info(
        &output
            .t("serena-report-symbols")
            .replace(
                "{symbols}",
                &result.analysis.metrics.total_symbols.to_string(),
            )
            .replace(
                "{relationships}",
                &result.analysis.metrics.total_relationships.to_string(),
            )
            .replace("{patterns}", &result.analysis.patterns.len().to_string()),
    );
}

fn extract_name_after_keyword(line: &str, keyword: &str) -> Option<String> {
    if let Some(start) = line.find(keyword) {
        let after = &line[start + keyword.len()..];
        let name = after.split(['(', '<', ' ', '{']).next()?.trim();

        if !name.is_empty() {
            return Some(name.to_string());
        }
    }
    None
}

fn is_source_file(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        matches!(
            ext.to_str(),
            Some("rs") | Some("js") | Some("ts") | Some("py") | Some("go")
        )
    } else {
        false
    }
}

fn is_ignored_dir(path: &Path) -> bool {
    if let Some(name) = path.file_name() {
        let name = name.to_string_lossy();
        matches!(
            name.as_ref(),
            "target" | "node_modules" | ".git" | "dist" | "build"
        )
    } else {
        false
    }
}
