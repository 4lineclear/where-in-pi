//! Generate pi in decimal form
//!
//! # References
//!
//! https://mathworld.wolfram.com/Digit-ExtractionAlgorithm.html
//!

// pub fn bernoulli() {}

// use rug::Rational;

use rug::{Float, Integer};

// fn binary_split_iterative(a: i128, b: i128) -> (Integer, Integer, Integer) {
//     let mut stack: Vec<(i128, i128, Integer, Integer, Integer)> = Vec::new();
//     stack.push((a, b, Integer::new(), Integer::new(), Integer::new()));
//
//     while let Some((a, b, mut pab, mut qab, mut rab)) = stack.pop() {
//         if b == a + 1 {
//             pab = Integer::from(-(6 * a - 5) * (a * 2 - 1) * (6 * a - 1));
//             qab = Integer::from(10_939_058_860_032_000i64) * (a as i128).pow(3);
//             rab = pab.clone() * (545140134 * a + 13591409);
//         } else {
//             let m = (a + b) / 2;
//             stack.push((a, m, Integer::new(), Integer::new(), Integer::new()));
//             stack.push((m, b, Integer::new(), Integer::new(), Integer::new()));
//             continue;
//         }
//         if let Some((pam, qam, ram)) = stack.last_mut() {
//             pab *= pam;
//             qab *= qam;
//             rab = qab.clone() * ram + pab.clone() * rab;
//         }
//         stack.pop(); // Pop the parent frame
//         if let Some((_, _, pp, qq, rr)) = stack.last_mut() {
//             *pp = pab;
//             *qq = qab;
//             *rr = rab;
//         }
//     }
//     (stack[0].2.clone(), stack[0].3.clone(), stack[0].4.clone())
// }

// pub fn binary_split_iterative(a: u32, b: u32) -> (Integer, Integer, Integer) {
//     let splits = split_empty(a, b);
//     let mut calcs = Vec::with_capacity(b as usize - 1);
//     // println!("Splits: {splits:?}");
//     for (a, b) in splits.into_iter().rev() {
//         if b == a + 1 {
//             let a = a as i128;
//             let pab = Integer::from(-(6 * a - 5) * (2 * a - 1) * (6 * a - 1));
//             let qab = Integer::from(10_939_058_860_032_000i64) * a.pow(3);
//             let rab = pab.clone() * (545140134 * a + 13591409);
//             // println!("Here {pab} {qab} {rab}");
//             calcs.push((pab, qab, rab));
//         } else {
//             // TODO: Ordering is wrong, must fix
//             // Incorrect ordering messes with ram & rmb
//             // Ordering seems to work when the splits are made recursivly
//             if calcs.len() >= 2 {
//                 let (pam, qam, ram) = calcs.pop().unwrap();
//                 let (pmb, qmb, rmb) = calcs.pop().unwrap();
//
//                 let pab = &pam * pmb;
//                 let qab = qam * &qmb;
//                 let rab = qmb * ram + pam * rmb;
//
//                 calcs.push((pab, qab, rab));
//                 // println!("{calcs:?}");
//             }
//         }
//     }
//     // println!("{calcs:?}");
//     if calcs.len() == 1 {
//         calcs.remove(0)
//     } else if calcs.len() == 2 {
//         let (pam, qam, ram) = calcs.pop().unwrap();
//         let (pmb, qmb, rmb) = calcs.pop().unwrap();
//
//         let pab = &pam * pmb;
//         let qab = qam * &qmb;
//         let rab = qmb * ram + pam * rmb;
//
//         (pab, qab, rab)
//     } else {
//         panic!("Calcs values:\n{calcs:#?}")
//     }
//     // println!("{calcs:?}");
// }

