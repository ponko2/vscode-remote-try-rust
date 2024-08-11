use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        mut s: Chars,
    }

    s.sort();

    println!(
        "{}",
        if s[0] == s[1] && s[1] != s[2] && s[2] == s[3] {
            "Yes"
        } else {
            "No"
        }
    )
}
