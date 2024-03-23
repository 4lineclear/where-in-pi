use rand::seq::IteratorRandom;
use rand::thread_rng;

use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

use rug::ops::Pow;
use rug::Float;
use rug::Integer;

/// A component of calculating a digit of pi
pub fn series(d: i64, j: u8, prec_float: u32) -> Float {
    macro_rules! float {
        ($f:expr) => {
            Float::with_val(prec_float, $f)
        };
    }

    let to_d = (0..=d)
        .map(|k| -> Float {
            // (16^(d - k) % (8 * k + j)) / (8 * k + j)
            let denom = 8 * k + j as i64;
            let numer = float!(Integer::from(16)
                .pow_mod(&(d - k).into(), &denom.into())
                .unwrap());
            let denom = float!(denom);
            numer / denom
        })
        .reduce(|l, r| l + r)
        .unwrap_or(float!(0));

    // NOTE: Do not use parallelism here.
    // For some reason, it messes up the precision
    let epsilon = float!(2).pow(-(prec_float as i64));
    let after_d = ((d + 1)..=i64::MAX)
        .map(|k| {
            // 16^(d - k) / (8 * k + j)
            let numer = float!(16).pow(d - k);
            let denom = float!(8 * k + j as i64);
            numer / denom
        })
        .take_while(|f| f.clone().abs() > epsilon)
        .fold(float!(0), |acc, next| acc + next);

    to_d.fract() + after_d.fract()
}

/// Calculates whats need to calculate pi
///
/// Doesn't fully calculate pi to allow for precision to be set later
pub fn pi_hex(d: i64, precision: u8) -> Integer {
    assert!(d >= 0);
    let precision = precision as u32;
    let prec_float = if precision < 7 { 53 } else { precision * 8 };

    let mut s1 = None;
    let mut s2 = None;
    let mut s3 = None;
    let mut s4 = None;

    rayon::scope(|s| {
        s.spawn(|_| s1 = Some(series(d, 1, prec_float) * 4.0));
        s.spawn(|_| s2 = Some(series(d, 4, prec_float) * 2.0));
        s.spawn(|_| s3 = Some(series(d, 5, prec_float)));
        s.spawn(|_| s4 = Some(series(d, 6, prec_float)));
    });

    let mut pi: Float = s1.unwrap() - s2.unwrap() - s3.unwrap() - s4.unwrap();

    pi = if pi < 0.0 {
        pi.fract() + 1.0
    } else if pi > 1.0 {
        pi.fract()
    } else {
        pi
    };

    let mut pi_hex = Integer::from(0);
    for i in 0..precision {
        pi *= 16.0;
        pi_hex +=
            pi.to_integer_round(rug::float::Round::Down).unwrap().0 << (4 * (precision - i - 1));
        pi = pi.fract();
    }
    pi_hex
}

pub fn check_precision_quick(
    check_start: i64,
    check_end: i64,
    pi: impl Send + Sync + Fn(i64, u8) -> Integer,
) {
    assert!(check_start >= 0);
    assert!(check_end > check_start);
    // Maximum check
    let mut size = 255;

    let mut left = 0;
    let mut right = size;
    let mut mid;

    let checks = |mid| {
        if check_end <= 1_000 {
            let mut p = pi(check_start, mid);
            ((check_start + 1)..check_end).all(|d| {
                let new_p = pi(d, mid);
                // compare the truncated parts that should be the same
                let p_trun = p.clone().keep_bits(mid as u32 * 4 - 4);
                let new_p_trun = new_p.clone() >> 4;

                p = new_p;
                p_trun == new_p_trun
            })
        } else {
            // Use random sampling to reduce number of checks
            ((check_start + 1)..check_end)
                .choose_multiple(
                    &mut thread_rng(),
                    (check_end - check_start).ilog2() as usize,
                )
                .par_iter()
                .all(|&d| {
                    let prev = pi(d - 1, mid).keep_bits(mid as u32 * 4 - 4);
                    let curr = pi(d, mid);
                    let next = pi(d + 1, mid) >> 4;

                    prev == curr.clone() >> 4 && next == curr.keep_bits(mid as u32 * 4 - 4)
                })
        }
    };
    while left < right {
        mid = left + size / 2;
        if mid < 2 {
            println!("Inputted pi function does not work");
            return;
        }

        if checks(mid) {
            println!("Correct at {mid} precision");
            left = mid + 1;
        } else {
            println!("Incorrect at {mid} precision");
            right = mid
        }

        size = right - left;
    }
}
