use cldev::core::i18n::{I18n, Language};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_i18n_new(c: &mut Criterion) {
    c.bench_function("i18n_new", |b| b.iter(|| black_box(I18n::new())));
}

fn bench_i18n_with_language(c: &mut Criterion) {
    c.bench_function("i18n_with_language", |b| {
        b.iter(|| black_box(I18n::with_language(Language::Japanese)))
    });
}

fn bench_i18n_get_message(c: &mut Criterion) {
    let i18n = I18n::new();

    c.bench_function("i18n_get_message", |b| {
        b.iter(|| black_box(i18n.get("error.config")))
    });
}

fn bench_i18n_language_detect(c: &mut Criterion) {
    c.bench_function("i18n_language_detect", |b| {
        b.iter(|| black_box(Language::detect()))
    });
}

fn bench_i18n_language_code(c: &mut Criterion) {
    let lang = Language::Japanese;

    c.bench_function("i18n_language_code", |b| b.iter(|| black_box(lang.code())));
}

fn bench_i18n_language_from_code(c: &mut Criterion) {
    c.bench_function("i18n_language_from_code", |b| {
        b.iter(|| black_box(Language::from_code("ja")))
    });
}

criterion_group!(
    benches,
    bench_i18n_new,
    bench_i18n_with_language,
    bench_i18n_get_message,
    bench_i18n_language_detect,
    bench_i18n_language_code,
    bench_i18n_language_from_code
);
criterion_main!(benches);
