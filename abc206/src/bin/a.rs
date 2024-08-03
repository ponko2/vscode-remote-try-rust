use std::cmp::Ordering;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: f32,
    }

    println!(
        "{}",
        match ((n * 1.08) as i32).cmp(&206) {
            Ordering::Less => "Yay!",
            Ordering::Equal => "so-so",
            Ordering::Greater => ":(",
        }
    );
}
