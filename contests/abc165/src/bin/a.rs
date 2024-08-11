use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        k: usize,
        (a, b): (usize, usize),
    }

    println!("{}", if b - (b % k) >= a { "OK" } else { "NG" })
}
