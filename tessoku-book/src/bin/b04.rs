use proconio::marker::Chars;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: Chars,
    }

    let size = n.len();
    println!(
        "{}",
        (0..size).fold(0, |acc, i| acc
            + if n[i] == '1' { 1 << (size - i - 1) } else { 0 })
    );
}
