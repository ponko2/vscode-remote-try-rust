use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        k: usize,
    }

    println!("{}", (k / 2) * (k - k / 2));
}
