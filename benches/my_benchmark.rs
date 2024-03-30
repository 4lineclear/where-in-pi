use std::{collections::HashMap, sync::Arc};

use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    // float_vs_integer(c);
    // standard(c);
    // split_ctx_vs_not(c);
    split_par_vs_not(c);
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

    // let n = 3_000_000;
    // let mut b = c.benchmark_group(format!("float vs integer {n}"));
    // b.sample_size(10);
    // b.bench_function(format!("integer"), |b| b.iter(|| integer(n)));
    // b.bench_function(format!("float"), |b| b.iter(|| float(n)));

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
        let context_par = Arc::new(where_in_pi::Context::new());
        b.iter(|| {
            for i in (3..3000).step_by(3) {
                black_box(where_in_pi::split_context(1, i, context_par.clone()));
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
    let series = [72_000, 144_000, 288_000];

    let mut b = c.benchmark_group("split test");
    b.bench_function("par", |b| {
        let context_par = Arc::new(where_in_pi::Context::new());
        b.iter(|| {
            series.iter().for_each(|&n| {
                black_box(where_in_pi::split_context(
                    1,
                    black_box(n),
                    context_par.clone(),
                ));
            });
        });
    });
    b.bench_function("par clean", |b| {
        b.iter(|| {
            let context_par = Arc::new(where_in_pi::Context::new());
            series.iter().for_each(|&n| {
                black_box(where_in_pi::split_context(
                    1,
                    black_box(n),
                    context_par.clone(),
                ));
            });
        });
    });
    b.bench_function("norm", |b| {
        b.iter(|| {
            series.iter().for_each(|&n| {
                black_box(where_in_pi::binary_split(1, black_box(n)));
            });
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
