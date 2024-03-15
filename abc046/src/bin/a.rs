use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        abc: [usize; 3],
    }

    println!("{}", abc.iter().unique().count());
}
