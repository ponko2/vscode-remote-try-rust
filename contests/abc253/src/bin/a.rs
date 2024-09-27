use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (a, b, c): (usize, usize, usize),
    }

    println!(
        "{}",
        if b == *[a, b, c].iter().sorted().nth(1).unwrap() {
            "Yes"
        } else {
            "No"
        }
    );
}
