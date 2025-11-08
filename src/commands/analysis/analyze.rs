use crate::cli::args::{AnalysisFormat, AnalysisTarget};
use crate::cli::output::OutputHandler;
use crate::core::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Main analysis result structure
#[derive(Debug, Serialize, Deserialize)]
pub struct AnalysisResult {
    pub target: String,
    pub timestamp: String,
    pub summary: AnalysisSummary,
    pub details: AnalysisDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalysisSummary {
    pub total_files: usize,
    pub total_lines: usize,
    pub languages: HashMap<String, usize>,
    pub issues_found: usize,
    pub overall_score: f32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AnalysisDetails {
    Structure(StructureAnalysis),
    Performance(PerformanceAnalysis),
    Quality(QualityAnalysis),
    Debt(DebtAnalysis),
    Overview(OverviewAnalysis),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StructureAnalysis {
    pub modules: Vec<ModuleInfo>,
    pub dependencies: Vec<DependencyInfo>,
    pub depth: usize,
    pub circular_dependencies: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModuleInfo {
    pub name: String,
    pub path: String,
    pub lines: usize,
    pub exports: usize,
    pub imports: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DependencyInfo {
    pub from: String,
    pub to: String,
    pub kind: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PerformanceAnalysis {
    pub hot_spots: Vec<HotSpot>,
    pub memory_usage: MemoryMetrics,
    pub optimization_suggestions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HotSpot {
    pub file: String,
    pub line: usize,
    pub issue: String,
    pub severity: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryMetrics {
    pub estimated_heap: usize,
    pub stack_frames: usize,
    pub large_allocations: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualityAnalysis {
    pub complexity_metrics: ComplexityMetrics,
    pub code_smells: Vec<CodeSmell>,
    pub test_coverage: f32,
    pub duplication: DuplicationMetrics,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComplexityMetrics {
    pub cyclomatic_avg: f32,
    pub cyclomatic_max: usize,
    pub cognitive_avg: f32,
    pub maintainability_index: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeSmell {
    pub file: String,
    pub line: usize,
    pub smell_type: String,
    pub description: String,
    pub severity: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DuplicationMetrics {
    pub duplicated_lines: usize,
    pub duplication_percentage: f32,
    pub duplicated_blocks: Vec<DuplicatedBlock>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DuplicatedBlock {
    pub file1: String,
    pub file2: String,
    pub lines: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DebtAnalysis {
    pub total_debt_hours: f32,
    pub debt_items: Vec<DebtItem>,
    pub debt_by_category: HashMap<String, f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DebtItem {
    pub file: String,
    pub category: String,
    pub description: String,
    pub effort_hours: f32,
    pub priority: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OverviewAnalysis {
    pub structure: StructureSummary,
    pub performance: PerformanceSummary,
    pub quality: QualitySummary,
    pub debt: DebtSummary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StructureSummary {
    pub modules_count: usize,
    pub max_depth: usize,
    pub circular_deps: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PerformanceSummary {
    pub hot_spots_count: usize,
    pub critical_issues: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QualitySummary {
    pub avg_complexity: f32,
    pub code_smells_count: usize,
    pub test_coverage: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DebtSummary {
    pub total_hours: f32,
    pub high_priority_items: usize,
}

/// Analyze project based on specified target
pub fn analyze_project(
    target: AnalysisTarget,
    format: AnalysisFormat,
    detailed: bool,
    output: &OutputHandler,
) -> Result<()> {
    output.info(&format!("Starting {:?} analysis...", target));

    let current_dir = std::env::current_dir()?;

    let result = match target {
        AnalysisTarget::Structure => analyze_structure(&current_dir, detailed, output)?,
        AnalysisTarget::Performance => analyze_performance(&current_dir, detailed, output)?,
        AnalysisTarget::Quality => analyze_quality(&current_dir, detailed, output)?,
        AnalysisTarget::Debt => analyze_debt(&current_dir, detailed, output)?,
        AnalysisTarget::Overview => analyze_overview(&current_dir, detailed, output)?,
    };

    // Format and output results
    match format {
        AnalysisFormat::Text => output_text(&result, output),
        AnalysisFormat::Json => output_json(&result, output)?,
        AnalysisFormat::Html => output_html(&result, output)?,
    }

    output.success(&format!(
        "Analysis complete. Overall score: {:.1}/10",
        result.summary.overall_score
    ));

    Ok(())
}

fn analyze_structure(
    path: &Path,
    _detailed: bool,
    output: &OutputHandler,
) -> Result<AnalysisResult> {
    output.debug("Analyzing project structure...");

    let modules = scan_modules(path)?;
    let dependencies = analyze_dependencies(&modules)?;
    let circular_deps = detect_circular_dependencies(&dependencies);

    let summary = AnalysisSummary {
        total_files: modules.len(),
        total_lines: modules.iter().map(|m| m.lines).sum(),
        languages: count_languages(&modules),
        issues_found: circular_deps.len(),
        overall_score: calculate_structure_score(&modules, &circular_deps),
    };

    let structure = StructureAnalysis {
        depth: calculate_max_depth(path),
        modules,
        dependencies,
        circular_dependencies: circular_deps,
    };

    Ok(AnalysisResult {
        target: "Structure".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        summary,
        details: AnalysisDetails::Structure(structure),
    })
}

fn analyze_performance(
    path: &Path,
    _detailed: bool,
    output: &OutputHandler,
) -> Result<AnalysisResult> {
    output.debug("Analyzing performance characteristics...");

    let hot_spots = detect_performance_hotspots(path)?;
    let memory_metrics = analyze_memory_usage(path)?;
    let suggestions = generate_optimization_suggestions(&hot_spots, &memory_metrics);

    let summary = AnalysisSummary {
        total_files: count_source_files(path),
        total_lines: count_total_lines(path)?,
        languages: detect_languages(path)?,
        issues_found: hot_spots.len(),
        overall_score: calculate_performance_score(&hot_spots),
    };

    let performance = PerformanceAnalysis {
        hot_spots,
        memory_usage: memory_metrics,
        optimization_suggestions: suggestions,
    };

    Ok(AnalysisResult {
        target: "Performance".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        summary,
        details: AnalysisDetails::Performance(performance),
    })
}

fn analyze_quality(path: &Path, _detailed: bool, output: &OutputHandler) -> Result<AnalysisResult> {
    output.debug("Analyzing code quality...");

    let complexity = calculate_complexity_metrics(path)?;
    let smells = detect_code_smells(path)?;
    let coverage = estimate_test_coverage(path)?;
    let duplication = analyze_duplication(path)?;

    let summary = AnalysisSummary {
        total_files: count_source_files(path),
        total_lines: count_total_lines(path)?,
        languages: detect_languages(path)?,
        issues_found: smells.len(),
        overall_score: calculate_quality_score(&complexity, &smells, coverage),
    };

    let quality = QualityAnalysis {
        complexity_metrics: complexity,
        code_smells: smells,
        test_coverage: coverage,
        duplication,
    };

    Ok(AnalysisResult {
        target: "Quality".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        summary,
        details: AnalysisDetails::Quality(quality),
    })
}

fn analyze_debt(path: &Path, _detailed: bool, output: &OutputHandler) -> Result<AnalysisResult> {
    output.debug("Analyzing technical debt...");

    let debt_items = scan_technical_debt(path)?;
    let debt_by_category = categorize_debt(&debt_items);
    let total_hours: f32 = debt_items.iter().map(|d| d.effort_hours).sum();

    let summary = AnalysisSummary {
        total_files: count_source_files(path),
        total_lines: count_total_lines(path)?,
        languages: detect_languages(path)?,
        issues_found: debt_items.len(),
        overall_score: calculate_debt_score(total_hours, debt_items.len()),
    };

    let debt = DebtAnalysis {
        total_debt_hours: total_hours,
        debt_items,
        debt_by_category,
    };

    Ok(AnalysisResult {
        target: "Debt".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        summary,
        details: AnalysisDetails::Debt(debt),
    })
}

fn analyze_overview(
    path: &Path,
    _detailed: bool,
    output: &OutputHandler,
) -> Result<AnalysisResult> {
    output.debug("Generating project overview...");

    // Collect high-level metrics from each analysis type
    let structure_summary = get_structure_summary(path)?;
    let performance_summary = get_performance_summary(path)?;
    let quality_summary = get_quality_summary(path)?;
    let debt_summary = get_debt_summary(path)?;

    let summary = AnalysisSummary {
        total_files: count_source_files(path),
        total_lines: count_total_lines(path)?,
        languages: detect_languages(path)?,
        issues_found: structure_summary.circular_deps
            + performance_summary.hot_spots_count
            + quality_summary.code_smells_count,
        overall_score: calculate_overall_score(
            &structure_summary,
            &performance_summary,
            &quality_summary,
            &debt_summary,
        ),
    };

    let overview = OverviewAnalysis {
        structure: structure_summary,
        performance: performance_summary,
        quality: quality_summary,
        debt: debt_summary,
    };

    Ok(AnalysisResult {
        target: "Overview".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        summary,
        details: AnalysisDetails::Overview(overview),
    })
}

// Helper functions for module scanning
fn scan_modules(path: &Path) -> Result<Vec<ModuleInfo>> {
    let mut modules = Vec::new();

    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && is_source_file(&path) {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    modules.push(ModuleInfo {
                        name: path.file_stem().unwrap().to_string_lossy().to_string(),
                        path: path.to_string_lossy().to_string(),
                        lines: content.lines().count(),
                        exports: count_exports(&content),
                        imports: count_imports(&content),
                    });
                }
            } else if path.is_dir() && !is_ignored_dir(&path) {
                modules.extend(scan_modules(&path)?);
            }
        }
    }

    Ok(modules)
}

fn analyze_dependencies(modules: &[ModuleInfo]) -> Result<Vec<DependencyInfo>> {
    let mut deps = Vec::new();

    for module in modules {
        // Simple dependency detection based on imports
        // In a real implementation, this would use a proper AST parser
        if let Ok(content) = std::fs::read_to_string(&module.path) {
            for line in content.lines() {
                if line.contains("use ") || line.contains("import ") || line.contains("require(") {
                    // Extract dependency information
                    deps.push(DependencyInfo {
                        from: module.name.clone(),
                        to: extract_import_name(line),
                        kind: "module".to_string(),
                    });
                }
            }
        }
    }

    Ok(deps)
}

fn detect_circular_dependencies(deps: &[DependencyInfo]) -> Vec<String> {
    // Simple cycle detection - in production, use a proper graph algorithm
    let mut circular = Vec::new();
    let mut seen = HashMap::new();

    for dep in deps {
        let key = format!("{}->{}", dep.from, dep.to);
        let reverse = format!("{}->{}", dep.to, dep.from);

        if seen.contains_key(&reverse) {
            circular.push(format!("{} <-> {}", dep.from, dep.to));
        }
        seen.insert(key, true);
    }

    circular
}

fn detect_performance_hotspots(path: &Path) -> Result<Vec<HotSpot>> {
    let mut hotspots = Vec::new();

    // Scan for common performance anti-patterns
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            let file_path = entry.path();
            if file_path.is_file() && is_source_file(&file_path) {
                if let Ok(content) = std::fs::read_to_string(&file_path) {
                    for (i, line) in content.lines().enumerate() {
                        // Detect performance issues
                        if line.contains("clone()") && line.contains(".clone().clone()") {
                            hotspots.push(HotSpot {
                                file: file_path.to_string_lossy().to_string(),
                                line: i + 1,
                                issue: "Multiple unnecessary clones".to_string(),
                                severity: "high".to_string(),
                            });
                        }
                        if line.contains("unwrap()")
                            && (line.contains("iter") || line.contains("loop"))
                        {
                            hotspots.push(HotSpot {
                                file: file_path.to_string_lossy().to_string(),
                                line: i + 1,
                                issue: "Unwrap in loop may cause panic".to_string(),
                                severity: "medium".to_string(),
                            });
                        }
                    }
                }
            } else if file_path.is_dir() && !is_ignored_dir(&file_path) {
                hotspots.extend(detect_performance_hotspots(&file_path)?);
            }
        }
    }

    Ok(hotspots)
}

fn analyze_memory_usage(path: &Path) -> Result<MemoryMetrics> {
    let mut large_allocations = Vec::new();

    // Estimate based on data structures
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            let file_path = entry.path();
            if file_path.is_file() && is_source_file(&file_path) {
                if let Ok(content) = std::fs::read_to_string(&file_path) {
                    for line in content.lines() {
                        if line.contains("Vec::with_capacity") {
                            if let Some(cap) = extract_capacity(line) {
                                if cap > 10000 {
                                    large_allocations.push(format!(
                                        "{}: capacity {}",
                                        file_path.to_string_lossy(),
                                        cap
                                    ));
                                }
                            }
                        }
                    }
                }
            } else if file_path.is_dir() && !is_ignored_dir(&file_path) {
                large_allocations.extend(analyze_memory_usage(&file_path)?.large_allocations);
            }
        }
    }

    Ok(MemoryMetrics {
        estimated_heap: large_allocations.len() * 1024, // Rough estimate
        stack_frames: 0,                                // Would need runtime analysis
        large_allocations,
    })
}

fn generate_optimization_suggestions(hotspots: &[HotSpot], memory: &MemoryMetrics) -> Vec<String> {
    let mut suggestions = Vec::new();

    if hotspots.len() > 5 {
        suggestions
            .push("Consider profiling with criterion to identify actual bottlenecks".to_string());
    }

    if memory.large_allocations.len() > 3 {
        suggestions.push(
            "Review large allocations and consider using smaller buffers or streaming".to_string(),
        );
    }

    suggestions.push("Use cargo-flamegraph for detailed performance profiling".to_string());
    suggestions.push("Consider using Arc for shared data instead of cloning".to_string());

    suggestions
}

fn calculate_complexity_metrics(path: &Path) -> Result<ComplexityMetrics> {
    let mut total_complexity = 0.0;
    let mut max_complexity = 0;
    let mut function_count = 0;

    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            let file_path = entry.path();
            if file_path.is_file() && is_source_file(&file_path) {
                if let Ok(content) = std::fs::read_to_string(&file_path) {
                    for line in content.lines() {
                        if line.contains("fn ") {
                            function_count += 1;
                            // Simple complexity estimate based on control flow
                            let complexity = count_control_flow_keywords(line);
                            total_complexity += complexity as f32;
                            max_complexity = max_complexity.max(complexity);
                        }
                    }
                }
            } else if file_path.is_dir() && !is_ignored_dir(&file_path) {
                let metrics = calculate_complexity_metrics(&file_path)?;
                total_complexity += metrics.cyclomatic_avg * function_count as f32;
                max_complexity = max_complexity.max(metrics.cyclomatic_max);
            }
        }
    }

    let avg_complexity = if function_count > 0 {
        total_complexity / function_count as f32
    } else {
        0.0
    };

    Ok(ComplexityMetrics {
        cyclomatic_avg: avg_complexity,
        cyclomatic_max: max_complexity,
        cognitive_avg: avg_complexity * 1.2, // Rough estimate
        maintainability_index: calculate_maintainability_index(avg_complexity),
    })
}

fn detect_code_smells(path: &Path) -> Result<Vec<CodeSmell>> {
    let mut smells = Vec::new();

    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            let file_path = entry.path();
            if file_path.is_file() && is_source_file(&file_path) {
                if let Ok(content) = std::fs::read_to_string(&file_path) {
                    for (i, line) in content.lines().enumerate() {
                        // Detect various code smells
                        if line.len() > 120 {
                            smells.push(CodeSmell {
                                file: file_path.to_string_lossy().to_string(),
                                line: i + 1,
                                smell_type: "Long line".to_string(),
                                description: format!(
                                    "Line length {} exceeds 120 characters",
                                    line.len()
                                ),
                                severity: "low".to_string(),
                            });
                        }
                        if line.contains("TODO") || line.contains("FIXME") {
                            smells.push(CodeSmell {
                                file: file_path.to_string_lossy().to_string(),
                                line: i + 1,
                                smell_type: "TODO comment".to_string(),
                                description: "Unresolved TODO/FIXME comment".to_string(),
                                severity: "medium".to_string(),
                            });
                        }
                        if line.contains("unwrap()") || line.contains("expect(") {
                            smells.push(CodeSmell {
                                file: file_path.to_string_lossy().to_string(),
                                line: i + 1,
                                smell_type: "Panic-inducing code".to_string(),
                                description: "Use of unwrap/expect may cause panic".to_string(),
                                severity: "high".to_string(),
                            });
                        }
                    }
                }
            } else if file_path.is_dir() && !is_ignored_dir(&file_path) {
                smells.extend(detect_code_smells(&file_path)?);
            }
        }
    }

    Ok(smells)
}

fn estimate_test_coverage(path: &Path) -> Result<f32> {
    let source_count = count_source_files(path);
    let test_count = count_test_files(path);

    if source_count == 0 {
        return Ok(0.0);
    }

    // Simple estimate: test files / source files * 100
    Ok((test_count as f32 / source_count as f32) * 100.0)
}

fn analyze_duplication(_path: &Path) -> Result<DuplicationMetrics> {
    // Simple duplication detection - in production, use a proper algorithm
    Ok(DuplicationMetrics {
        duplicated_lines: 0,
        duplication_percentage: 0.0,
        duplicated_blocks: Vec::new(),
    })
}

fn scan_technical_debt(path: &Path) -> Result<Vec<DebtItem>> {
    let mut debt_items = Vec::new();

    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            let file_path = entry.path();
            if file_path.is_file() && is_source_file(&file_path) {
                if let Ok(content) = std::fs::read_to_string(&file_path) {
                    for line in content.lines() {
                        if line.contains("TODO") {
                            debt_items.push(DebtItem {
                                file: file_path.to_string_lossy().to_string(),
                                category: "Documentation".to_string(),
                                description: "TODO comment found".to_string(),
                                effort_hours: 0.5,
                                priority: "low".to_string(),
                            });
                        }
                        if line.contains("FIXME") {
                            debt_items.push(DebtItem {
                                file: file_path.to_string_lossy().to_string(),
                                category: "Bug".to_string(),
                                description: "FIXME comment found".to_string(),
                                effort_hours: 2.0,
                                priority: "high".to_string(),
                            });
                        }
                        if line.contains("HACK") {
                            debt_items.push(DebtItem {
                                file: file_path.to_string_lossy().to_string(),
                                category: "Refactoring".to_string(),
                                description: "HACK comment found".to_string(),
                                effort_hours: 4.0,
                                priority: "medium".to_string(),
                            });
                        }
                    }
                }
            } else if file_path.is_dir() && !is_ignored_dir(&file_path) {
                debt_items.extend(scan_technical_debt(&file_path)?);
            }
        }
    }

    Ok(debt_items)
}

fn categorize_debt(items: &[DebtItem]) -> HashMap<String, f32> {
    let mut by_category = HashMap::new();

    for item in items {
        *by_category.entry(item.category.clone()).or_insert(0.0) += item.effort_hours;
    }

    by_category
}

// Summary generation functions
fn get_structure_summary(path: &Path) -> Result<StructureSummary> {
    let modules = scan_modules(path)?;
    let deps = analyze_dependencies(&modules)?;
    let circular = detect_circular_dependencies(&deps);

    Ok(StructureSummary {
        modules_count: modules.len(),
        max_depth: calculate_max_depth(path),
        circular_deps: circular.len(),
    })
}

fn get_performance_summary(path: &Path) -> Result<PerformanceSummary> {
    let hotspots = detect_performance_hotspots(path)?;
    let critical = hotspots.iter().filter(|h| h.severity == "high").count();

    Ok(PerformanceSummary {
        hot_spots_count: hotspots.len(),
        critical_issues: critical,
    })
}

fn get_quality_summary(path: &Path) -> Result<QualitySummary> {
    let complexity = calculate_complexity_metrics(path)?;
    let smells = detect_code_smells(path)?;
    let coverage = estimate_test_coverage(path)?;

    Ok(QualitySummary {
        avg_complexity: complexity.cyclomatic_avg,
        code_smells_count: smells.len(),
        test_coverage: coverage,
    })
}

fn get_debt_summary(path: &Path) -> Result<DebtSummary> {
    let items = scan_technical_debt(path)?;
    let high_priority = items.iter().filter(|i| i.priority == "high").count();
    let total: f32 = items.iter().map(|i| i.effort_hours).sum();

    Ok(DebtSummary {
        total_hours: total,
        high_priority_items: high_priority,
    })
}

// Utility functions
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

fn count_source_files(path: &Path) -> usize {
    let mut count = 0;

    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && is_source_file(&path) {
                count += 1;
            } else if path.is_dir() && !is_ignored_dir(&path) {
                count += count_source_files(&path);
            }
        }
    }

    count
}

fn count_test_files(path: &Path) -> usize {
    let mut count = 0;

    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                if let Some(name) = path.file_name() {
                    if name.to_string_lossy().contains("test")
                        || name.to_string_lossy().contains("spec")
                    {
                        count += 1;
                    }
                }
            } else if path.is_dir() && !is_ignored_dir(&path) {
                count += count_test_files(&path);
            }
        }
    }

    count
}

fn count_total_lines(path: &Path) -> Result<usize> {
    let mut total = 0;

    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && is_source_file(&path) {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    total += content.lines().count();
                }
            } else if path.is_dir() && !is_ignored_dir(&path) {
                total += count_total_lines(&path)?;
            }
        }
    }

    Ok(total)
}

