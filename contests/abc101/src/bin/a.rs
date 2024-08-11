use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars
    }

    println!(
        "{}",
        s.iter().fold(0, |acc, c| match c {
            '+' => acc + 1,
            '-' => acc - 1,
            _ => unreachable!(),
        })
    )
}
