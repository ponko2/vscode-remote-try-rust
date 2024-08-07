use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, a, x, y): (usize, usize, usize, usize),
    }

    println!("{}", n.min(a) * x + n.saturating_sub(a) * y);
}
