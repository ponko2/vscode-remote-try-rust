use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (m, d): (usize, usize),
    }

    println!("{}", if m % d == 0 { "YES" } else { "NO" });
}
