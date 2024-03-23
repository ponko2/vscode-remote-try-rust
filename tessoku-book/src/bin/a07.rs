use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        d: usize,
        n: usize,
        lr: [(usize, usize); n]
    }

    let mut b = vec![0; d + 2];
    for (l, r) in lr {
        b[l] += 1;
        b[r + 1] -= 1;
    }

    for ans in b.iter().skip(1).take(d).scan(0, |state, &x| {
        *state += x;
        Some(*state)
    }) {
        println!("{ans}");
    }
}
