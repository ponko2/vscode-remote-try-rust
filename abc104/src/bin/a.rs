use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        r: usize,
    }

    println!(
        "{}",
        if r < 1200 {
            "ABC"
        } else if r < 2800 {
            "ARC"
        } else {
            "AGC"
        }
    )
}
