use crate::cli::output::OutputHandler;
use crate::core::error::Result;
use std::collections::HashMap;
use std::path::Path;

/// Explanation result
#[derive(Debug)]
pub struct Explanation {
    pub target: String,
    pub kind: ExplanationKind,
    pub description: String,
    pub usage_examples: Vec<String>,
    pub related_items: Vec<String>,
    pub source_locations: Vec<SourceLocation>,
}

#[derive(Debug)]
pub enum ExplanationKind {
    Function,
    Type,
    Module,
    Trait,
    Macro,
    Concept,
}

#[derive(Debug)]
pub struct SourceLocation {
    pub file: String,
    pub line: usize,
    pub context: String,
}

/// Explain a target (function/component/concept)
pub fn explain_target(
    target: &str,
    examples: bool,
    detailed: bool,
    output: &OutputHandler,
) -> Result<()> {
    output.info(&format!("Analyzing '{}' in project...", target));

    let current_dir = std::env::current_dir()?;
    let explanation = find_and_explain(&current_dir, target, detailed)?;

    display_explanation(&explanation, examples, detailed, output);

    Ok(())
}

fn find_and_explain(path: &Path, target: &str, detailed: bool) -> Result<Explanation> {
    let locations = search_in_codebase(path, target)?;

    if locations.is_empty() {
        // Check if it's a concept
        if let Some(concept) = explain_concept(target) {
            return Ok(concept);
        }

        return Err(crate::core::CldevError::command(format!(
            "Target '{}' not found in codebase",
            target
        )));
    }

    // Determine kind based on context
    let kind = determine_kind(&locations);
    let description = generate_description(target, &kind, &locations, detailed);
    let usage_examples = if detailed {
        find_usage_examples(target, path)?
    } else {
        Vec::new()
    };
    let related_items = find_related_items(target, &locations)?;

    Ok(Explanation {
        target: target.to_string(),
        kind,
        description,
        usage_examples,
        related_items,
        source_locations: locations,
    })
}

fn search_in_codebase(path: &Path, target: &str) -> Result<Vec<SourceLocation>> {
    let mut locations = Vec::new();

    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            let entry_path = entry.path();

            if entry_path.is_file() && is_source_file(&entry_path) {
                if let Ok(content) = std::fs::read_to_string(&entry_path) {
                    for (i, line) in content.lines().enumerate() {
                        if line.contains(target) {
                            // Check if this is a definition or usage
                            if is_definition(line, target) {
                                locations.push(SourceLocation {
                                    file: entry_path.to_string_lossy().to_string(),
                                    line: i + 1,
                                    context: line.trim().to_string(),
                                });
                            }
                        }
                    }
                }
            } else if entry_path.is_dir() && !is_ignored_dir(&entry_path) {
                locations.extend(search_in_codebase(&entry_path, target)?);
            }
        }
    }

    Ok(locations)
}

fn is_definition(line: &str, target: &str) -> bool {
    let trimmed = line.trim();

    // Rust patterns
    if trimmed.starts_with("fn ") && trimmed.contains(target) {
        return true;
    }
    if trimmed.starts_with("struct ") && trimmed.contains(target) {
        return true;
    }
    if trimmed.starts_with("enum ") && trimmed.contains(target) {
        return true;
    }
    if trimmed.starts_with("trait ") && trimmed.contains(target) {
        return true;
    }
    if trimmed.starts_with("type ") && trimmed.contains(target) {
        return true;
    }
    if trimmed.starts_with("macro_rules! ") && trimmed.contains(target) {
        return true;
    }

    // TypeScript/JavaScript patterns
    if (trimmed.starts_with("function ")
        || trimmed.starts_with("const ")
        || trimmed.starts_with("let "))
        && trimmed.contains(target)
    {
        return true;
    }
    if (trimmed.starts_with("class ") || trimmed.starts_with("interface "))
        && trimmed.contains(target)
    {
        return true;
    }

    // Python patterns
    if trimmed.starts_with("def ") && trimmed.contains(target) {
        return true;
    }
    if trimmed.starts_with("class ") && trimmed.contains(target) {
        return true;
    }

    false
}

fn determine_kind(locations: &[SourceLocation]) -> ExplanationKind {
    for loc in locations {
        let line = &loc.context;

        if line.contains("fn ") || line.contains("function ") || line.contains("def ") {
            return ExplanationKind::Function;
        }
        if line.contains("struct ") || line.contains("class ") || line.contains("interface ") {
            return ExplanationKind::Type;
        }
        if line.contains("mod ") {
            return ExplanationKind::Module;
        }
        if line.contains("trait ") {
            return ExplanationKind::Trait;
        }
        if line.contains("macro_rules!") {
            return ExplanationKind::Macro;
        }
    }

    ExplanationKind::Concept
}

