use cldev::cli::output::OutputHandler;
/// Demonstration of the i18n system
///
/// This example shows how to use the I18n system with different languages
/// and variable substitution.
///
/// Run with: cargo run --example i18n_demo
use cldev::core::i18n::{I18n, Language};
use std::collections::HashMap;

fn main() {
    println!("=== cldev i18n System Demo ===\n");

    // Demo 1: Automatic language detection
    println!("--- Demo 1: Automatic Language Detection ---");
    let i18n = I18n::new();
    println!("Detected language: {}", i18n.language());
    println!("Message 'command-success': {}", i18n.get("command-success"));
    println!();

    // Demo 2: Explicit language setting (English)
    println!("--- Demo 2: English Messages ---");
    let i18n_en = I18n::with_language(Language::English);
    println!("command-success: {}", i18n_en.get("command-success"));
    println!(
        "config-check-success: {}",
        i18n_en.get("config-check-success")
    );
    println!(
        "config-init-success: {}",
        i18n_en.get("config-init-success")
    );
    println!();

    // Demo 3: Japanese messages
    println!("--- Demo 3: Japanese Messages ---");
    let i18n_ja = I18n::with_language(Language::Japanese);
    println!("command-success: {}", i18n_ja.get("command-success"));
    println!(
        "config-check-success: {}",
        i18n_ja.get("config-check-success")
    );
    println!(
        "config-init-success: {}",
        i18n_ja.get("config-init-success")
    );
    println!();

    // Demo 4: Variable substitution with single variable
    println!("--- Demo 4: Single Variable Substitution ---");
    let next_step_en = i18n_en.format("next-step", "command", "cldev config check");
    let next_step_ja = i18n_ja.format("next-step", "command", "cldev config check");
    println!("English: {}", next_step_en);
    println!("Japanese: {}", next_step_ja);
    println!();

    // Demo 5: Variable substitution with multiple variables
    println!("--- Demo 5: Multiple Variable Substitution ---");
    let mut vars = HashMap::new();
    vars.insert("current", "1");
    vars.insert("total", "5");
    let step_en = i18n_en.get_with_vars("step", &vars);
    let step_ja = i18n_ja.get_with_vars("step", &vars);
    println!("English: {}", step_en);
    println!("Japanese: {}", step_ja);
    println!();

    // Demo 6: OutputHandler with i18n integration
    println!("--- Demo 6: OutputHandler with i18n ---");
    let output = OutputHandler::new(false, false, false);
    println!("OutputHandler language: {}", output.i18n().language());
    output.success(&output.t("config-check-success"));
    output.info(&output.t_format("next-step", "command", "cldev config init"));
    println!();

    // Demo 7: Fallback behavior
    println!("--- Demo 7: Fallback Behavior ---");
    println!(
        "Non-existent key in English: {}",
        i18n_en.get("non-existent-key")
    );
    println!(
        "Non-existent key in Japanese: {}",
        i18n_ja.get("non-existent-key")
    );
    println!();

    // Demo 8: Language switching
    println!("--- Demo 8: Dynamic Language Switching ---");
    let mut i18n_switch = I18n::with_language(Language::English);
    println!("Initial (English): {}", i18n_switch.get("command-success"));
    i18n_switch.set_language(Language::Japanese);
    println!(
        "After switch (Japanese): {}",
        i18n_switch.get("command-success")
    );
    println!();

    // Demo 9: Available languages
    println!("--- Demo 9: Available Languages ---");
    let available = i18n.available_languages();
    println!("Supported languages: {:?}", available);
    for lang in available {
        println!("  - {} ({})", lang, lang.code());
    }
    println!();

    println!("=== Demo Complete ===");
}
