use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        x: Chars,
    }

    println!("{}", x.iter().filter_map(|c| c.to_digit(10)).sum::<u32>());
}
