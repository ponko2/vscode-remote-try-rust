use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (sx, sy, gx, gy): (f64, f64, f64, f64),
    }

    println!("{}", (gy * sx + sy * gx) / (sy + gy));
}
