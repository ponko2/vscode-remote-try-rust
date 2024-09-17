use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _: usize,
        s: String,
    }

    println!("{}", s.chars().last().unwrap());
}
