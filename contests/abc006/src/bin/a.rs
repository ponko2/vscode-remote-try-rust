use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    println!("{}", if n % 3 == 0 { "YES" } else { "NO" });
}
