use proconio::{fastout, input};
use std::cmp::Ordering;

#[fastout]
fn main() {
    input! {
        (x, y): (char, char),
    }

    println!(
        "{}",
        match x.cmp(&y) {
            Ordering::Less => "<",
            Ordering::Greater => ">",
            Ordering::Equal => "=",
        }
    )
}
