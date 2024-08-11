use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    println!("{}", (0..3).filter(|&i| s[i] == t[i]).count());
}
