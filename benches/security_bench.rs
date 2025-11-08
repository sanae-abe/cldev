use cldev::core::security::SecurePath;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tempfile::TempDir;

fn bench_secure_path_new(c: &mut Criterion) {
    c.bench_function("secure_path_new", |b| {
        let temp_dir = TempDir::new().unwrap();
        let test_path = temp_dir.path().to_path_buf();

        b.iter(|| black_box(SecurePath::new(test_path.clone()).unwrap()))
    });
}

fn bench_secure_path_with_subdirs(c: &mut Criterion) {
    c.bench_function("secure_path_with_subdirs", |b| {
        let temp_dir = TempDir::new().unwrap();
        let subdir = temp_dir.path().join("subdir");
        std::fs::create_dir(&subdir).unwrap();

        b.iter(|| black_box(SecurePath::new(subdir.clone()).unwrap()))
    });
}

fn bench_secure_path_clone(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    let secure_path = SecurePath::new(temp_dir.path().to_path_buf()).unwrap();

    c.bench_function("secure_path_clone", |b| {
        b.iter(|| black_box(secure_path.clone()))
    });
}

criterion_group!(
    benches,
    bench_secure_path_new,
    bench_secure_path_with_subdirs,
    bench_secure_path_clone
);
criterion_main!(benches);
