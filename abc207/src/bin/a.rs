use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        abc: [usize; 3],
    }

    println!("{}", abc.iter().sorted().skip(1).sum::<usize>());
}
