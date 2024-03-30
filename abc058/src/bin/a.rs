use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (a, b, c): (isize, isize, isize),
    }

    println!("{}", if (b - a) == (c - b) { "YES" } else { "NO" });
}
