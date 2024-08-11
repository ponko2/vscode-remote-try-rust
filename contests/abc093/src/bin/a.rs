use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    println!(
        "{}",
        if s.iter().sorted().collect::<String>() == "abc" {
            "Yes"
        } else {
            "No"
        }
    );
}
