use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, r): (usize, usize),
    }

    println!("{}", r + if n >= 10 { 0 } else { 100 * (10 - n) });
}
