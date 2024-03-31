fn main() {
    for splits in (3..=10).map(|n| where_in_pi::split_empty(1, n)) {
        println!("{splits:#?}\n{}", splits.len());
    }
}
