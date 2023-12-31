use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (_, m, x, t, d): (usize, usize, usize, usize, usize),
    }

    println!("{}", t - x.saturating_sub(m) * d);
}
