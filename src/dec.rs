//! Generate pi in decimal form
//!
//! # References
//!
//! https://mathworld.wolfram.com/Digit-ExtractionAlgorithm.html
//!

// pub fn bernoulli() {}

// use rug::Rational;

use rug::{Float, Integer};

pub fn binary_split_iterative(a: u32, b: u32) -> (Integer, Integer, Integer) {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum Kind {
        Before(u32, u32),
        After(u32, u32),
    }
    use Kind::*;

    let mut vec = vec![Before(a, b)];
    let mut calcs = Vec::new();

    while let Some(kind) = vec.pop() {
        let (a, b) = match kind {
            Before(a, b) | After(a, b) => (a, b),
        };

        if b == a + 1 {
            let a = a as i128;
            let pab = Integer::from(-(6 * a - 5) * (2 * a - 1) * (6 * a - 1));
            let qab = Integer::from(10_939_058_860_032_000i64) * a.pow(3);
            let rab = pab.clone() * (545140134 * a + 13591409);
            calcs.push((pab, qab, rab));
        } else if matches!(kind, After(..)) {
            let (pmb, qmb, rmb) = calcs.pop().unwrap();
            let (pam, qam, ram) = calcs.pop().unwrap();
            let pab = &pam * pmb;
            let qab = qam * &qmb;
            let rab = qmb * ram + pam * rmb;
            calcs.push((pab, qab, rab));
        } else {
            let m = (a + b) / 2;
            vec.push(After(a, b));
            vec.push(Before(m, b));
            vec.push(Before(a, m));
        }
    }
    calcs.remove(0)
}

pub fn binary_split_parallel(a: i128, b: i128) -> (Integer, Integer, Integer) {
    let (pab, qab, rab);
    if b == a + 1 {
        // With i128::MAX a's maximum is ~1.385 trillion
        pab = Integer::from(-(6 * a - 5) * (2 * a - 1) * (6 * a - 1));
        qab = Integer::from(10_939_058_860_032_000i64) * a.pow(3);
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

pub fn binary_split(a: i128, b: i128) -> (Integer, Integer, Integer) {
    let (pab, qab, rab);
    if b == a + 1 {
        // With i128::MAX a's maximum is ~1.385 trillion
        pab = Integer::from(-(6 * a - 5) * (2 * a - 1) * (6 * a - 1));
        qab = Integer::from(10_939_058_860_032_000i64) * a.pow(3);
        rab = pab.clone() * (545140134 * a + 13591409);
    } else {
        let m = (a + b) / 2;
        let (pam, qam, ram) = binary_split(a, m);
        let (pmb, qmb, rmb) = binary_split(m, b);
        pab = &pam * pmb;
        qab = qam * &qmb;
        rab = qmb * ram + pam * rmb;
    }
    (pab, qab, rab)
}

pub fn chudnovsky_iterative(n: u32) -> Float {
    macro_rules! float {
        ($val:expr) => {
            Float::with_val(n * 14 * 4, $val)
        };
    }
    assert!(n >= 2, "n >= 2 only");
    let (_p1n, q1n, r1n) = binary_split_iterative(1, n);
    (float!(426880) * float!(10005).sqrt() * q1n.clone()) / (Integer::from(13591409) * q1n + r1n)
}

pub fn chudnovsky(n: u32) -> Float {
    macro_rules! float {
        ($val:expr) => {
            Float::with_val(n * 14 * 4, $val)
        };
    }
    assert!(n >= 2, "n >= 2 only");
    let (_p1n, q1n, r1n) = binary_split_parallel(1, n as i128);
    (426880 * float!(10005).sqrt() * &q1n) / (13591409 * q1n + r1n)
}

pub fn chudnovsky_parallel(n: u32) -> Float {
    macro_rules! float {
        ($val:expr) => {
            Float::with_val(n * 14 * 4, $val)
        };
    }
    assert!(n >= 2, "n >= 2 only");
    let (_p1n, q1n, r1n) = binary_split_parallel(1, n as i128);
    let q2n = q1n.clone();
    let (numer, denom) = rayon::join(
        || (426880 * float!(10005).sqrt() * q2n),
        || (13591409 * q1n + r1n),
    );
    numer / denom
}

#[cfg(test)]
mod tests {
    use crate::{chudnovsky, chudnovsky_iterative};

    #[test]
    fn one_million() {
        let control = &include_str!("../pi.txt")[..=1_000_001]; // cutoff non digit chars
        let test = chudnovsky(72_000).to_string(); // 72_000 * 14 > 1,000,000
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
    fn one_million_iterative() {
        let control = &include_str!("../pi.txt")[..=1_000_001]; // cutoff non digit chars
        let test = chudnovsky_iterative(72_000).to_string(); // 72_000 * 14 > 1,000,000
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
    fn iterative_vs_recursive() {
        for (n, test, control) in (3..=1_000).map(|n| (n, chudnovsky(n), chudnovsky_iterative(n))) {
            assert!(test == control, "{n}\n{test:#?}\n{control:#?}")
        }
    }
}
