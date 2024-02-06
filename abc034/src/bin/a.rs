use proconio::{fastout, input};
use std::cmp::Ordering::*;

#[fastout]
fn main() {
    input! {
        (x, y): (u32, u32),
    }

    println!(
        "{}",
        match y.cmp(&x) {
            Greater => "Better",
            Less => "Worse",
            _ => unreachable!(),
        }
    )
}
