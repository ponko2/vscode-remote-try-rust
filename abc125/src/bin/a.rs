use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (a, b, t): (usize, usize, usize),
    }

    println!("{}", t / a * b);
}
