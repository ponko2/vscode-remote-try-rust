use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (s, t): (String, String),
    }

    println!("{t}{s}");
}
