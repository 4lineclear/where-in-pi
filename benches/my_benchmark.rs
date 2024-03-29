use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    // pi_benchmark(c);
    // split_benchmark(c);
    // split_vs_full(c);
    // pi_parallel(c);

    let mut g = c.benchmark_group("Large pi");
    g.sample_size(10);
    g.bench_function("par", |b| {
        b.iter(|| black_box(where_in_pi::chudnovsky_parallel(1_000_000)))
    });
    g.bench_function("not", |b| {
        b.iter(|| black_box(where_in_pi::chudnovsky(1_000_000)))
    });
}

pub fn split_benchmark(c: &mut Criterion) {
    let mut split = c.benchmark_group("split");

    let norm = |n| black_box(where_in_pi::binary_split(1, n));
    let para = |n| black_box(where_in_pi::binary_split_parallel(1, n));
    let iter = |n| black_box(where_in_pi::binary_split_iterative(1, n as u32));

    let mut n = 3;
    while n <= 300_000 {
        split.bench_function(format!("{n} norm"), |b| b.iter(|| norm(n)));
        split.bench_function(format!("{n} para"), |b| b.iter(|| para(n)));
        split.bench_function(format!("{n} iter"), |b| b.iter(|| iter(n)));
        n *= 10;
    }
}

pub fn pi_benchmark(c: &mut Criterion) {
    let mut pi = c.benchmark_group("pi");

    let norm = |n| black_box(where_in_pi::chudnovsky(black_box(n)));
    let iter = |n| black_box(where_in_pi::chudnovsky_iterative(black_box(n)));

    let mut n = 3;
    while n <= 300_000 {
        pi.bench_function(format!("{n} norm"), |b| b.iter(|| norm(n)));
        pi.bench_function(format!("{n} iter"), |b| b.iter(|| iter(n)));
        n *= 10;
    }
}

pub fn pi_parallel(c: &mut Criterion) {
    let mut split = c.benchmark_group("par vs not");
    let mut n = 3;
    while n <= 300_000 {
        split.bench_function(format!("{n} par"), |b| {
            b.iter(|| black_box(where_in_pi::chudnovsky_parallel(n)))
        });
        split.bench_function(format!("{n} not"), |b| {
            b.iter(|| black_box(where_in_pi::chudnovsky(n)))
        });
        n *= 10;
    }
}

pub fn split_vs_full(c: &mut Criterion) {
    let mut split = c.benchmark_group("split vs full");
    let mut n = 3;
    while n <= 300_000 {
        split.bench_function(format!("{n} split"), |b| {
            b.iter(|| black_box(where_in_pi::binary_split_parallel(1, n as i128)))
        });
        split.bench_function(format!("{n} full"), |b| {
            b.iter(|| black_box(where_in_pi::chudnovsky(n)))
        });
        n *= 10;
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
