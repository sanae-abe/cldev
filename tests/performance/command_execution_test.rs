use std::process::Command;
use std::time::{Duration, Instant};
use tempfile::TempDir;

const TARGET_COMMAND_OVERHEAD: Duration = Duration::from_millis(50);

#[test]
fn test_init_command_performance() {
    let temp_dir = TempDir::new().unwrap();

    let start = Instant::now();
    let output = Command::new("cargo")
        .args(&["run", "--release", "--", "init"])
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to execute init command");
    let elapsed = start.elapsed();

    assert!(output.status.success());
    println!("Init command execution time: {:?}", elapsed);

    assert!(
        elapsed <= Duration::from_secs(1),
        "Init command time {:?} exceeds 1 second",
        elapsed
    );
}

#[test]
fn test_config_get_command_performance() {
    let temp_dir = TempDir::new().unwrap();

    // Initialize first
    Command::new("cargo")
        .args(&["run", "--release", "--", "init"])
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to initialize");

    // Set a value
    Command::new("cargo")
        .args(&["run", "--release", "--", "config", "set", "test.key", "value"])
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to set config");

    // Measure get command
    let start = Instant::now();
    let output = Command::new("cargo")
        .args(&["run", "--release", "--", "config", "get", "test.key"])
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to get config");
    let elapsed = start.elapsed();

    assert!(output.status.success());
    println!("Config get command execution time: {:?}", elapsed);

    assert!(
        elapsed <= TARGET_COMMAND_OVERHEAD,
        "Config get command time {:?} exceeds target {:?}",
        elapsed,
        TARGET_COMMAND_OVERHEAD
    );
}

#[test]
fn test_config_set_command_performance() {
    let temp_dir = TempDir::new().unwrap();

    // Initialize first
    Command::new("cargo")
        .args(&["run", "--release", "--", "init"])
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to initialize");

    let start = Instant::now();
    let output = Command::new("cargo")
        .args(&["run", "--release", "--", "config", "set", "test.key", "value"])
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to set config");
    let elapsed = start.elapsed();

    assert!(output.status.success());
    println!("Config set command execution time: {:?}", elapsed);

    assert!(
        elapsed <= TARGET_COMMAND_OVERHEAD,
        "Config set command time {:?} exceeds target {:?}",
        elapsed,
        TARGET_COMMAND_OVERHEAD
    );
}

#[test]
fn test_status_command_performance() {
    let temp_dir = TempDir::new().unwrap();

    // Initialize first
    Command::new("cargo")
        .args(&["run", "--release", "--", "init"])
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to initialize");

    let start = Instant::now();
    let output = Command::new("cargo")
        .args(&["run", "--release", "--", "status"])
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to get status");
    let elapsed = start.elapsed();

    assert!(output.status.success());
    println!("Status command execution time: {:?}", elapsed);

    assert!(
        elapsed <= Duration::from_millis(200),
        "Status command time {:?} exceeds 200ms",
        elapsed
    );
}

#[test]
fn test_multiple_command_overhead() {
    let temp_dir = TempDir::new().unwrap();

    // Initialize
    Command::new("cargo")
        .args(&["run", "--release", "--", "init"])
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to initialize");

    let commands = vec![
        vec!["config", "set", "key1", "value1"],
        vec!["config", "get", "key1"],
        vec!["config", "set", "key2", "value2"],
        vec!["config", "get", "key2"],
        vec!["status"],
    ];

    let total_start = Instant::now();
    for cmd in &commands {
        let output = Command::new("cargo")
            .args(&["run", "--release", "--"])
            .args(cmd)
            .current_dir(temp_dir.path())
            .output()
            .expect("Failed to execute command");

        assert!(output.status.success());
    }
    let total_elapsed = total_start.elapsed();

    let avg_time = total_elapsed / commands.len() as u32;
    println!("Average command execution time: {:?}", avg_time);
    println!("Total execution time: {:?}", total_elapsed);

    assert!(
        avg_time <= TARGET_COMMAND_OVERHEAD * 2,
        "Average command time {:?} exceeds target {:?}",
        avg_time,
        TARGET_COMMAND_OVERHEAD * 2
    );
}

#[test]
fn test_command_with_error_handling_performance() {
    let temp_dir = TempDir::new().unwrap();

    // Initialize
    Command::new("cargo")
        .args(&["run", "--release", "--", "init"])
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to initialize");

    // Test error case (non-existent key)
    let start = Instant::now();
    let output = Command::new("cargo")
        .args(&["run", "--release", "--", "config", "get", "nonexistent.key"])
        .current_dir(temp_dir.path())
        .output()
        .expect("Failed to execute command");
    let elapsed = start.elapsed();

    println!("Error handling time: {:?}", elapsed);

    // Error handling should not significantly slow down
    assert!(
        elapsed <= TARGET_COMMAND_OVERHEAD * 2,
        "Error handling time {:?} exceeds target {:?}",
        elapsed,
        TARGET_COMMAND_OVERHEAD * 2
    );
}

#[test]
fn test_completion_generation_performance() {
    let temp_dir = TempDir::new().unwrap();

    let shells = vec!["bash", "zsh", "fish"];

    for shell in shells {
        let start = Instant::now();
        let output = Command::new("cargo")
            .args(&["run", "--release", "--", "completion", shell])
            .current_dir(temp_dir.path())
            .output()
            .expect("Failed to generate completion");
        let elapsed = start.elapsed();

        assert!(output.status.success());
        println!("{} completion generation time: {:?}", shell, elapsed);

        assert!(
            elapsed <= Duration::from_millis(100),
            "{} completion generation time {:?} exceeds 100ms",
            shell,
            elapsed
        );
    }
}
