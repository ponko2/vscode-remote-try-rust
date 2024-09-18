use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (a, b, c, d): (usize, usize, usize, usize),
    }

    println!(
        "{}",
        if a * 60 + b <= c * 60 + d {
            "Takahashi"
        } else {
            "Aoki"
        }
    );
}
