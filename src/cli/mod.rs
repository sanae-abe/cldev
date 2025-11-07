pub mod args;
pub mod completions;
pub mod output;

pub use args::{Cli, Commands};
pub use completions::{generate_completions, print_installation_instructions};
pub use output::OutputHandler;