fn detect_languages(path: &Path) -> Result<HashMap<String, usize>> {
    let mut langs = HashMap::new();

    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            let file_path = entry.path();
            if file_path.is_file() {
                if let Some(ext) = file_path.extension() {
                    let ext = ext.to_string_lossy().to_string();
                    *langs.entry(ext).or_insert(0) += 1;
                }
            } else if file_path.is_dir() && !is_ignored_dir(&file_path) {
                for (lang, count) in detect_languages(&file_path)? {
                    *langs.entry(lang).or_insert(0) += count;
                }
            }
        }
    }

    Ok(langs)
}

fn count_languages(modules: &[ModuleInfo]) -> HashMap<String, usize> {
    let mut langs = HashMap::new();

    for module in modules {
        if let Some(ext) = PathBuf::from(&module.path).extension() {
            let ext = ext.to_string_lossy().to_string();
            *langs.entry(ext).or_insert(0) += 1;
        }
    }

    langs
}

fn calculate_max_depth(path: &Path) -> usize {
    let mut max_depth = 0;

    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() && !is_ignored_dir(&path) {
                max_depth = max_depth.max(1 + calculate_max_depth(&path));
            }
        }
    }

    max_depth
}

fn count_exports(content: &str) -> usize {
    content
        .lines()
        .filter(|l| l.contains("pub ") || l.contains("export "))
        .count()
}

