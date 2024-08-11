use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: char,
    }

    println!(
        "{}",
        match a {
            'A'..='Z' => 'A',
            'a'..='z' => 'a',
            _ => unreachable!(),
        }
    )
}
