use std::time::Instant;

use where_in_pi::pi_hex;

fn main() {
    // (0..100).into_iter().for_each(|i| {
    //     let pi = pi_hex(i, 2);
    //     println!("{pi:02x}")
    // });
    // let pi = pi_hex(18_651_926_753_001, 32);
    // println!("{pi:024X}\n!{pi:024x}");
    let start = Instant::now();
    where_in_pi::check_precision_quick(1_000_000, 1_001_000, pi_hex);
    println!("Elapsed: {}", start.elapsed().as_secs());
}