pub fn split_empty(a: u32, b: u32) -> (Integer, Integer, Integer) {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum Kind {
        Before(u32, u32),
        After(u32, u32),
    }
    use Kind::*;

    // let capacity = ((b - 2) * 2) as usize;
    let mut vec = vec![Before(a, b)];
    // let mut res = Vec::with_capacity(capacity);
    let mut calcs = Vec::new();
    while let Some(kind) = vec.pop() {
        let (a, b) = match kind {
            Before(a, b) | After(a, b) => (a, b),
        };

        // res.push((a, b));
        if b == a + 1 {
            let a = a as i128;
            let pab = Integer::from(-(6 * a - 5) * (2 * a - 1) * (6 * a - 1));
            let qab = Integer::from(10_939_058_860_032_000i64) * a.pow(3);
            let rab = pab.clone() * (545140134 * a + 13591409);
            // println!("iter: End({a}, {b}) -> {pab} {qab} {rab}");
            calcs.push((pab, qab, rab));
        } else if matches!(kind, After(..)) {
            let (pmb, qmb, rmb) = calcs.pop().unwrap();
            let (pam, qam, ram) = calcs.pop().unwrap();
            let pab = &pam * pmb;
            let qab = qam * &qmb;
            let rab = qmb * ram + pam * rmb;
            calcs.push((pab, qab, rab));
            // println!("iter: {kind:?}");
        } else {
            // println!("iter: {kind:?}");
            let m = (a + b) / 2;
            vec.push(After(a, b));
            vec.push(Before(m, b));
            vec.push(Before(a, m));
        }
    }
    // println!("{:E}\n{:E}\n{:E}\n", calcs[0].0, calcs[0].1, calcs[0].2);
    calcs[0].clone()
    // res.remove(0);
    // res
}

pub fn binary_split_empty(a: u32, b: u32) {
    fn inner(a: u32, b: u32, vec: &mut Vec<(u32, u32)>) -> (Integer, Integer, Integer) {
        let (pab, qab, rab);
        if b == a + 1 {
            let a = a as i128;
            pab = Integer::from(-(6 * a - 5) * (2 * a - 1) * (6 * a - 1));
            qab = Integer::from(10_939_058_860_032_000i64) * a.pow(3);
            rab = pab.clone() * (545140134 * a + 13591409);
            // println!("recu End: {a}, {b} -> {pab:E} {qab:E} {rab:E}");
        } else {
            // println!("recu Before: {a}, {b}");
            let m = (a + b) / 2;
            vec.push((a, m));
            let (pam, qam, ram) = inner(a, m, vec);
            vec.push((m, b));
            let (pmb, qmb, rmb) = inner(m, b, vec);
            pab = &pam * pmb;
            qab = qam * &qmb;
            rab = qmb * ram + pam * rmb;
            // println!("recu After: {a}, {b} -> {pab:E} {qab:E} {rab:E}");
        }
        (pab, qab, rab)
    }
    let mut vec = Vec::new();
    inner(a, b, &mut vec);
    // vec
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
        let ((pam, qam, ram), (pmb, qmb, rmb)) =
            rayon::join(|| binary_split(a, m), || binary_split(m, b));
        pab = &pam * pmb;
        qab = qam * &qmb;
        rab = qmb * ram + pam * rmb;
    }
    (pab, qab, rab)
}

// pub fn chudnovsky_iterative(n: u32) -> Float {
//     macro_rules! float {
//         ($val:expr) => {
//             Float::with_val(n * 14 * 4, $val)
//         };
//     }
//     assert!(n >= 2, "n >= 2 only");
//     let (_p1n, q1n, r1n) = binary_split_iterative(1, n);
//     (float!(426880) * float!(10005).sqrt() * q1n.clone()) / (Integer::from(13591409) * q1n + r1n)
// }

pub fn chudnovsky(n: u32) -> Float {
    macro_rules! float {
        ($val:expr) => {
            Float::with_val(n * 14 * 4, $val)
        };
    }
    assert!(n >= 2, "n >= 2 only");
    let (_p1n, q1n, r1n) = binary_split(1, n as i128);
    (float!(426880) * float!(10005).sqrt() * q1n.clone()) / (Integer::from(13591409) * q1n + r1n)
}

#[cfg(test)]
mod tests {
    use crate::chudnovsky;

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
    // #[test]
    // fn iterative() {
    //     let control = &include_str!("../pi.txt")[..=1_000_001]; // cutoff non digit chars
    //     let test = chudnovsky_iterative(72_000).to_string(); // 72_000 * 14 > 1,000,000
    //     control
    //         .chars()
    //         .zip(test.chars())
    //         .enumerate()
    //         .for_each(|(i, (pi, test))| {
    //             assert!(
    //                 pi == test,
    //                 "pi_n != test_n at index {i}, pi_n={pi}, test_n={test}"
    //             );
    //         });
    // }
}
