use where_in_pi::int;

fn main() {
    let mut ctx = where_in_pi::VContext::default();
    println!("{ctx:#?}");
    ctx.insert(1, 3, (int(1), int(1), int(1)));
    // for splits in (3..=10).map(|n| where_in_pi::split_empty(1, n)) {
    //     println!("{splits:#?}\n{}", splits.len());
    // }
}
