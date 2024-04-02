use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dashmap::DashMap;

pub fn criterion_benchmark(c: &mut Criterion) {
    // float_vs_integer(c);
    // standard(c);
    // split_ctx_vs_not(c);
    // split_par_vs_not(c);
    // gen_splits(c);
    deduce_splits(c);
}

pub fn standard(c: &mut Criterion) {
    let mut b = c.benchmark_group("pi");

    let split = |n| black_box(where_in_pi::binary_split(1, black_box(n)));
    let pi = |n| black_box(where_in_pi::chudnovsky_float(black_box(n as u32)));

    let mut n = 3;
    while n <= 300_000 {
        b.bench_function(format!("{n} split"), |b| b.iter(|| split(n)));
        b.bench_function(format!("{n} pi"), |b| b.iter(|| pi(n)));
        n *= 10;
    }
}

pub fn float_vs_integer(c: &mut Criterion) {
    let integer = |n| black_box(where_in_pi::chudnovsky_integer(black_box(n)));
    let float = |n| black_box(where_in_pi::chudnovsky_float(black_box(n)));

    let mut n = 3;
    while n <= 300_000 {
        let mut b = c.benchmark_group(format!("float vs integer {n}"));
        b.bench_function(format!("integer"), |b| b.iter(|| integer(n)));
        b.bench_function(format!("float"), |b| b.iter(|| float(n)));
        n *= 10;
    }
}

pub fn split_ctx_vs_not(c: &mut Criterion) {
    let mut b = c.benchmark_group("ctx vs not");
    b.bench_function("ctx 3..3000", |b| {
        let context = where_in_pi::Context::new();
        b.iter(|| {
            for i in (3..3000).step_by(3) {
                black_box(where_in_pi::split_context(1, i, &context));
            }
        });
    });
    b.bench_function("not 3..3000", |b| {
        b.iter(|| {
            for i in (3..3000).step_by(3) {
                black_box(where_in_pi::binary_split(1, i));
            }
        });
    });
}

pub fn split_par_vs_not(c: &mut Criterion) {
    let series = [9_000, 18_000, 36_000, 72_000, 144_000, 288_000];

    let mut b = c.benchmark_group("split test");
    b.bench_function("ctx default", move |b| {
        let context = where_in_pi::Context::new();
        b.iter(|| {
            series.iter().for_each(|&n| {
                black_box(where_in_pi::split_context(1, black_box(n), &context));
            });
        });
    });
    b.bench_function("ctx ahash", move |b| {
        let context = where_in_pi::Context::with_hasher(ahash::RandomState::new());
        b.iter(|| {
            series.iter().for_each(|&n| {
                black_box(where_in_pi::split_context(1, black_box(n), &context));
            });
        });
    });
    b.bench_function("norm", move |b| {
        b.iter(|| {
            series.iter().for_each(|&n| {
                black_box(where_in_pi::binary_split(1, black_box(n)));
            });
        });
    });
}

pub fn gen_splits(c: &mut Criterion) {
    let mut b = c.benchmark_group("gen splits");
    let start = 10000;
    let end = 100000;
    let step = 10;

    b.bench_function("v4", move |b| {
        let splits = DashMap::new();
        b.iter(|| {
            (start..=end)
                .rev()
                .step_by(step)
                .for_each(|b| where_in_pi::gen_splits(1, b, &splits));
        });
        black_box(splits);
    });
}

pub fn deduce_splits(c: &mut Criterion) {
    let mut b = c.benchmark_group("deduce splits");
    let start = 10000;
    let end = 100000;
    let step = 1;

    b.bench_function("v4", move |b| {
        b.iter(|| black_box(where_in_pi::deduce_splits(start, end, step, false)));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
