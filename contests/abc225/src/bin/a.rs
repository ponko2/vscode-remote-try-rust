use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }

    println!("{}", s.chars().permutations(3).unique().count());
}
