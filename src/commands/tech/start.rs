use crate::cli::args::{Environment, TechStack};
use crate::core::{CldevError, ProjectDetector, Result};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::process::{Command, Stdio};

/// Handle tech stack start command
pub fn handle_start(stack: TechStack, port: Option<u16>, env: Environment) -> Result<()> {
    println!("{}", "🚀 Starting development environment...".cyan().bold());

    // Auto-detect project type if possible
    let detected_type = ProjectDetector::new(None).ok().map(|d| d.project_type());
    println!(
        "{} Tech stack: {}",
        "ℹ️".cyan(),
        format!("{:?}", stack).green()
    );

    if let Some(detected) = detected_type {
        println!(
            "{} Detected project: {}",
            "ℹ️".cyan(),
            format!("{:?}", detected).yellow()
        );
    }

    // Environment setup
    let env_name = match env {
        Environment::Development => "development",
        Environment::Staging => "staging",
        Environment::Production => "production",
    };
    println!("{} Environment: {}", "ℹ️".cyan(), env_name.yellow());

    // Port configuration
    let default_port = get_default_port(&stack);
    let actual_port = port.unwrap_or(default_port);
    println!("{} Port: {}", "ℹ️".cyan(), actual_port.to_string().cyan());

    // Pre-start checks
    println!("\n{}", "🔍 Running pre-start checks...".cyan());
    run_prestart_checks(&stack)?;

    // Start the development environment
    println!("\n{}", "🎬 Starting services...".cyan().bold());
    start_development_server(&stack, actual_port, env)?;

    println!(
        "\n{}",
        "✅ Development environment started successfully!"
            .green()
            .bold()
    );
    display_access_info(&stack, actual_port);

    Ok(())
}

/// Get default port for tech stack
fn get_default_port(stack: &TechStack) -> u16 {
    match stack {
        TechStack::Web => 3000,
        TechStack::Api => 8000,
        TechStack::Mobile => 8081,
        TechStack::DataScience => 8888,
    }
}

/// Run pre-start checks
fn run_prestart_checks(stack: &TechStack) -> Result<()> {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );

    match stack {
        TechStack::Web => {
            pb.set_message("Checking Node.js installation...");
            check_node_installation()?;
            println!("  {} Node.js available", "✓".green());

            pb.set_message("Checking dependencies...");
            check_dependencies()?;
            println!("  {} Dependencies installed", "✓".green());
        }
        TechStack::Api => {
            pb.set_message("Checking runtime environment...");
            check_api_runtime()?;
            println!("  {} Runtime available", "✓".green());
        }
        TechStack::Mobile => {
            pb.set_message("Checking mobile development tools...");
            check_mobile_tools()?;
            println!("  {} Mobile tools available", "✓".green());
        }
        TechStack::DataScience => {
            pb.set_message("Checking Jupyter installation...");
            check_jupyter_installation()?;
            println!("  {} Jupyter available", "✓".green());
        }
    }

    pb.finish_and_clear();
    Ok(())
}

/// Check Node.js installation
fn check_node_installation() -> Result<()> {
    let output = Command::new("node")
        .arg("--version")
        .output()
        .map_err(|_| CldevError::command("Node.js not found. Please install Node.js"))?;

    if !output.status.success() {
        return Err(CldevError::command("Node.js check failed"));
    }

    Ok(())
}

/// Check if dependencies are installed
fn check_dependencies() -> Result<()> {
    if std::path::Path::new("package.json").exists()
        && !std::path::Path::new("node_modules").exists() {
            println!("  {} Installing dependencies...", "→".cyan());
            let status = Command::new("npm").arg("install").status()?;

            if !status.success() {
                return Err(CldevError::command("Failed to install dependencies"));
            }
        }

    Ok(())
}

/// Check API runtime (Node.js, Python, Go, Rust)
fn check_api_runtime() -> Result<()> {
    // Try different runtimes
    let runtimes = vec![
        ("node", "--version"),
        ("python", "--version"),
        ("go", "version"),
        ("cargo", "--version"),
    ];

    for (cmd, arg) in runtimes {
        if Command::new(cmd).arg(arg).output().is_ok() {
            return Ok(());
        }
    }

    Err(CldevError::command(
        "No API runtime found. Please install Node.js, Python, Go, or Rust",
    ))
}

/// Check mobile development tools
fn check_mobile_tools() -> Result<()> {
    // Check for React Native or Flutter
    let has_react_native =
        std::path::Path::new("package.json").exists() && std::path::Path::new("app.json").exists();

    let has_flutter = std::path::Path::new("pubspec.yaml").exists();

    if !has_react_native && !has_flutter {
        return Err(CldevError::command(
            "No mobile project detected. Expected React Native or Flutter project",
        ));
    }

    if has_flutter {
        Command::new("flutter")
            .arg("--version")
            .output()
            .map_err(|_| CldevError::command("Flutter not found. Please install Flutter SDK"))?;
    }

    Ok(())
}

/// Check Jupyter installation
fn check_jupyter_installation() -> Result<()> {
    let output = Command::new("jupyter")
        .arg("--version")
        .output()
        .map_err(|_| CldevError::command("Jupyter not found. Install with: pip install jupyter"))?;

    if !output.status.success() {
        return Err(CldevError::command("Jupyter check failed"));
    }

    Ok(())
}

/// Start development server based on tech stack
fn start_development_server(stack: &TechStack, port: u16, env: Environment) -> Result<()> {
    match stack {
        TechStack::Web => start_web_dev_server(port, env),
        TechStack::Api => start_api_dev_server(port, env),
        TechStack::Mobile => start_mobile_dev_server(port),
        TechStack::DataScience => start_jupyter_server(port),
    }
}

