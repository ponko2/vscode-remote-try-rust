use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, d): (usize, f64),
        v: [(f64, f64); n],
    }

    println!("{}", v.iter().filter(|&&(x, y)| x.hypot(y) <= d).count());
}
