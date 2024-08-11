use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (x, y): (usize, usize),
    }

    println!("{}", if x.min(y) + 3 > x.max(y) { "Yes" } else { "No" });
}
