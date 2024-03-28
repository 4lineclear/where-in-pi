use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut pi_standard = c.benchmark_group("pi standard");

    let norm = |n| black_box(where_in_pi::chudnovsky(black_box(n)));
    let iter = |n| black_box(where_in_pi::chudnovsky_iterative(black_box(n)));

    pi_standard.bench_function("3 norm", |n| n.iter(|| norm(3)));
    pi_standard.bench_function("3 iter", |n| n.iter(|| iter(3)));

    pi_standard.bench_function("30 norm", |n| n.iter(|| norm(30)));
    pi_standard.bench_function("30 iter", |n| n.iter(|| iter(30)));

    pi_standard.bench_function("300 norm", |n| n.iter(|| norm(300)));
    pi_standard.bench_function("300 iter", |n| n.iter(|| iter(300)));

    pi_standard.bench_function("3000 norm", |n| n.iter(|| norm(3000)));
    pi_standard.bench_function("3000 iter", |n| n.iter(|| iter(3000)));

    pi_standard.bench_function("30000 norm", |n| n.iter(|| norm(30000)));
    pi_standard.bench_function("30000 iter", |n| n.iter(|| iter(30000)));

    // NOTE: A bit too much for now
    pi_standard.bench_function("300_000 norm", |n| n.iter(|| norm(300_000)));
    pi_standard.bench_function("300_000 iter", |n| n.iter(|| iter(300_000)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
