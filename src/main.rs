use rayon::slice::ParallelSliceMut;

fn main() {
    let start = 10000;
    let end = 100000;
    let step = 1;
    let progress = true;
    let splits = where_in_pi::deduce_splits(start, end, step, progress);

    let mut splits: Vec<_> = splits.into_iter().collect();
    splits.par_sort_unstable_by(|s1, s2| {
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
}
