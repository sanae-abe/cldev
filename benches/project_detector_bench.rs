use cldev::core::project_detector::{ProjectDetector, ProjectType};
use criterion::{criterion_group, criterion_main, Criterion};
use std::fs;
use std::hint::black_box;
use tempfile::TempDir;

fn bench_detect_nodejs(c: &mut Criterion) {
    c.bench_function("detect_nodejs", |b| {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("package.json"), r#"{"name": "test"}"#).unwrap();

        b.iter(|| black_box(ProjectDetector::new(Some(temp_dir.path())).unwrap()))
    });
}

fn bench_detect_rust(c: &mut Criterion) {
    c.bench_function("detect_rust", |b| {
        let temp_dir = TempDir::new().unwrap();
        fs::write(
            temp_dir.path().join("Cargo.toml"),
            "[package]\nname = \"test\"",
        )
        .unwrap();

        b.iter(|| black_box(ProjectDetector::new(Some(temp_dir.path())).unwrap()))
    });
}

fn bench_detect_python(c: &mut Criterion) {
    c.bench_function("detect_python", |b| {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("requirements.txt"), "pytest==7.0.0").unwrap();

        b.iter(|| black_box(ProjectDetector::new(Some(temp_dir.path())).unwrap()))
    });
}

fn bench_detect_unknown(c: &mut Criterion) {
    c.bench_function("detect_unknown", |b| {
        let temp_dir = TempDir::new().unwrap();

        b.iter(|| black_box(ProjectDetector::new(Some(temp_dir.path())).unwrap()))
    });
}

fn bench_project_type_display(c: &mut Criterion) {
    let project_type = ProjectType::Rust;

    c.bench_function("project_type_display", |b| {
        b.iter(|| black_box(format!("{:?}", project_type)))
    });
}

criterion_group!(
    benches,
    bench_detect_nodejs,
    bench_detect_rust,
    bench_detect_python,
    bench_detect_unknown,
    bench_project_type_display
);
criterion_main!(benches);
