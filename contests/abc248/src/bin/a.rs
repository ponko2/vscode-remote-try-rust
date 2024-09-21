use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }

    println!("{}", ('0'..='9').find(|&c| !s.contains(c)).unwrap());
}
