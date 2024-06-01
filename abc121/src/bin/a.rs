use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (h1, w1): (usize, usize),
        (h2, w2): (usize, usize),
    }

    println!("{}", (h1 - h2) * (w1 - w2));
}
