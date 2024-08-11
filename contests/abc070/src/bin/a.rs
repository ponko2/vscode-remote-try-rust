use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: Chars,
    }

    println!(
        "{}",
        if n.first().unwrap() == n.last().unwrap() {
            "Yes"
        } else {
            "No"
        }
    );
}
