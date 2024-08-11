use proconio::{fastout, input};
use std::cmp::Ordering::*;

#[fastout]
fn main() {
    input! {
        (a, b, c, d): (f32, f32, f32, f32),
    }

    println!(
        "{}",
        match (b / a).partial_cmp(&(d / c)) {
            Some(Greater) => "TAKAHASHI",
            Some(Equal) => "DRAW",
            Some(Less) => "AOKI",
            None => unreachable!(),
        }
    );
}
