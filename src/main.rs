use where_in_pi::pi_hex;

fn main() {
    let pi = pi_hex(000_000_000, 0_010_000);
    println!("{pi:X}");
}
