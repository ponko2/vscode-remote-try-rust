use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        (_, s, _): (String, Chars, String),
    }

    println!("A{}C", s[0]);
}
