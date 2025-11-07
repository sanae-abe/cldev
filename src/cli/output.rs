use crate::core::i18n::I18n;
use colored::{ColoredString, Colorize};
use std::collections::HashMap;
use std::io::{self, Write};

/// Output level for controlling verbosity
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OutputLevel {
    /// No output except errors
    Quiet = 0,
    /// Normal output
    Normal = 1,
    /// Verbose output with details
    Verbose = 2,
}

/// Handler for structured output with color support and i18n
pub struct OutputHandler {
    level: OutputLevel,
    use_color: bool,
    i18n: I18n,
}

impl OutputHandler {
    /// Create a new output handler
    pub fn new(verbose: bool, quiet: bool, no_color: bool) -> Self {
        let level = if quiet {
            OutputLevel::Quiet
        } else if verbose {
            OutputLevel::Verbose
        } else {
            OutputLevel::Normal
        };

        Self {
            level,
            use_color: !no_color && atty::is(atty::Stream::Stdout),
            i18n: I18n::new(),
        }
    }

    /// Get a reference to the i18n handler
    pub fn i18n(&self) -> &I18n {
        &self.i18n
    }

    /// Get a localized message
    pub fn t(&self, key: &str) -> String {
        self.i18n.get(key)
    }

    /// Get a localized message with variable substitution
    pub fn t_with_vars(&self, key: &str, vars: &HashMap<&str, &str>) -> String {
        self.i18n.get_with_vars(key, vars)
    }

    /// Get a localized message with a single variable
    pub fn t_format(&self, key: &str, var_name: &str, var_value: &str) -> String {
        self.i18n.format(key, var_name, var_value)
    }

    /// Print a success message
    pub fn success(&self, msg: &str) {
        if self.level >= OutputLevel::Normal {
            self.println(&self.colorize("✓", msg, |s| s.green().bold()));
        }
    }

    /// Print an error message
    pub fn error(&self, msg: &str) {
        self.eprintln(&self.colorize("✗", msg, |s| s.red().bold()));
    }

    /// Print a warning message
    pub fn warning(&self, msg: &str) {
        if self.level >= OutputLevel::Normal {
            self.println(&self.colorize("⚠", msg, |s| s.yellow().bold()));
        }
    }

    /// Print an info message
    pub fn info(&self, msg: &str) {
        if self.level >= OutputLevel::Normal {
            self.println(&self.colorize("ℹ", msg, |s| s.blue()));
        }
    }

    /// Print a verbose/debug message
    pub fn debug(&self, msg: &str) {
        if self.level >= OutputLevel::Verbose {
            self.println(&self.colorize("→", msg, |s| s.dimmed()));
        }
    }

    /// Print a step message (for multi-step operations)
    pub fn step(&self, step: usize, total: usize, msg: &str) {
        if self.level >= OutputLevel::Normal {
            let prefix = format!("[{}/{}]", step, total);
            self.println(&self.colorize(&prefix, msg, |s| s.cyan()));
        }
    }

    /// Print a header/section message
    pub fn header(&self, msg: &str) {
        if self.level >= OutputLevel::Normal {
            let separator = "=".repeat(msg.len() + 4);
            self.println(&self.apply_color(&separator, |s| s.bright_blue()));
            self.println(&self.apply_color(&format!("  {}  ", msg), |s| s.bright_blue().bold()));
            self.println(&self.apply_color(&separator, |s| s.bright_blue()));
        }
    }

    /// Print a subheader message
    pub fn subheader(&self, msg: &str) {
        if self.level >= OutputLevel::Normal {
            self.println(&self.apply_color(&format!("--- {} ---", msg), |s| s.cyan()));
        }
    }

    /// Print a section message (alias for subheader)
    pub fn section(&self, msg: &str) {
        self.subheader(msg)
    }

    /// Print raw message without formatting
    pub fn raw(&self, msg: &str) {
        if self.level >= OutputLevel::Normal {
            self.println(msg);
        }
    }

    /// Print a list item
    pub fn list_item(&self, msg: &str) {
        if self.level >= OutputLevel::Normal {
            self.println(&self.colorize("•", msg, |s| s.normal()));
        }
    }

