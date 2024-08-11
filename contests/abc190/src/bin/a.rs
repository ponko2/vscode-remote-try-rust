use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (a, b, c): (usize, usize, usize),
    }

    println!("{}", if a + c > b { "Takahashi" } else { "Aoki" });
}
