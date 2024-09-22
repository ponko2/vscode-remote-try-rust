use proconio::{fastout, input};
use std::cmp::Ordering;

#[fastout]
fn main() {
    input! {
        (a, b, c, d, e, f, x): (usize, usize, usize, usize, usize, usize, usize),
    }

    println!(
        "{}",
        match ((x / (a + c) * a + (x % (a + c)).min(a)) * b)
            .cmp(&((x / (d + f) * d + (x % (d + f)).min(d)) * e))
        {
            Ordering::Greater => "Takahashi",
            Ordering::Less => "Aoki",
            Ordering::Equal => "Draw",
        }
    );
}
