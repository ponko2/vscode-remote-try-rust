use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    println!(
        "{}",
        if n <= 59 {
            "Bad"
        } else if (60..=89).contains(&n) {
            "Good"
        } else if (90..=99).contains(&n) {
            "Great"
        } else if n == 100 {
            "Perfect"
        } else {
            unreachable!()
        }
    )
}
