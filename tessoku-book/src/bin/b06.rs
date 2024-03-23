use proconio::{fastout, input};
use std::cmp::Ordering;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
        lr: [(usize, usize); q],
    }

    let b: Vec<_> = std::iter::once(0)
        .chain(a.iter().scan(0, |state, &a| {
            *state += if a == 0 { -1 } else { 1 };
            Some(*state)
        }))
        .collect();

    for (l, r) in lr {
        println!(
            "{}",
            match b[r].cmp(&b[l - 1]) {
                Ordering::Greater => "win",
                Ordering::Less => "lose",
                Ordering::Equal => "draw",
            }
        )
    }
}
