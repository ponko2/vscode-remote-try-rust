use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (m, h): (usize, usize),
    }

    println!("{}", if h % m == 0 { "Yes" } else { "No" });
}