fn count_imports(content: &str) -> usize {
    content
        .lines()
        .filter(|l| l.contains("use ") || l.contains("import "))
        .count()
}

fn extract_import_name(line: &str) -> String {
    // Simple extraction - in production, use proper parsing
    line.split_whitespace()
        .nth(1)
        .unwrap_or("unknown")
        .to_string()
}

fn extract_capacity(line: &str) -> Option<usize> {
    // Extract capacity from Vec::with_capacity call
    line.split('(')
        .nth(1)?
        .split(')')
        .next()?
        .trim()
        .parse()
        .ok()
}

fn count_control_flow_keywords(line: &str) -> usize {
    let keywords = ["if", "else", "match", "loop", "while", "for"];
    keywords.iter().filter(|k| line.contains(*k)).count()
}

fn calculate_maintainability_index(avg_complexity: f32) -> f32 {
    // Simplified MI calculation: 171 - 5.2 * ln(V) - 0.23 * G - 16.2 * ln(LOC)
    // Higher is better (0-100 scale)
    if avg_complexity < 1.0 {
        return 100.0;
    }

    let base = 100.0 - (avg_complexity * 5.0);
    base.max(0.0).min(100.0)
}

// Score calculation functions
fn calculate_structure_score(_modules: &[ModuleInfo], circular: &[String]) -> f32 {
    let base_score = 10.0;
    let penalty = (circular.len() as f32) * 0.5;
    (base_score - penalty).max(0.0)
}

