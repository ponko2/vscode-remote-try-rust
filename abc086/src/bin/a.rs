use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (a, b): (usize, usize),
    }

    println!("{}", if (a * b) % 2 != 0 { "Odd" } else { "Even" });
}
