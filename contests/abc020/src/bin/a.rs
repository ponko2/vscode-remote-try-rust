use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        q: usize,
    }

    println!("{}", if q == 1 { "ABC" } else { "chokudai" });
}