fn calculate_performance_score(hotspots: &[HotSpot]) -> f32 {
    let base_score = 10.0;
    let critical = hotspots.iter().filter(|h| h.severity == "high").count() as f32;
    let medium = hotspots.iter().filter(|h| h.severity == "medium").count() as f32;

    let penalty = critical * 1.0 + medium * 0.3;
    (base_score - penalty).max(0.0)
}

fn calculate_quality_score(
    metrics: &ComplexityMetrics,
    smells: &[CodeSmell],
    coverage: f32,
) -> f32 {
    let complexity_score = (15.0 - metrics.cyclomatic_avg).max(0.0).min(10.0);
    let smell_penalty = (smells.len() as f32 * 0.1).min(5.0);
    let coverage_bonus = (coverage / 10.0).min(2.0);

    (complexity_score - smell_penalty + coverage_bonus)
        .max(0.0)
        .min(10.0)
}

fn calculate_debt_score(total_hours: f32, item_count: usize) -> f32 {
    let base_score = 10.0;
    let hour_penalty = (total_hours / 10.0).min(5.0);
    let item_penalty = (item_count as f32 * 0.1).min(3.0);

    (base_score - hour_penalty - item_penalty).max(0.0)
}

fn calculate_overall_score(
    structure: &StructureSummary,
    performance: &PerformanceSummary,
    quality: &QualitySummary,
    debt: &DebtSummary,
) -> f32 {
    let structure_score = if structure.circular_deps > 0 {
        5.0
    } else {
        10.0
    };
    let performance_score = if performance.critical_issues > 5 {
        5.0
    } else {
        8.0
    };
    let quality_score = (15.0 - quality.avg_complexity).max(0.0).min(10.0);
    let debt_score = (10.0 - (debt.total_hours / 10.0)).max(0.0);

    (structure_score + performance_score + quality_score + debt_score) / 4.0
}

