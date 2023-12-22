use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }

    println!("2018{}", &s[4..]);
}
