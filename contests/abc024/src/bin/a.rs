use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (a, b, c, k): (usize, usize, usize, usize),
        (s, t): (usize, usize),
    }

    println!(
        "{}",
        a * s + b * t - {
            let st = s + t;
            if st >= k {
                st * c
            } else {
                0
            }
        }
    );
}
