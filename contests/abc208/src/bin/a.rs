use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (a, b): (usize, usize),
    }

    println!("{}", if a <= b && a * 6 >= b { "Yes" } else { "No" });
}
