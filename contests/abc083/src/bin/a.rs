use proconio::{fastout, input};
use std::cmp::Ordering;

#[fastout]
fn main() {
    input! {
        (a, b, c, d): (usize, usize, usize, usize),
    }

    println!(
        "{}",
        match (a + b).cmp(&(c + d)) {
            Ordering::Greater => "Left",
            Ordering::Equal => "Balanced",
            Ordering::Less => "Right",
        }
    );
}
