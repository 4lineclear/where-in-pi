use criterion::{black_box, criterion_group, criterion_main, Criterion};
// use where_in_pi::{
//     dec::{bernoulli, bernoulli_naive, Bernoulli},
//     pi_hex,
// };

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("pi-hex");
    let d = 1_000_000;
    let precision = 24;
    group.bench_function("normal", |b| {
        b.iter(|| black_box(where_in_pi::pi_hex(d, precision)))
    });
    group.bench_function("condensed", |b| {
        b.iter(|| black_box(where_in_pi::pi_hex_alt(d, precision)))
    });
    // c.bench_function("pi_hex", |b| {
    //     b.iter(|| black_box(pi_hex(black_box(1_000_000), black_box(24))))
    // });
    // c.bench_function("bernoulli iter", |b| {
    //     b.iter(|| {
    //         let context = Bernoulli::default();
    //         let res = context.take(30 + 1).collect::<Vec<_>>();
    //         black_box(res);
    //     })
    // });
    // c.bench_function("bernoulli", |b| {
    //     b.iter(|| {
    //         let mut context = Bernoulli::new(30 + 1);
    //         let mut res = vec![];
    //         for n in 0..30 + 1 {
    //             let b = bernoulli(n * 2, &mut context);
    //             res.push(b);
    //         }
    //         black_box(res);
    //     });
    // });
    // c.bench_function("bernoulli naive", |b| {
    //     b.iter(|| {
    //         let mut res = vec![];
    //         for n in 0..30 + 1 {
    //             let b = bernoulli_naive(n * 2);
    //             res.push(b);
    //         }
    //         black_box(res);
    //     });
    // });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
