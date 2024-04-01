use dashmap::DashMap;

use crate::{
    binary_split, chudnovsky_float, chudnovsky_integer, deduce_splits_v3, deduce_splits_v4,
    gen_splits_v3, gen_splits_v4, split_context, Context,
};

const PI_CONTROL: &str = include_str!("../../pi.txt");
#[test]
fn one_million() {
    let control = &PI_CONTROL[..=1_000_001]; // cutoff non digit chars
    let test = chudnovsky_float(72_000).to_string(); // 72_000 * 14 > 1,000,000
    control
        .chars()
        .zip(test.chars())
        .enumerate()
        .for_each(|(i, (pi, test))| {
            assert!(
                pi == test,
                "pi_n != test_n at index {i}, pi_n={pi}, test_n={test}"
            );
        });
}
#[test]
fn one_million_integer() {
    let control = &PI_CONTROL[2..=1_000_001]; // cutoff non digit chars
    let test = chudnovsky_integer(72_000).to_string(); // 72_000 * 14 > 1,000,000
                                                       // Test first digit
    assert_eq!(test.as_bytes()[0], b'3');
    // Test rest of digits
    control
        .chars()
        .zip(test[1..].chars())
        .enumerate()
        .for_each(|(i, (pi, test))| {
            assert!(
                pi == test,
                "pi_n != test_n at index {i}, pi_n={pi}, test_n={test}"
            );
        });
}
#[test]
fn context() {
    let context = Context::new();
    (3..=100)
        .map(|n| (binary_split(1, n), split_context(1, n, &context)))
        .for_each(|(control, test)| assert_eq!(control, test));
}
#[test]
fn gen_split() {
    let a = 1;
    let b = 100000;

    let splits_v3 = DashMap::new();
    let splits_v4 = DashMap::new();

    gen_splits_v3(a, b, &splits_v3);
    gen_splits_v4(a, b, &splits_v4);

    assert_eq!(
        splits_v3
            .into_iter()
            .collect::<std::collections::HashMap<_, _>>(),
        splits_v4
            .into_iter()
            .collect::<std::collections::HashMap<_, _>>(),
    );
}
#[test]
fn deduce_split() {
    let start = 10000;
    let end = 100000;
    let step = 1;

    let (splits_v3, splits_v4) = rayon::join(
        || deduce_splits_v3(start, end, step, false),
        || deduce_splits_v4(start, end, step, false),
    );

    let (mut v3_not_in_v4, mut v4_not_in_v3) = rayon::join(
        || {
            splits_v3
                .iter()
                .filter(|v3| !splits_v4.contains_key(v3.key()))
                .map(|r| (r.key().clone(), r.value().clone()))
                .peekable()
        },
        || {
            splits_v4
                .iter()
                .filter(|v4| !splits_v3.contains_key(v4.key()))
                .map(|r| (r.key().clone(), r.value().clone()))
                .peekable()
        },
    );

    if v3_not_in_v4.peek().is_some() || v4_not_in_v3.peek().is_some() {
        panic!(
            "{:?} {:?}",
            v3_not_in_v4.collect::<Vec<_>>(),
            v4_not_in_v3.collect::<Vec<_>>()
        );
    }
}
