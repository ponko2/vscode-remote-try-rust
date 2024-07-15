use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    println!("{}", (10000 - n) % 1000);
}
