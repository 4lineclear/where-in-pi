use where_in_pi::{pi_hex, pi_hex_alt};

fn main() {
    // for i in 0..10 {
    //     let pi = chudnovsky(i);
    //     println!("{pi}");
    // }
    let start = std::time::Instant::now();
    let d = 000_000_000;
    let precision = 8366;
    let pi = pi_hex(d, precision);
    let test = pi_hex_alt(d, precision);
    let elapsed = start.elapsed();
    println!("{pi:X}\n\n{test:X}\n\n{}\n", pi == test);
    println!("Elapsed: {}", elapsed.as_secs());

    // let pi = chudnovsky(10_000_000);
    // let start = std::time::Instant::now();
    // let mut b = Bernoulli::default();
    // for n in (0..=60).step_by(2) {
    //     println!("{}", bernoulli(n, &mut b));
    // }
    // println!("{}", bernoulli(0, &mut b));
    // println!("Elapsed: {}", start.elapsed().as_secs());
    // println!("{b:#?}");

    // println!("{}", bernoulli(10_000));
    // (0..100).into_iter().for_each(|i| {
    //     let pi = pi_hex(i, 2);
    //     println!("{pi:02x}")
    // });
    // let pi = pi_hex(18_651_926_753_001, 32);
    // println!("{pi:024X}\n!{pi:024x}");
    // let start = Instant::now();
    // where_in_pi::check_precision_quick(1_000_000, 1_001_000, pi_hex);
    // println!("Elapsed: {}", start.elapsed().as_secs());
    // pi_hex(1, 100);
}
