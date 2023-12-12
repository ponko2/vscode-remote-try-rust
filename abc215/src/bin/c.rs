use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        (s, k): (Chars, usize),
    }

    println!(
        "{}",
        s.iter()
            .sorted()
            .permutations(s.len())
            .unique()
            .nth(k - 1)
            .unwrap()
            .into_iter()
            .collect::<String>()
    );
}
