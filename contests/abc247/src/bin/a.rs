use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    println!("0{}{}{}", s[0], s[1], s[2]);
}
