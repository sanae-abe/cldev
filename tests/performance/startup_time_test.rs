use std::process::Command;
use std::time::{Duration, Instant};

const TARGET_STARTUP_TIME: Duration = Duration::from_millis(100);
const ITERATIONS: usize = 10;

#[test]
fn test_cli_startup_time() {
    let mut timings = Vec::new();

    for _ in 0..ITERATIONS {
        let start = Instant::now();

        let output = Command::new("cargo")
            .args(&["run", "--release", "--", "--version"])
            .output()
            .expect("Failed to execute command");

        let elapsed = start.elapsed();
        timings.push(elapsed);

        assert!(
            output.status.success(),
            "Command failed with status: {}",
            output.status
        );
    }

    let avg_time = timings.iter().sum::<Duration>() / ITERATIONS as u32;
    let min_time = timings.iter().min().unwrap();
    let max_time = timings.iter().max().unwrap();

    println!("Startup time statistics:");
    println!("  Average: {:?}", avg_time);
    println!("  Minimum: {:?}", min_time);
    println!("  Maximum: {:?}", max_time);
    println!("  Target:  {:?}", TARGET_STARTUP_TIME);

    assert!(
        avg_time <= TARGET_STARTUP_TIME,
        "Average startup time {:?} exceeds target {:?}",
        avg_time,
        TARGET_STARTUP_TIME
    );
}

#[test]
fn test_help_command_startup() {
    let start = Instant::now();

    let output = Command::new("cargo")
        .args(&["run", "--release", "--", "--help"])
        .output()
        .expect("Failed to execute command");

    let elapsed = start.elapsed();

    assert!(output.status.success());
    println!("Help command startup time: {:?}", elapsed);

    assert!(
        elapsed <= TARGET_STARTUP_TIME,
        "Help command startup time {:?} exceeds target {:?}",
        elapsed,
        TARGET_STARTUP_TIME
    );
}

#[test]
fn test_subcommand_startup() {
    let commands = vec!["config", "init", "status"];

    for cmd in commands {
        let start = Instant::now();

        let output = Command::new("cargo")
            .args(&["run", "--release", "--", cmd, "--help"])
            .output()
            .expect("Failed to execute command");

        let elapsed = start.elapsed();

        assert!(output.status.success());
        println!("{} subcommand startup time: {:?}", cmd, elapsed);

        assert!(
            elapsed <= TARGET_STARTUP_TIME * 2, // Allow 2x for subcommands
            "{} subcommand startup time {:?} exceeds target {:?}",
            cmd,
            elapsed,
            TARGET_STARTUP_TIME * 2
        );
    }
}

#[test]
fn test_cold_start_vs_warm_start() {
    // Cold start
    let cold_start = Instant::now();
    Command::new("cargo")
        .args(&["run", "--release", "--", "--version"])
        .output()
        .expect("Failed to execute command");
    let cold_elapsed = cold_start.elapsed();

    // Warm starts
    let mut warm_timings = Vec::new();
    for _ in 0..5 {
        let warm_start = Instant::now();
        Command::new("cargo")
            .args(&["run", "--release", "--", "--version"])
            .output()
            .expect("Failed to execute command");
        warm_timings.push(warm_start.elapsed());
    }

    let avg_warm = warm_timings.iter().sum::<Duration>() / warm_timings.len() as u32;

    println!("Cold start time: {:?}", cold_elapsed);
    println!("Warm start time (avg): {:?}", avg_warm);

    // Warm start should be faster or equal
    assert!(avg_warm <= cold_elapsed * 2);
}

#[test]
fn test_binary_size() {
    let output = Command::new("cargo")
        .args(&["build", "--release"])
        .output()
        .expect("Failed to build");

    assert!(output.status.success());

    let binary_path = std::env::current_dir()
        .unwrap()
        .join("target/release/cldev");

    if binary_path.exists() {
        let metadata = std::fs::metadata(&binary_path).unwrap();
        let size_mb = metadata.len() as f64 / 1024.0 / 1024.0;

        println!("Binary size: {:.2} MB", size_mb);

        // Ensure binary size is reasonable (< 10MB for a CLI tool)
        assert!(
            size_mb < 10.0,
            "Binary size {:.2} MB is too large",
            size_mb
        );
    }
}
