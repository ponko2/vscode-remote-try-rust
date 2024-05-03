use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (a, b, x): (usize, usize, usize),
    }

    println!("{}", if x >= a && x - a <= b { "YES" } else { "NO" });
}
