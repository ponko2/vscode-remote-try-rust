use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, a, b): (usize, usize, usize),
    }

    println!("{}", (n * a).min(b));
}
