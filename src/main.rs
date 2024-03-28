// use where_in_pi::chudnovsky;

fn main() {
    if let Some((n, control, test)) = (3..=10_000)
        .map(|n| {
            (
                n,
                where_in_pi::binary_split(1, n as i128),
                where_in_pi::split_empty(1, n),
            )
        })
        .inspect(|(i, ..)| println!("{i}"))
        .find(|(_, test, control)| test != control)
    {
        println!("{n}\n{test:#?}\n{control:#?}");
    }
    // let a = 1;
    // let b = 10;
    //
    // let control = where_in_pi::binary_split(a, b);
    // let test = where_in_pi::split_empty(a as u32, b as u32);
    // println!("{control:?}\n\n{test:?}");
    // let a = 1;
    // let b = 10;
    //
    // let control = where_in_pi::binary_split_empty(a, b);
    // let test = where_in_pi::split_empty(a, b);
    //
    // let incorrect = control
    //     .iter()
    //     .zip(test.iter())
    //     .enumerate()
    //     .find(|(_, (c_n, t_n))| c_n != t_n);
    // if let Some(i) = incorrect {
    //     println!("{i:?}\n\n{control:?}\n\n{test:?}");
    // }
    // let n = 10;
    // println!(
    //     "{}\n\n{}",
    //     where_in_pi::chudnovsky(n),
    //     where_in_pi::chudnovsky_iterative(n),
    // );
    // if let Some((n, control, test)) = (3..=10_000)
    //     .map(|n| {
    //         (
    //             n,
    //             where_in_pi::chudnovsky(n),
    //             where_in_pi::chudnovsky_iterative(n),
    //         )
    //     })
    //     .inspect(|(i, ..)| println!("{i}"))
    //     .find(|(_, test, control)| test != control)
    // {
    //     println!("{n}\n{test:#?}\n{control:#?}");
    // }
    // let control = where_in_pi::chudnovsky(13);
    // let test = where_in_pi::chudnovsky_iterative(13);
    // println!("{control:#?}");
    // println!("{test:#?}");
    // let pi = where_in_pi::chudnovsky(1_000_000);
    // println!("{pi}\n{}", pi.prec());
}
// steps = (n - 2) * 2 + 1
// 2 -> 1
// 3 -> 2
// 4 -> 3
// 5 -> 2 2
// 6 -> 3 2
// 7 -> 3 3
// 8 -> 2 2 3
// 9 -> 2 2 2 2
//10 -> 3 2 2 2
//11 -> 3 2 3 2
//12 -> 3 3 3 2
//13 -> 3 3 3 3
//14 -> 2 2 3 3 3
//15 -> 2 2 3 2 2 3
//16 -> 2 2 2 2 2 2 3
//17 -> 2 2 2 2 2 2 2 2
//18 -> 3 2 2 2 2 2 2 2
//19 -> 3 2 2 2 3 2 2 2
//20 -> 3 2 3 2 3 2 2 2
//21 -> 3 2 3 2 3 2 3 2
//22 -> 3 3 3 2 3 2 3 2
//23 -> 3 3 3 3 3 2 3 2
//24 -> 3 3 3 3 3 3 3 2
//25 -> 3 3 3 3 3 3 3 3
//26 -> 2 2 3 3 3 3 3 3 3
//26 -> 2 2 3 3 3 2 2 3 3 3
//26 -> 2 2 3 2 2 3 2 2 3 3 3
//26 -> 2 2 3 2 2 3 2 2 3 2 2 3
//26 -> 2 2 2 2 2 2 3 2 2 3 2 2 3
//26 -> 2 2 2 2 2 2 2 2 2 2 3 2 2 3
//26 -> 2 2 2 2 2 2 2 2 2 2 3 2 2 3
