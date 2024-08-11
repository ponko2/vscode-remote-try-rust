use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }

    if s.chars()
        .enumerate()
        .any(|(i, c)| (i % 2 == 0) ^ c.is_ascii_lowercase())
    {
        println!("No");
    } else {
        println!("Yes");
    }
}
