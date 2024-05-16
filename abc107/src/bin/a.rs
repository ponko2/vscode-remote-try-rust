use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, i): (usize, usize),
    }

    println!("{}", n - i + 1);
}
