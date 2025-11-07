use cldev::core::Config;
use std::time::{Duration, Instant};
use tempfile::TempDir;

const TARGET_LOAD_TIME: Duration = Duration::from_millis(10);

#[test]
fn test_config_load_empty() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.toml");

    // Create empty config
    let config = Config::default();
    config.save(Some(config_path.clone())).unwrap();

    let start = Instant::now();
    let _loaded = Config::load(Some(config_path)).unwrap();
    let elapsed = start.elapsed();

    println!("Empty config load time: {:?}", elapsed);

    assert!(
        elapsed <= TARGET_LOAD_TIME,
        "Empty config load time {:?} exceeds target {:?}",
        elapsed,
        TARGET_LOAD_TIME
    );
}

#[test]
fn test_config_default_creation() {
    let start = Instant::now();
    let _config = Config::default();
    let elapsed = start.elapsed();

    println!("Config default creation time: {:?}", elapsed);

    assert!(
        elapsed <= Duration::from_micros(100),
        "Config default creation time {:?} exceeds 100μs",
        elapsed
    );
}

#[test]
fn test_config_save_performance() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.toml");

    let config = Config::default();

    let start = Instant::now();
    config.save(Some(config_path)).unwrap();
    let elapsed = start.elapsed();

    println!("Config save time: {:?}", elapsed);

    assert!(
        elapsed <= TARGET_LOAD_TIME * 2,
        "Config save time {:?} exceeds target {:?}",
        elapsed,
        TARGET_LOAD_TIME * 2
    );
}

#[test]
fn test_config_clone_performance() {
    let config = Config::default();

    let start = Instant::now();
    let _cloned = config.clone();
    let elapsed = start.elapsed();

    println!("Config clone time: {:?}", elapsed);

    assert!(
        elapsed <= Duration::from_micros(10),
        "Config clone time {:?} exceeds 10μs",
        elapsed
    );
}

#[test]
fn test_config_load_save_cycle() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.toml");

    let original = Config::default();
    original.save(Some(config_path.clone())).unwrap();

    let start = Instant::now();
    let loaded = Config::load(Some(config_path.clone())).unwrap();
    let elapsed_load = start.elapsed();

    let start = Instant::now();
    loaded.save(Some(config_path)).unwrap();
    let elapsed_save = start.elapsed();

    println!("Load-save cycle: load {:?}, save {:?}", elapsed_load, elapsed_save);

    let total = elapsed_load + elapsed_save;
    assert!(
        total <= TARGET_LOAD_TIME * 3,
        "Load-save cycle time {:?} exceeds target {:?}",
        total,
        TARGET_LOAD_TIME * 3
    );
}
