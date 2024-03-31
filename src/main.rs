fn main() {
    let splits = where_in_pi::deduce_splits(10000, 100000, 1);

    let mut splits: Vec<_> = splits.into_iter().collect();
    splits.sort_unstable_by(|s1, s2| {
        use std::cmp::Ordering::*;
        match s1.1.cmp(&s2.1) {
            Equal => s1.0.cmp(&s2.0),
            other => other,
        }
    });
    let mut prev = None;
    let len = splits.len();
    let contiguous_max: Vec<_> = splits
        .into_iter()
        .rev()
        .take_while(|s| *prev.get_or_insert(s.1) == s.1)
        .collect();

    contiguous_max
        .iter()
        .rev()
        .for_each(|s| println!("{s:?}\n"));

    println!("{len} {}", contiguous_max.len());
    // let max = splits.into_iter().max_by_key(|a| a.1);
    // println!("{max:?}")
    //

    // for splits in (3..=10).map(|n| where_in_pi::split_empty(1, n)) {
    //     println!("{splits:#?}\n{}", splits.len());
    // }
}
