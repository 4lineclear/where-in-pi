use criterion::{black_box, criterion_group, criterion_main, Criterion};
use where_in_pi::pi_hex;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("pi_hex", |b| {
        b.iter(|| black_box(pi_hex(black_box(1_000_000), black_box(24))))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
