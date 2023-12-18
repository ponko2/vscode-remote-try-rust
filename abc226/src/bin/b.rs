use std::collections::HashSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut s = HashSet::new();
    for _ in 0..n {
        input! {
            l: usize,
            a: [usize;l],
        }
        s.insert(a);
    }

    println!("{}", s.len());
}
