use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        abc: [Chars; 3],
    }

    println!(
        "{}",
        if abc
            .iter()
            .tuple_windows()
            .all(|(l, r)| l.last().unwrap() == r.first().unwrap())
        {
            "YES"
        } else {
            "NO"
        }
    );
}