// Output formatters
fn output_text(result: &AnalysisResult, output: &OutputHandler) {
    output.info(&format!("\n=== {} Analysis Report ===", result.target));
    output.info(&format!("Timestamp: {}", result.timestamp));
    output.info("\n--- Summary ---");
    output.info(&format!("Total files: {}", result.summary.total_files));
    output.info(&format!("Total lines: {}", result.summary.total_lines));
    output.info(&format!("Languages: {:?}", result.summary.languages));
    output.info(&format!("Issues found: {}", result.summary.issues_found));
    output.info(&format!(
        "Overall score: {:.1}/10",
        result.summary.overall_score
    ));

    match &result.details {
        AnalysisDetails::Structure(s) => output_structure_text(s, output),
        AnalysisDetails::Performance(p) => output_performance_text(p, output),
        AnalysisDetails::Quality(q) => output_quality_text(q, output),
        AnalysisDetails::Debt(d) => output_debt_text(d, output),
        AnalysisDetails::Overview(o) => output_overview_text(o, output),
    }
}

fn output_structure_text(structure: &StructureAnalysis, output: &OutputHandler) {
    output.info("\n--- Structure Details ---");
    output.info(&format!("Modules: {}", structure.modules.len()));
    output.info(&format!("Max depth: {}", structure.depth));
    output.info(&format!("Dependencies: {}", structure.dependencies.len()));

    if !structure.circular_dependencies.is_empty() {
        output.warning(&format!(
            "Circular dependencies detected: {}",
            structure.circular_dependencies.len()
        ));
        for dep in &structure.circular_dependencies {
            output.list_item(&format!("  {}", dep));
        }
    }
}

