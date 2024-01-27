use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    println!("{}", (n + 1) * 10000 / 2);
}
