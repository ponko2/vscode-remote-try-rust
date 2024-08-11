use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        c: Chars,
    }

    println!(
        "{}",
        if c[0] == c[1] && c[1] == c[2] {
            "Won"
        } else {
            "Lost"
        }
    );
}
