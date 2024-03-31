//! Generate pi in decimal form
//!
//! # References
//!
//! https://mathworld.wolfram.com/Digit-ExtractionAlgorithm.html
//!

use dashmap::DashMap;
use rug::{ops::Pow, Float, Integer};

pub type ContextKey = (u32, u32);

pub type ContextValue = (Integer, Integer, Integer);

pub type Context<RS> = DashMap<ContextKey, ContextValue, RS>;

pub fn split_context<RS: std::hash::BuildHasher + Clone + Send + Sync>(
    a: u32,
    b: u32,
    context: &Context<RS>,
) -> (Integer, Integer, Integer) {
    let (pab, qab, rab);
    let key = (a, b);
    if let Some(r) = context.get(&key) {
        return r.clone();
    }
    if b == a + 1 {
        let a = a as i128;
        // With i128::MAX a's maximum is ~1.385 trillion
        pab = int(-(6 * a - 5) * (2 * a - 1) * (6 * a - 1));
        qab = int(10_939_058_860_032_000i64) * a.pow(3);
        rab = pab.clone() * (545140134 * a + 13591409);
    } else {
        let m = (a + b) / 2;
        let ((pam, qam, ram), (pmb, qmb, rmb)) = rayon::join(
            || split_context(a, m, &context),
            || split_context(m, b, &context),
        );
        pab = &pam * pmb;
        qab = qam * &qmb;
        rab = qmb * ram + pam * rmb;
    }
    context.insert(key, (pab.clone(), qab.clone(), rab.clone()));
    (pab, qab, rab)
}

pub fn binary_split(a: u32, b: u32) -> (Integer, Integer, Integer) {
    let (pab, qab, rab);
    if b == a + 1 {
        let a = a as i128;
        // With i128::MAX a's maximum is ~1.385 trillion
        pab = int(-(6 * a - 5) * (2 * a - 1) * (6 * a - 1));
        qab = int(10_939_058_860_032_000i64) * a.pow(3);
        rab = pab.clone() * (545140134 * a + 13591409);
    } else {
        let m = (a + b) / 2;
        let ((pam, qam, ram), (pmb, qmb, rmb)) =
            rayon::join(|| binary_split(a, m), || binary_split(m, b));
        pab = &pam * pmb;
        qab = qam * &qmb;
        rab = qmb * ram + pam * rmb;
    }
    (pab, qab, rab)
}

pub fn split_empty(a: u32, b: u32) -> Vec<(u32, u32)> {
    fn inner(a: u32, b: u32, splits: &mut Vec<(u32, u32)>) {
        splits.push((a, b));
        if b != a + 1 {
            let m = (a + b) / 2;
            inner(a, m, splits);
            inner(m, b, splits);
        }
    }
    let mut splits = Vec::with_capacity(b as usize * 2 - 3);
    inner(a, b, &mut splits);
    splits
}

pub fn chudnovsky_float(n: u32) -> Float {
    // NOTE:
    // consider returning an integer shifted left 14n * 4 times
    // also maybe lower from 4 to log_2(10)
    // set precision of log_2(10) = 14n
    assert!(n >= 2, "n >= 2 only");
    let (root, (_p1n, q1n, r1n)) = rayon::join(
        || Float::with_val(n * 14 * 4, 10005).sqrt(),
        || binary_split(1, n),
    );
    (426880 * root * &q1n) / (13591409 * q1n + r1n)
}

/// Chudnovsky algorith returning an integer
pub fn chudnovsky_integer(n: u32) -> Integer {
    assert!(n >= 2, "n >= 2 only");
    // root = √(10005 * 10^2(n * 14 + 1) )
    let (root, (_p1n, q1n, r1n)) = rayon::join(
        || (int(10).pow(2 * (n * 14 + 1)) * 10005u32).sqrt(),
        || binary_split(1, n),
    );
    (426880 * root * &q1n) / (13591409 * q1n + r1n)
}

/// Shorthand for [`Integer::from`]
#[inline(always)]
pub fn int(int: impl Into<Integer>) -> Integer {
    int.into()
}

#[cfg(test)]
mod tests {
    use crate::{binary_split, chudnovsky_float, chudnovsky_integer, split_context, Context};

    #[test]
    fn one_million() {
        let control = &include_str!("../pi.txt")[..=1_000_001]; // cutoff non digit chars
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
        let control = &include_str!("../pi.txt")[2..=1_000_001]; // cutoff non digit chars
        let test = chudnovsky_integer(72_000).to_string(); // 72_000 * 14 > 1,000,000
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
}
