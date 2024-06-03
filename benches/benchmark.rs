use criterion::{criterion_group, criterion_main, Criterion};
use merkle_tree::root;

fn criterion_benchmark(c: &mut Criterion) {
    let fixture = "fixtures/input-perf.txt".to_string();
    c.bench_function("input-perf.txt", |b| b.iter(|| root(&fixture)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
