use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    println!("{}", if n >= 1000 { "ABD" } else { "ABC" });
}
