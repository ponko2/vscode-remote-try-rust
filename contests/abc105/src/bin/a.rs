use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, k): (usize, usize),
    }

    println!("{}", if n % k == 0 { 0 } else { 1 });
}