    /// Print a key-value pair
    pub fn key_value(&self, key: &str, value: &str) {
        if self.level >= OutputLevel::Normal {
            let key_colored = self.apply_color(key, |s| s.cyan().bold());
            self.println(&format!("{}: {}", key_colored, value));
        }
    }

    /// Start a spinner/progress indicator (returns a message to show)
    pub fn start_progress(&self, msg: &str) -> String {
        if self.level >= OutputLevel::Normal {
            let formatted = self.colorize("⏳", msg, |s| s.yellow());
            eprint!("\r{}", formatted);
            io::stderr().flush().ok();
            formatted
        } else {
            String::new()
        }
    }

    /// Finish a progress indicator
    pub fn finish_progress(&self, msg: &str, success: bool) {
        if self.level >= OutputLevel::Normal {
            let formatted = if success {
                self.colorize("✓", msg, |s| s.green().bold())
            } else {
                self.colorize("✗", msg, |s| s.red().bold())
            };
            eprintln!("\r{}", formatted);
        }
    }

    /// Create a formatted message with prefix and color
    fn colorize<F>(&self, prefix: &str, msg: &str, color_fn: F) -> String
    where
        F: Fn(ColoredString) -> ColoredString,
    {
        if self.use_color {
            format!("{} {}", color_fn(prefix.into()), msg)
        } else {
            format!("{} {}", prefix, msg)
        }
    }

    /// Apply color to a string
    fn apply_color<F>(&self, text: &str, color_fn: F) -> String
    where
        F: Fn(ColoredString) -> ColoredString,
    {
        if self.use_color {
            color_fn(text.into()).to_string()
        } else {
            text.to_string()
        }
    }

    /// Print to stdout
    fn println(&self, msg: &str) {
        println!("{}", msg);
    }

    /// Print to stderr
    fn eprintln(&self, msg: &str) {
        eprintln!("{}", msg);
    }

    /// Check if verbose mode is enabled
    pub fn is_verbose(&self) -> bool {
        self.level >= OutputLevel::Verbose
    }

    /// Check if quiet mode is enabled
    pub fn is_quiet(&self) -> bool {
        self.level == OutputLevel::Quiet
    }

    /// Get current output level
    pub fn level(&self) -> OutputLevel {
        self.level
    }
}

impl Default for OutputHandler {
    fn default() -> Self {
        Self::new(false, false, false)
    }
}

/// Helper macro for conditional output
#[macro_export]
macro_rules! output_debug {
    ($handler:expr, $($arg:tt)*) => {
        if $handler.is_verbose() {
            $handler.debug(&format!($($arg)*));
        }
    };
}

/// Helper macro for progress tracking
#[macro_export]
macro_rules! with_progress {
    ($handler:expr, $msg:expr, $block:block) => {{
        let _progress = $handler.start_progress($msg);
        let result = $block;
        let success = result.is_ok();
        $handler.finish_progress($msg, success);
        result
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_levels() {
        assert!(OutputLevel::Verbose > OutputLevel::Normal);
        assert!(OutputLevel::Normal > OutputLevel::Quiet);
    }

    #[test]
    fn test_handler_creation() {
        let handler = OutputHandler::new(false, false, false);
        assert_eq!(handler.level(), OutputLevel::Normal);
        assert!(!handler.is_verbose());
        assert!(!handler.is_quiet());

        let verbose_handler = OutputHandler::new(true, false, false);
        assert_eq!(verbose_handler.level(), OutputLevel::Verbose);
        assert!(verbose_handler.is_verbose());

        let quiet_handler = OutputHandler::new(false, true, false);
        assert_eq!(quiet_handler.level(), OutputLevel::Quiet);
        assert!(quiet_handler.is_quiet());
    }

    #[test]
    fn test_color_application() {
        let handler_color = OutputHandler::new(false, false, false);
        let handler_no_color = OutputHandler::new(false, false, true);

        // Both should produce non-empty output
        let with_color = handler_color.colorize("✓", "test", |s| s.green());
        let without_color = handler_no_color.colorize("✓", "test", |s| s.green());

        assert!(with_color.contains("test"));
        assert!(without_color.contains("test"));
    }
}
