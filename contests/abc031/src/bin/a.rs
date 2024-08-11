use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (a, d): (usize, usize),
    }

    println!("{}", ((a + 1) * d).max((d + 1) * a));
}
