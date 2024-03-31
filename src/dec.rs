//! Generate pi in decimal form
//!
//! # References
//!
//! https://mathworld.wolfram.com/Digit-ExtractionAlgorithm.html
//!

use ahash::HashMap;
use dashmap::DashMap;
use rug::{ops::Pow, Float, Integer};

pub type ContextKey = (u32, u32);

pub type ContextValue = (Integer, Integer, Integer);

pub type Context = DashMap<ContextKey, ContextValue, ahash::RandomState>;

#[derive(Debug, Clone)]
pub struct IContext {
    pub inner: std::collections::HashMap<ContextKey, ContextValue>,
    pub write_buffer: Vec<(ContextKey, ContextValue)>,
}

#[derive(Debug, Clone, Default)]
pub struct VContext {
    inner: Vec<Vec<ContextValue>>,
}

impl VContext {
    #[inline]
    fn grow(&mut self, b: u32) {
        let size = (b as usize * 2 - 3).saturating_sub(self.inner.len());
        self.inner.extend(std::iter::repeat(Vec::new()).take(size));
    }
    #[inline]
    fn grow_at(&mut self, a: u32, b: u32) {
        self.grow(b);
        self.inner[b as usize - 1].extend(
            std::iter::repeat((Integer::new(), Integer::new(), Integer::new())).take(a as usize),
        );
    }
    pub fn get(&self, a: u32, b: u32) -> &ContextValue {
        &self.inner[a as usize][b as usize]
    }
    pub fn insert(&mut self, a: u32, b: u32, value: ContextValue) {
        self.grow_at(a, b);
        self.inner[b as usize - 1][a as usize - 1] = value;
    }
}

pub fn split_v2(a: u32, b: u32, context: &mut HashMap<ContextKey, ContextValue>) {
    fn inner(
        a: u32,
        b: u32,
        context: &HashMap<ContextKey, ContextValue>,
        write_buffer: &boxcar::Vec<(ContextKey, ContextValue)>,
    ) {
        let key = (a, b);
        if context.contains_key(&key) {
            return;
        }
        let (pab, qab, rab);

        if b == a + 1 {
            let a = a as i128;
            // With i128::MAX a's maximum is ~1.385 trillion
            pab = int(-(6 * a - 5) * (2 * a - 1) * (6 * a - 1));
            qab = int(10_939_058_860_032_000i64) * a.pow(3);
            rab = pab.clone() * (545140134 * a + 13591409);
        } else {
            let m = (a + b) / 2;
            rayon::join(
                || inner(a, m, context, write_buffer),
                || inner(m, b, context, write_buffer),
            );
            let (pam, qam, ram) = &*context.get(&(a, m)).unwrap();
            let (pmb, qmb, rmb) = &*context.get(&(m, b)).unwrap();

            pab = pam.clone() * pmb;
            qab = qam.clone() * qmb;
            rab = qmb.clone() * ram + pam * rmb;
        }
        write_buffer.push(((a, b), (pab, qab, rab)));
    }
    let buf = boxcar::Vec::new();
    inner(a, b, context, &buf);
    context.extend(buf.into_iter());
}

pub fn split_indexed<'a>(a: u32, b: u32, context: &'a Context) {
    let key = (a, b);
    if context.contains_key(&key) {
        return;
    }
    let (pab, qab, rab);

    if b == a + 1 {
        let a = a as i128;
        // With i128::MAX a's maximum is ~1.385 trillion
        pab = int(-(6 * a - 5) * (2 * a - 1) * (6 * a - 1));
        qab = int(10_939_058_860_032_000i64) * a.pow(3);
        rab = pab.clone() * (545140134 * a + 13591409);
    } else {
        let m = (a + b) / 2;
        rayon::join(
            || split_indexed(a, m, context),
            || split_indexed(m, b, context),
        );
        let (pam, qam, ram) = &*context.get(&(a, m)).unwrap();
        let (pmb, qmb, rmb) = &*context.get(&(m, b)).unwrap();

        pab = pam.clone() * pmb;
        qab = qam.clone() * qmb;
        rab = qmb.clone() * ram + pam * rmb;
    }
    context.insert(key, (pab, qab, rab));
}

pub fn split_context(a: u32, b: u32, context: &Context) -> (Integer, Integer, Integer) {
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
    use std::sync::Arc;

    use crate::{
        binary_split, chudnovsky_float, chudnovsky_integer, split_context, split_indexed, Context,
    };

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
        let context = Arc::new(Context::default());
        (3..=100)
            .map(|n| (binary_split(1, n), split_context(1, n, &context)))
            .for_each(|(control, test)| assert_eq!(control, test));
    }
    #[test]
    fn context_index() {
        let context = Context::default();
        (3..=100)
            .map(|n| {
                (binary_split(1, n), {
                    split_indexed(1, n, &context);
                    (1, n)
                })
            })
            .for_each(|(control, test)| assert_eq!(&control, &*context.get(&test).unwrap()));
    }
}
