use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (s, w): (usize, usize),
    }

    println!("{}", if s <= w { "unsafe" } else { "safe" })
}