fn generate_description(
    target: &str,
    kind: &ExplanationKind,
    locations: &[SourceLocation],
    detailed: bool,
) -> String {
    let mut description = String::new();

    // Add kind-specific description
    description.push_str(&format!("{:?}: {}\n\n", kind, target));

    // Extract documentation comments if available
    if let Some(first_loc) = locations.first() {
        if let Ok(content) = std::fs::read_to_string(&first_loc.file) {
            let lines: Vec<&str> = content.lines().collect();
            let start_line = first_loc.line.saturating_sub(1);

            // Look for doc comments above the definition
            let mut doc_lines = Vec::new();
            for i in (0..start_line).rev() {
                if i >= lines.len() {
                    break;
                }
                let line = lines[i].trim();

                if line.starts_with("///") || line.starts_with("//!") {
                    doc_lines.insert(
                        0,
                        line.trim_start_matches("///")
                            .trim_start_matches("//!")
                            .trim(),
                    );
                } else if line.starts_with("/**") || line.starts_with("/*") {
                    // Multi-line comment
                    doc_lines.insert(
                        0,
                        line.trim_start_matches("/**")
                            .trim_start_matches("/*")
                            .trim(),
                    );
                } else if !line.is_empty() && !line.starts_with("//") {
                    break;
                }
            }

            if !doc_lines.is_empty() {
                description.push_str("Documentation:\n");
                for doc_line in doc_lines {
                    description.push_str(&format!("  {}\n", doc_line));
                }
                description.push('\n');
            }
        }

        // Add definition
        description.push_str(&format!(
            "Defined in: {}:{}\n",
            first_loc.file, first_loc.line
        ));
        description.push_str(&format!("Definition: {}\n", first_loc.context));
    }

    if detailed && locations.len() > 1 {
        description.push_str(&format!("\nFound in {} locations\n", locations.len()));
    }

    description
}

fn find_usage_examples(target: &str, path: &Path) -> Result<Vec<String>> {
    let mut examples = Vec::new();

    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            let entry_path = entry.path();

            if entry_path.is_file() && is_source_file(&entry_path) {
                if let Ok(content) = std::fs::read_to_string(&entry_path) {
                    for line in content.lines() {
                        if line.contains(target) && !is_definition(line, target) {
                            // This is a usage, not a definition
                            examples.push(format!("  {}", line.trim()));

                            if examples.len() >= 5 {
                                return Ok(examples);
                            }
                        }
                    }
                }
            } else if entry_path.is_dir() && !is_ignored_dir(&entry_path) {
                examples.extend(find_usage_examples(target, &entry_path)?);

                if examples.len() >= 5 {
                    return Ok(examples);
                }
            }
        }
    }

    Ok(examples)
}

fn find_related_items(target: &str, locations: &[SourceLocation]) -> Result<Vec<String>> {
    let mut related = Vec::new();

    // Find items in the same files
    for loc in locations {
        if let Ok(content) = std::fs::read_to_string(&loc.file) {
            for line in content.lines() {
                // Look for related functions/types in the same file
                if (line.contains("fn ") || line.contains("struct ") || line.contains("enum "))
                    && !line.contains(target)
                {
                    if let Some(name) = extract_item_name(line) {
                        if !related.contains(&name) && related.len() < 10 {
                            related.push(name);
                        }
                    }
                }
            }
        }
    }

    Ok(related)
}

fn extract_item_name(line: &str) -> Option<String> {
    let trimmed = line.trim();

    for keyword in &[
        "fn ",
        "struct ",
        "enum ",
        "trait ",
        "type ",
        "class ",
        "function ",
    ] {
        if let Some(start) = trimmed.find(keyword) {
            let after_keyword = &trimmed[start + keyword.len()..];
            if let Some(end) = after_keyword.find(['(', '<', ' ', '{']) {
                return Some(after_keyword[..end].trim().to_string());
            }
        }
    }

    None
}

