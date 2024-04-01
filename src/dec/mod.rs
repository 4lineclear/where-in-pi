//! Generate pi in decimal form
//!
//! # References
//!
//! https://mathworld.wolfram.com/Digit-ExtractionAlgorithm.html
//!

use std::num::NonZeroUsize;

use dashmap::DashMap;
use indicatif::ParallelProgressIterator;
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelExtend, ParallelIterator};
use rug::{ops::Pow, Float, Integer};

#[cfg(test)]
mod tests;

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

pub fn gen_splits_v4(a: u32, b: u32, splits: &DashMap<ContextKey, u32>) {
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

pub fn gen_splits_v3(a: u32, b: u32, splits: &DashMap<ContextKey, u32>) {
    let key = (a, b);

    if let Some(mut v) = splits.get_mut(&key) {
        *v += 1;
        return;
    }

    if b != a + 1 {
        let m = (a + b) / 2;
        rayon::join(
            || gen_splits_v3(a, m, splits),
            || gen_splits_v3(m, b, splits),
        );
    }

    splits.insert(key, 1);
}

pub fn gen_splits_v2(a: u32, b: u32) -> dashmap::DashSet<(u32, u32)> {
    fn inner(a: u32, b: u32, splits: &dashmap::DashSet<(u32, u32)>) {
        if splits.insert((a, b)) {
            return;
        }
        if b != a + 1 {
            let m = (a + b) / 2;
            rayon::join(|| inner(a, m, splits), || inner(m, b, splits));
        }
    }
    let mut splits = dashmap::DashSet::with_capacity(b as usize * 2 - 3);
    inner(a, b, &mut splits);
    splits
}

// TODO: Create custom parallel impl

pub fn deduce_splits_v5(start: u32, end: u32, step: u32) -> DashMap<(u32, u32), u32> {
    let Ok(threads) = std::thread::available_parallelism().map(usize::from) else {
        panic!("Unable to get available threads")
    };

    let splits = DashMap::new();
    let mut b = (start..end).step_by(step as usize);
    let (tx, rx) = std::sync::mpsc::channel::<()>();
    (0..threads).try_for_each(|_| tx.send(())).unwrap();

    std::thread::scope(|s| {
        while let Ok(_) = rx.recv() {
            let Some(b) = b.next() else {
                break;
            };
            let splits = std::sync::Arc::new(&splits);
            let tx = tx.clone();
            s.spawn(move || {
                gen_splits_v4(1, b, &splits);
                tx.send(()).unwrap();
            });
        }
    });
    splits
}

pub fn deduce_splits_v4(
    start: u32,
    end: u32,
    step: u32,
    progress: bool,
) -> DashMap<(u32, u32), u32> {
    let splits = DashMap::new();
    if progress {
        (start..=end)
            // .step_by(step as usize)
            // .rev()
            // .progress_count(((end - start) / step) as u64)
            .step_by(step as usize)
            .par_bridge()
            .progress_count(((end - start) / step) as u64)
            // .par_bridge()
            .for_each(|b| gen_splits_v4(1, b, &splits));
    } else {
        (start..=end)
            .step_by(step as usize)
            .par_bridge()
            .for_each(|b| gen_splits_v4(1, b, &splits));
    }
    splits
}

pub fn deduce_splits_v3(
    start: u32,
    end: u32,
    step: u32,
    progress: bool,
) -> DashMap<(u32, u32), u32> {
    let splits = DashMap::new();
    if progress {
        (start..=end)
            // .step_by(step as usize)
            // .rev()
            // .progress_count(((end - start) / step) as u64)
            .step_by(step as usize)
            .par_bridge()
            .progress_count(((end - start) / step) as u64)
            // .par_bridge()
            .for_each(|b| gen_splits_v3(1, b, &splits));
    } else {
        (start..=end)
            .step_by(step as usize)
            .par_bridge()
            .for_each(|b| gen_splits_v3(1, b, &splits));
    }
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
