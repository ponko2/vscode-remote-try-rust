use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, m): (usize, usize),
    }

    println!("{}", (n - 1) * (m - 1));
}
