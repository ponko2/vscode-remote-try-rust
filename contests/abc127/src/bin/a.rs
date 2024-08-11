use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (a, b): (usize, usize),
    }

    println!(
        "{}",
        if a >= 13 {
            b
        } else if (6..=12).contains(&a) {
            b / 2
        } else {
            0
        }
    )
}
