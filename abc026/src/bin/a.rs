use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
    }

    println!("{}", a / 2 * (a - a / 2));
}