fn output_performance_text(performance: &PerformanceAnalysis, output: &OutputHandler) {
    output.info("\n--- Performance Details ---");
    output.info(&format!("Hot spots found: {}", performance.hot_spots.len()));

    for hotspot in &performance.hot_spots {
        output.warning(&format!(
            "[{}] {}:{} - {}",
            hotspot.severity, hotspot.file, hotspot.line, hotspot.issue
        ));
    }

    output.info("\n--- Optimization Suggestions ---");
    for suggestion in &performance.optimization_suggestions {
        output.list_item(suggestion);
    }
}

fn output_quality_text(quality: &QualityAnalysis, output: &OutputHandler) {
    output.info("\n--- Quality Details ---");
    output.info(&format!(
        "Average complexity: {:.1}",
        quality.complexity_metrics.cyclomatic_avg
    ));
    output.info(&format!(
        "Max complexity: {}",
        quality.complexity_metrics.cyclomatic_max
    ));
    output.info(&format!(
        "Maintainability index: {:.1}",
        quality.complexity_metrics.maintainability_index
    ));
    output.info(&format!("Test coverage: {:.1}%", quality.test_coverage));
    output.info(&format!("Code smells: {}", quality.code_smells.len()));
}

fn output_debt_text(debt: &DebtAnalysis, output: &OutputHandler) {
    output.info("\n--- Technical Debt Details ---");
    output.info(&format!("Total debt: {:.1} hours", debt.total_debt_hours));
    output.info(&format!("Debt items: {}", debt.debt_items.len()));

    output.info("\n--- Debt by Category ---");
    for (category, hours) in &debt.debt_by_category {
        output.list_item(&format!("{}: {:.1} hours", category, hours));
    }
}