fn explain_concept(target: &str) -> Option<Explanation> {
    let concepts: HashMap<&str, (&str, Vec<&str>)> = [
        (
            "ownership",
            (
                "Rust's ownership system ensures memory safety without garbage collection",
                vec![
                    "Each value has a single owner",
                    "When the owner goes out of scope, the value is dropped",
                    "Values can be moved or borrowed",
                ],
            ),
        ),
        (
            "borrowing",
            (
                "References allow you to access data without taking ownership",
                vec![
                    "Immutable references: &T (multiple allowed)",
                    "Mutable references: &mut T (only one allowed)",
                    "Cannot have mutable and immutable references simultaneously",
                ],
            ),
        ),
        (
            "lifetime",
            (
                "Lifetimes ensure references are always valid",
                vec![
                    "Prevent dangling references",
                    "Explicit lifetime annotations when needed",
                    "Most lifetimes are inferred by the compiler",
                ],
            ),
        ),
        (
            "trait",
            (
                "Traits define shared behavior across types",
                vec![
                    "Similar to interfaces in other languages",
                    "Can have default implementations",
                    "Used for polymorphism and generic programming",
                ],
            ),
        ),
        (
            "async",
            (
                "Asynchronous programming with async/await syntax",
                vec![
                    "Non-blocking I/O operations",
                    "Futures represent values that will be available later",
                    "Runtime (like Tokio) required for execution",
                ],
            ),
        ),
    ]
    .iter()
    .cloned()
    .collect();

    if let Some((desc, points)) = concepts.get(target.to_lowercase().as_str()) {
        let mut description = format!("Concept: {}\n\n{}\n\nKey points:\n", target, desc);
        for point in points {
            description.push_str(&format!("  - {}\n", point));
        }

        return Some(Explanation {
            target: target.to_string(),
            kind: ExplanationKind::Concept,
            description,
            usage_examples: Vec::new(),
            related_items: get_related_concepts(target),
            source_locations: Vec::new(),
        });
    }

    None
}

fn get_related_concepts(concept: &str) -> Vec<String> {
    let relations: HashMap<&str, Vec<&str>> = [
        ("ownership", vec!["borrowing", "lifetime", "move semantics"]),
        ("borrowing", vec!["ownership", "lifetime", "references"]),
        ("lifetime", vec!["ownership", "borrowing", "references"]),
        (
            "trait",
            vec!["generics", "polymorphism", "associated types"],
        ),
        ("async", vec!["futures", "tokio", "await", "runtime"]),
    ]
    .iter()
    .cloned()
    .collect();

    relations
        .get(concept.to_lowercase().as_str())
        .map(|items| items.iter().map(|s| s.to_string()).collect())
        .unwrap_or_default()
}

fn display_explanation(
    explanation: &Explanation,
    show_examples: bool,
    detailed: bool,
    output: &OutputHandler,
) {
    output.info(&format!("\n=== Explanation: {} ===\n", explanation.target));

    // Display description
    output.info(&explanation.description);

    // Display source locations
    if !explanation.source_locations.is_empty() {
        output.info("\n--- Source Locations ---");
        for loc in &explanation.source_locations {
            output.list_item(&format!("{}:{}", loc.file, loc.line));
            if detailed {
                output.info(&format!("    {}", loc.context));
            }
        }
    }

    // Display usage examples
    if show_examples && !explanation.usage_examples.is_empty() {
        output.info("\n--- Usage Examples ---");
        for example in &explanation.usage_examples {
            output.info(example);
        }
    }

    // Display related items
    if !explanation.related_items.is_empty() {
        output.info("\n--- Related Items ---");
        for item in &explanation.related_items {
            output.list_item(item);
        }
    }

    // Display suggestions
    output.info("\n💡 Suggestions:");
    match explanation.kind {
        ExplanationKind::Function => {
            output.list_item("Use --examples to see usage examples");
            output.list_item("Check tests for more usage patterns");
        }
        ExplanationKind::Type => {
            output.list_item("Look for impl blocks for available methods");
            output.list_item("Check for trait implementations");
        }
        ExplanationKind::Trait => {
            output.list_item("Find types that implement this trait");
            output.list_item("Review trait documentation for constraints");
        }
        ExplanationKind::Concept => {
            output.list_item("Review related concepts listed above");
            output.list_item("Check official Rust documentation");
        }
        _ => {
            output.list_item("Use --detailed for more information");
        }
    }
}

fn is_source_file(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        matches!(
            ext.to_str(),
            Some("rs") | Some("js") | Some("ts") | Some("py") | Some("go") | Some("java")
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
            "target"
                | "node_modules"
                | ".git"
                | "dist"
                | "build"
                | ".output"
                | ".nuxt"
                | ".next"
                | "coverage"
                | ".cache"
                | "tmp"
                | "temp"
                | ".vscode"
                | ".idea"
        )
    } else {
        false
    }
}
