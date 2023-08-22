use criterion::{criterion_group, criterion_main, Criterion};
use rscalc::lev;

fn lev_benchmark(c: &mut Criterion) {
    c.bench_function("lev", |b| b.iter(|| lev("levenshtein", "lefenshdeen")));
}

criterion_group!(benches, lev_benchmark);
criterion_main!(benches);
