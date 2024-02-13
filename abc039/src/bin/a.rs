use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (a, b, c): (usize, usize, usize),
    }

    println!("{}", (a * b + a * c + b * c) * 2);
}
