use proconio::{fastout, input};
use std::cmp::Ordering;

#[fastout]
fn main() {
    input! {
        (a, b): (usize, usize),
    }

    let c = |x| if x == 1 { 14 } else { x };

    println!(
        "{}",
        match c(a).cmp(&c(b)) {
            Ordering::Greater => "Alice",
            Ordering::Less => "Bob",
            Ordering::Equal => "Draw",
        }
    );
}
