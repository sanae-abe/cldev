use cldev::core::Config;
use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use tempfile::TempDir;

fn bench_config_new(c: &mut Criterion) {
    c.bench_function("config_new", |b| b.iter(|| black_box(Config::default())));
}

fn bench_config_load(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.toml");

    // Create initial config
    let config = Config::default();
    config.save(Some(config_path.clone())).unwrap();

    c.bench_function("config_load", |b| {
        b.iter(|| black_box(Config::load(Some(config_path.clone())).unwrap()))
    });
}

fn bench_config_save(c: &mut Criterion) {
    c.bench_function("config_save", |b| {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.toml");
        let config = Config::default();

        b.iter(|| {
            let _: () = config.save(Some(config_path.clone())).unwrap();
            black_box(())
        })
    });
}

fn bench_config_default_path(c: &mut Criterion) {
    c.bench_function("config_default_path", |b| {
        b.iter(|| black_box(Config::default_path().unwrap()))
    });
}

fn bench_config_clone(c: &mut Criterion) {
    let config = Config::default();

    c.bench_function("config_clone", |b| b.iter(|| black_box(config.clone())));
}

criterion_group!(
    benches,
    bench_config_new,
    bench_config_load,
    bench_config_save,
    bench_config_default_path,
    bench_config_clone
);
criterion_main!(benches);
