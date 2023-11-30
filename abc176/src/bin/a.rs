#![allow(unstable_name_collisions)]
use num_integer::Integer;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, x, t): (usize, usize, usize),
    }

    println!("{}", n.div_ceil(&x) * t);
}
