use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    println!(
        "{}",
        s.iter().position(|&c| c.is_ascii_uppercase()).unwrap() + 1
    );
}
