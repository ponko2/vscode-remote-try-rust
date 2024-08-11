use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (a, p): (usize, usize),
    }

    println!("{}", (a * 3 + p) / 2);
}
