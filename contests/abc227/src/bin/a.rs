use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, k, a): (usize, usize, usize),
    }

    println!("{}", (a + k - 2) % n + 1)
}
