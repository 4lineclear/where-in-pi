use where_in_pi::{pi_hex, pi_hex_alt};

fn main() {
    let d = 000_000_000;
    let precision = 8366;
    let pi = pi_hex(d, precision);
    let test = pi_hex_alt(d, precision);
    println!("{pi:X}\n\n{test:X}\n\n{}\n", pi == test);
}
