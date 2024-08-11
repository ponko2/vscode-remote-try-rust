use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    println!(
        "{}",
        if s.iter().unique().count() == 2 {
            "Yes"
        } else {
            "No"
        }
    );
}
