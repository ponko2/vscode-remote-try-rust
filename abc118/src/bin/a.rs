use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (a, b): (usize, usize),
    }

    println!("{}", if b % a == 0 { a + b } else { b - a });
}
