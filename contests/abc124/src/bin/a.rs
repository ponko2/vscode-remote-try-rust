use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (a, b): (usize, usize),
    }

    println!("{}", (a + b).max(a.max(b) * 2 - 1));
}
