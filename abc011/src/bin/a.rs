use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    println!("{}", if n == 12 { 1 } else { n + 1 });
}
