use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        (a, b): (Chars, Chars),
    }

    println!(
        "{}",
        a.iter()
            .map(|&c| c.to_digit(10).unwrap())
            .sum::<u32>()
            .max(b.iter().map(|&c| c.to_digit(10).unwrap()).sum::<u32>())
    );
}
