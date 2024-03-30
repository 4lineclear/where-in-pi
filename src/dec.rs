//! Generate pi in decimal form
//!
//! # References
//!
//! https://mathworld.wolfram.com/Digit-ExtractionAlgorithm.html
//!

use std::sync::Arc;

use dashmap::DashMap;
use rug::{ops::Pow, Float, Integer};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContextKey {
    /// Recursive, non bottom level case
    Norm(i128, i128),
    /// Base case
    Base(i128),
}

pub type ContextValue = (Integer, Integer, Integer);

pub type Context = DashMap<ContextKey, ContextValue>;

pub fn split_context(a: i128, b: i128, context: Arc<Context>) -> (Integer, Integer, Integer) {
    let (pab, qab, rab);
    if b == a + 1 {
        if let Some(r) = context.get(&ContextKey::Base(a)) {
            let (pab, qab, rab) = r.clone();
            return (pab, qab, rab);
        }
        // With i128::MAX a's maximum is ~1.385 trillion
        pab = int(-(6 * a - 5) * (2 * a - 1) * (6 * a - 1));
        qab = int(10_939_058_860_032_000i64) * a.pow(3);
        rab = pab.clone() * (545140134 * a + 13591409);
        context.insert(ContextKey::Base(a), (pab.clone(), qab.clone(), rab.clone()));
    } else {
        if let Some(r) = context.get(&ContextKey::Norm(a, b)) {
            let (pab, qab, rab) = r.clone();
            return (pab, qab, rab);
        }
        let m = (a + b) / 2;
        let ((pam, qam, ram), (pmb, qmb, rmb)) = rayon::join(
            || split_context(a, m, context.clone()),
            || split_context(m, b, context.clone()),
        );
        pab = &pam * pmb;
        qab = qam * &qmb;
        rab = qmb * ram + pam * rmb;
        context.insert(
            ContextKey::Norm(a, b),
            (pab.clone(), qab.clone(), rab.clone()),
        );
    }
    (pab, qab, rab)
}

pub fn binary_split(a: i128, b: i128) -> (Integer, Integer, Integer) {
    let (pab, qab, rab);
    if b == a + 1 {
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

pub fn chudnovsky_float(n: u32) -> Float {
    // NOTE:
    // consider returning an integer shifted left 14n * 4 times
    // also maybe lower from 4 to log_2(10)
    // set precision of log_2(10) = 14n
    assert!(n >= 2, "n >= 2 only");
    let (root, (_p1n, q1n, r1n)) = rayon::join(
        || Float::with_val(n * 14 * 4, 10005).sqrt(),
        || binary_split(1, n as i128),
    );
    (426880 * root * &q1n) / (13591409 * q1n + r1n)
}

/// Chudnovsky algorith returning an integer
pub fn chudnovsky_integer(n: u32) -> Integer {
    assert!(n >= 2, "n >= 2 only");
    // root = √(10005 * 10^2(n * 14 + 1) )
    let (root, (_p1n, q1n, r1n)) = rayon::join(
        || (int(10).pow(2 * (n * 14 + 1)) * 10005u32).sqrt(),
        || binary_split(1, n as i128),
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
    use std::sync::Arc;

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
        let context = Arc::new(Context::new());
        (3..=100)
            .map(|n| (binary_split(1, n), split_context(1, n, context.clone())))
            .for_each(|(control, test)| assert_eq!(control, test));
    }
}
