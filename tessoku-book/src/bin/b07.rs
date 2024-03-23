use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
        n: usize,
        lr: [(usize, usize); n],
    }

    let mut b = vec![0; t + 1];
    for (l, r) in lr {
        b[l] += 1;
        b[r] -= 1;
    }

    for ans in b.iter().take(t).scan(0, |state, &x| {
        *state += x;
        Some(*state)
    }) {
        println!("{ans}");
    }
}
