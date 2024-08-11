use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        pqr: [usize; 3],
    }

    println!("{}", pqr.iter().sorted().take(2).sum::<usize>());
}
