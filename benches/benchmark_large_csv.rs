use criterion::{black_box, criterion_group, criterion_main, Criterion};
use distributing_iterator::distribute_csv;

pub fn criterion_benchmark(c: &mut Criterion) {
    let path = "/Users/andrii/dev/fetlife/data-analysis/ranking_content/pre-distribute.csv";
    let data = std::fs::read_to_string(path).unwrap();
    c.bench_function("distribute-large-csv", |b| {
        b.iter(|| distribute_csv(black_box(&data.clone()), "user_id", 400))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
