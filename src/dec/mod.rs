//! Generate pi in decimal form
//!
//! # References
//!
//! https://mathworld.wolfram.com/Digit-ExtractionAlgorithm.html
//!

use dashmap::DashMap;
use indicatif::ParallelProgressIterator;
use rayon::iter::{ParallelBridge, ParallelIterator};
use rug::{ops::Pow, Float, Integer};

#[cfg(test)]
mod tests;

pub type Split = (u32, u32);

pub type PQR = (Integer, Integer, Integer);

pub fn split_context<RS: std::hash::BuildHasher + Clone + Send + Sync>(
    a: u32,
    b: u32,
    context: &DashMap<Split, PQR, RS>,
) -> PQR {
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

pub fn binary_split(a: u32, b: u32) -> PQR {
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

// TODO: create testing for gen_splits & deduce_splits

pub fn gen_splits(a: u32, b: u32, splits: &DashMap<Split, u32>) {
    let mut stack = vec![(a, b)];
    while let Some(k) = stack.pop() {
        if let Some(mut split) = splits.get_mut(&k) {
            *split += 1;
            continue;
        }
        splits.insert(k, 1);

        let (a, b) = k;
        if b != a + 1 {
            let m = (a + b) / 2;
            stack.push((a, m));
            stack.push((m, b));
        }
    }
}

pub fn deduce_splits(start: u32, end: u32, step: u32, progress: bool) -> DashMap<Split, u32> {
    let splits = DashMap::new();
    if progress {
        (start..=end)
            .step_by(step as usize)
            .par_bridge()
            .progress_count(((end - start) / step) as u64)
            .for_each(|b| gen_splits(1, b, &splits));
    } else {
        (start..=end)
            .step_by(step as usize)
            .par_bridge()
            .for_each(|b| gen_splits(1, b, &splits));
    }
    splits
}

pub fn chudnovsky_float(n: u32) -> Float {
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
