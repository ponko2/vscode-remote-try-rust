use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    println!("{}", 700 + 100 * s.iter().filter(|&&s| s == 'o').count());
}
