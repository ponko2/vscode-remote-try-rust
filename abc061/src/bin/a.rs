use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (a, b, c): (i32, i32, i32),
    }

    println!("{}", if c >= a && c <= b { "Yes" } else { "No" });
}
