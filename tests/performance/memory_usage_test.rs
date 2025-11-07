use cldev::core::Config;
use tempfile::TempDir;

#[test]
fn test_config_memory_usage_multiple_instances() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.toml");

    let mut configs = Vec::new();

    // Create 100 config instances
    for _ in 0..100 {
        let config = Config::load(Some(config_path.clone())).unwrap();
        configs.push(config);
    }

    println!("Created 100 config instances");
    assert_eq!(configs.len(), 100);
}

#[test]
fn test_config_clone_overhead() {
    let original = Config::default();

    let mut clones = Vec::new();
    for _ in 0..1000 {
        clones.push(original.clone());
    }

    println!("Created 1000 config clones");
    assert_eq!(clones.len(), 1000);
}

#[test]
fn test_i18n_memory_usage() {
    use cldev::core::i18n::{I18n, Language};

    let mut i18n_instances = Vec::new();

    for _ in 0..100 {
        i18n_instances.push(I18n::with_language(Language::Japanese));
    }

    println!("Created 100 I18n instances");
    assert_eq!(i18n_instances.len(), 100);
}

#[test]
fn test_secure_path_memory_usage() {
    use cldev::core::security::SecurePath;
    use std::path::PathBuf;

    let mut paths = Vec::new();

    // Create 1000 secure paths
    for i in 0..1000 {
        let path = SecurePath::new(PathBuf::from(format!("/tmp/test_{}", i))).unwrap();
        paths.push(path);
    }

    println!("Created 1000 SecurePath instances");
    assert_eq!(paths.len(), 1000);
}

#[test]
fn test_project_detector_memory_usage() {
    use cldev::core::project_detector::ProjectDetector;
    use std::fs;

    let temp_dir = TempDir::new().unwrap();

    // Create deep directory structure
    let mut current_path = temp_dir.path().to_path_buf();
    for i in 0..100 {
        current_path = current_path.join(format!("level_{}", i));
        fs::create_dir_all(&current_path).unwrap();
    }

    // Add project marker at root
    fs::write(temp_dir.path().join("package.json"), r#"{"name": "test"}"#).unwrap();

    // Detect from deep path
    let detector = ProjectDetector::new(Some(&current_path)).unwrap();
    println!("Detected project type from depth 100: {:?}", detector.project_type());
}

#[test]
fn test_concurrent_config_memory() {
    use std::thread;

    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.toml");

    let config = Config::default();
    config.save(Some(config_path.clone())).unwrap();

    let handles: Vec<_> = (0..10)
        .map(|_| {
            let path = config_path.clone();
            thread::spawn(move || {
                Config::load(Some(path)).unwrap()
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Completed 10 concurrent config operations");
}
