use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: usize,
    }

    println!("{}", if n % 500 <= a { "Yes" } else { "No" });
}
