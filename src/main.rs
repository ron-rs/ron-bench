use std::fs::read_to_string;
use criterion::{criterion_group, Criterion, BenchmarkId, PlottingBackend};

fn ron_de_legacy(content: &str) -> impl Sized {
    ron::from_str::<ron::Value>(content)
}

fn ron_de_reboot(content: &str) -> impl Sized {
    ron_reboot::from_str::<ron::Value>(content)
}

fn bench_serde_de(c: &mut Criterion) {
    let mut group = c.benchmark_group("Serde Deserialization");
    for i in ["data/canada.ron"].iter() {
        let content = read_to_string(i).unwrap();
        group.bench_with_input(BenchmarkId::new("ron", i), &content,
                               |b, i| b.iter(|| ron_de_legacy(i)));
        group.bench_with_input(BenchmarkId::new("ron-reboot", i), &content,
                               |b, i| b.iter(|| ron_de_reboot(i)));
    }
    group.finish();
}

criterion_group!(benches, bench_serde_de);
//criterion_main!(benches);

fn main() {
    benches();

    Criterion::default().configure_from_args().plotting_backend(PlottingBackend::Plotters).final_summary();
}
