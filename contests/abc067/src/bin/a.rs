use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (a, b): (usize, usize),
    }

    println!(
        "{}",
        if a % 3 == 0 || b % 3 == 0 || (a + b) % 3 == 0 {
            "Possible"
        } else {
            "Impossible"
        }
    )
}
