//! Generate pi in decimal form
//!
//! # References
//!
//! https://mathworld.wolfram.com/Digit-ExtractionAlgorithm.html
//!

// pub fn bernoulli() {}

// use rug::Rational;

use rug::{Float, Integer, Rational};

#[derive(Debug, Default, Clone)]
pub struct Bernoulli {
    digits: Vec<Rational>,
}

impl Bernoulli {
    pub fn new(len: usize) -> Self {
        let mut this = Self {
            digits: Vec::with_capacity(len),
        };
        bernoulli(len as u64, &mut this);
        this
    }
}

impl Iterator for Bernoulli {
    type Item = Rational;

    fn next(&mut self) -> Option<Self::Item> {
        Some(bernoulli((self.digits.len() + 1) as u64 * 2, self))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.digits.len(), Some(self.digits.len()))
    }
}

impl ExactSizeIterator for Bernoulli {}

pub fn bernoulli(n: u64, b: &mut Bernoulli) -> Rational {
    // Change to do this in a single iteration
    for m in 0..=n as usize {
        if m >= b.digits.len() {
            b.digits.push(Rational::from((1, (m + 1))));
            for j in (1..=m as usize).rev() {
                b.digits[j - 1] =
                    (b.digits[j - 1].clone() - b.digits[j].clone()) * (Rational::from(j));
            }
        }
    }
    b.digits[0].clone()
}

pub fn bernoulli_naive(n: u64) -> Rational {
    let mut out = Vec::with_capacity(n as usize);

    for m in 0..=n {
        out.push(Rational::from((1, m + 1)));
        for j in (1..=m as usize).rev() {
            out[j - 1] = (out[j - 1].clone() - &out[j]) * Rational::from((j, 1));
        }
    }

    out.into_iter().next().unwrap_or_default()
}

fn binary_split(a: i64, b: i64) -> (Integer, Integer, Integer) {
    let (pab, qab, rab);
    if b == a + 1 {
        pab = Integer::from(-(6 * a - 5) * (a * 2 - 1) * (6 * a - 1));
        qab = Integer::from(10_939_058_860_032_000i64) * (a as i128).pow(3);
        rab = pab.clone() * (545140134 * a + 13591409);
    } else {
        let m = (a + b) / 2;
        let ((pam, qam, ram), (pmb, qmb, rmb)) =
            rayon::join(|| binary_split(a, m), || binary_split(m, b));

        pab = pam.clone() * pmb;
        qab = qam * qmb.clone();
        rab = qmb * ram + pam * rmb;
    }
    (pab, qab, rab)
}

pub fn chudnovsky(n: u64) -> Float {
    macro_rules! float {
        ($val:expr) => {
            Float::with_val(n as u32 * 14 * 4, $val)
        };
    }
    if n < 2 {
        return Float::with_val(n as u32 * 14 * 4, std::f64::consts::PI);
    }
    let (_p1n, q1n, r1n) = binary_split(1, n as i64);
    println!("{_p1n}, {q1n}, {r1n}");
    (float!(426880) * float!(10005).sqrt() * q1n.clone()) / (Integer::from(13591409) * q1n + r1n)
}
