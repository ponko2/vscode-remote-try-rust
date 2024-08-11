use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, x): (usize, usize),
    }

    println!("{}", (n - x).min(x - 1));
}