fn output_overview_text(overview: &OverviewAnalysis, output: &OutputHandler) {
    output.info("\n--- Overview ---");
    output.info(&format!(
        "Structure: {} modules, depth {}, {} circular deps",
        overview.structure.modules_count,
        overview.structure.max_depth,
        overview.structure.circular_deps
    ));
    output.info(&format!(
        "Performance: {} hot spots, {} critical",
        overview.performance.hot_spots_count, overview.performance.critical_issues
    ));
    output.info(&format!(
        "Quality: {:.1} avg complexity, {} smells, {:.1}% coverage",
        overview.quality.avg_complexity,
        overview.quality.code_smells_count,
        overview.quality.test_coverage
    ));
    output.info(&format!(
        "Debt: {:.1} hours, {} high priority items",
        overview.debt.total_hours, overview.debt.high_priority_items
    ));
}

fn output_json(result: &AnalysisResult, output: &OutputHandler) -> Result<()> {
    let json = serde_json::to_string_pretty(result)?;
    output.info(&json);
    Ok(())
}

fn output_html(result: &AnalysisResult, output: &OutputHandler) -> Result<()> {
    let html = format!(
        r#"
<!DOCTYPE html>
<html>
<head>
    <title>Analysis Report - {}</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 20px; }}
        h1 {{ color: #333; }}
        .summary {{ background: #f5f5f5; padding: 15px; border-radius: 5px; }}
        .score {{ font-size: 2em; color: #4CAF50; }}
    </style>
</head>
<body>
    <h1>{} Analysis Report</h1>
    <div class="summary">
        <p><strong>Timestamp:</strong> {}</p>
        <p><strong>Total Files:</strong> {}</p>
        <p><strong>Total Lines:</strong> {}</p>
        <p><strong>Issues Found:</strong> {}</p>
        <p class="score">Overall Score: {:.1}/10</p>
    </div>
</body>
</html>
"#,
        result.target,
        result.target,
        result.timestamp,
        result.summary.total_files,
        result.summary.total_lines,
        result.summary.issues_found,
        result.summary.overall_score
    );

    output.info(&html);
    Ok(())
}
