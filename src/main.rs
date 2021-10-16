#![allow(dead_code)]

use criterion::{criterion_group, BenchmarkId, Criterion, PlottingBackend};
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

fn json_de(content: &str) -> impl Sized {
    serde_json::from_str::<ron::Value>(content)
}

fn ron_de_legacy(content: &str) -> impl Sized {
    ron::from_str::<ron::Value>(content)
}

fn ron_de_reboot(content: &str) -> impl Sized {
    ron_reboot::from_str::<ron::Value>(content)
}

fn with_extension(path: impl AsRef<Path>, ext: &str) -> PathBuf {
    path.as_ref().with_extension(ext)
}

fn bench_serde_de(c: &mut Criterion) {
    let mut group = c.benchmark_group("Serde Deserialization");
    for file in ["data/canada"].iter() {
        let content_json = read_to_string(&with_extension(file, "json")).unwrap();
        let content = read_to_string(&with_extension(file, "json")).unwrap();
        group.bench_with_input(BenchmarkId::new("json", file), &content_json, |b, i| {
            b.iter(|| json_de(i))
        });
        group.bench_with_input(BenchmarkId::new("ron", file), &content, |b, i| {
            b.iter(|| ron_de_legacy(i))
        });
        /*
        TODO: infinite loop bug...
        group.bench_with_input(BenchmarkId::new("ron-reboot", i), &content, |b, i| {
            b.iter(|| ron_de_reboot(i))
        });
         */
    }
    group.finish();
}

criterion_group!(benches, bench_serde_de);
//criterion_main!(benches);

fn main() {
    benches();

    Criterion::default()
        .configure_from_args()
        .plotting_backend(PlottingBackend::Plotters)
        .final_summary();
}
