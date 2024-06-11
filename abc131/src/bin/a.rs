use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    for i in 1..s.len() {
        if s[i - 1] == s[i] {
            println!("Bad");
            return;
        }
    }

    println!("Good");
}
