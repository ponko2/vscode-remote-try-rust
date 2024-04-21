use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    println!("{}", s.iter().filter(|&x| *x == '1').count());
}
