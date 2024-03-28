use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (a, b): (char, char),
    }

    println!("{}", if a == b { "H" } else { "D" })
}
