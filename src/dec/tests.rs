use dashmap::DashMap;

use crate::{binary_split, chudnovsky_float, chudnovsky_integer, split_context};

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
    let context = DashMap::new();
    (3..=100)
        .map(|n| (binary_split(1, n), split_context(1, n, &context)))
        .for_each(|(control, test)| assert_eq!(control, test));
}
