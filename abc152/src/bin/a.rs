use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, m): (usize, usize),
    }

    println!("{}", if n == m { "Yes" } else { "No" });
}
