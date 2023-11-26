use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, q): (usize, usize),
        a: [usize; n],
        lr: [(usize, usize); q],
    }

    let s: Vec<_> = std::iter::once(0)
        .chain(a.iter().scan(0, |state, a| {
            *state += a;
            Some(*state)
        }))
        .collect();

    for (l, r) in lr {
        println!("{}", s[r] - s[l - 1]);
    }
}
