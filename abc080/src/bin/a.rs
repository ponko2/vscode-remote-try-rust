use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, a, b): (usize, usize, usize),
    }

    println!("{}", (a * n).min(b));
}