/// Start web development server
fn start_web_dev_server(port: u16, env: Environment) -> Result<()> {
    println!(
        "  {} Starting web dev server on port {}...",
        "→".cyan(),
        port
    );

    let env_name = match env {
        Environment::Development => "development",
        Environment::Staging => "staging",
        Environment::Production => "production",
    };

    // Check for common web frameworks
    if std::path::Path::new("next.config.js").exists()
        || std::path::Path::new("next.config.ts").exists()
    {
        // Next.js
        println!("  {} Detected Next.js project", "→".cyan());
        Command::new("npm")
            .args(["run", "dev", "--", "-p", &port.to_string()])
            .env("NODE_ENV", env_name)
            .spawn()?;
    } else if std::path::Path::new("vite.config.js").exists()
        || std::path::Path::new("vite.config.ts").exists()
    {
        // Vite
        println!("  {} Detected Vite project", "→".cyan());
        Command::new("npm")
            .args(["run", "dev", "--", "--port", &port.to_string()])
            .env("NODE_ENV", env_name)
            .spawn()?;
    } else if std::path::Path::new("package.json").exists() {
        // Generic npm project
        Command::new("npm")
            .args(["run", "dev"])
            .env("PORT", port.to_string())
            .env("NODE_ENV", env_name)
            .spawn()?;
    } else {
        return Err(CldevError::command("No web framework detected"));
    }

    println!("  {} Web server starting...", "✓".green());
    Ok(())
}

/// Start API development server
fn start_api_dev_server(port: u16, env: Environment) -> Result<()> {
    println!("  {} Starting API server on port {}...", "→".cyan(), port);

    let env_name = match env {
        Environment::Development => "development",
        Environment::Staging => "staging",
        Environment::Production => "production",
    };

    // Try different API frameworks
    if std::path::Path::new("main.py").exists() || std::path::Path::new("app.py").exists() {
        // Python (FastAPI/Flask)
        if std::path::Path::new("requirements.txt").exists() {
            let content = std::fs::read_to_string("requirements.txt")?;
            if content.contains("fastapi") {
                println!("  {} Detected FastAPI project", "→".cyan());
                Command::new("uvicorn")
                    .args(["main:app", "--reload", "--port", &port.to_string()])
                    .env("ENVIRONMENT", env_name)
                    .spawn()?;
            } else if content.contains("flask") {
                println!("  {} Detected Flask project", "→".cyan());
                Command::new("flask")
                    .args(["run", "--port", &port.to_string()])
                    .env("FLASK_ENV", env_name)
                    .spawn()?;
            }
        }
    } else if std::path::Path::new("package.json").exists() {
        // Node.js API
        Command::new("npm")
            .args(["run", "dev"])
            .env("PORT", port.to_string())
            .env("NODE_ENV", env_name)
            .spawn()?;
    } else if std::path::Path::new("Cargo.toml").exists() {
        // Rust API
        println!("  {} Detected Rust project", "→".cyan());
        Command::new("cargo")
            .args(["run"])
            .env("PORT", port.to_string())
            .env("RUST_ENV", env_name)
            .spawn()?;
    } else if std::path::Path::new("go.mod").exists() {
        // Go API
        println!("  {} Detected Go project", "→".cyan());
        Command::new("go")
            .args(["run", "."])
            .env("PORT", port.to_string())
            .spawn()?;
    }

    println!("  {} API server starting...", "✓".green());
    Ok(())
}

/// Start mobile development server
fn start_mobile_dev_server(port: u16) -> Result<()> {
    println!(
        "  {} Starting mobile dev server on port {}...",
        "→".cyan(),
        port
    );

    if std::path::Path::new("pubspec.yaml").exists() {
        // Flutter
        println!("  {} Starting Flutter development...", "→".cyan());
        println!("  {} Run: flutter run", "→".cyan());
    } else if std::path::Path::new("app.json").exists() {
        // React Native
        println!("  {} Starting React Native Metro bundler...", "→".cyan());
        Command::new("npx")
            .args(["react-native", "start", "--port", &port.to_string()])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?;
    }

    println!("  {} Mobile dev server starting...", "✓".green());
    Ok(())
}

/// Start Jupyter server
fn start_jupyter_server(port: u16) -> Result<()> {
    println!(
        "  {} Starting Jupyter server on port {}...",
        "→".cyan(),
        port
    );

    Command::new("jupyter")
        .args(["lab", "--port", &port.to_string(), "--no-browser"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    println!("  {} Jupyter server starting...", "✓".green());
    Ok(())
}

/// Display access information
fn display_access_info(stack: &TechStack, port: u16) {
    use comfy_table::{modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL, Table};

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec!["Service", "URL"]);

    let url = match stack {
        TechStack::Web => format!("http://localhost:{}", port),
        TechStack::Api => format!("http://localhost:{}/api", port),
        TechStack::Mobile => format!("http://localhost:{}", port),
        TechStack::DataScience => format!("http://localhost:{}/lab", port),
    };

    table.add_row(vec![&format!("{:?}", stack), &url]);

    println!("\n{}", table);
    println!("\n{}", "Press Ctrl+C to stop the server".yellow());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_ports() {
        assert_eq!(get_default_port(&TechStack::Web), 3000);
        assert_eq!(get_default_port(&TechStack::Api), 8000);
        assert_eq!(get_default_port(&TechStack::Mobile), 8081);
        assert_eq!(get_default_port(&TechStack::DataScience), 8888);
    }
}
