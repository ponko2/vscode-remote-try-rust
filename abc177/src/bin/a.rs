use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (d, t, s): (f64, f64, f64),
    }

    println!("{}", if d / s <= t { "Yes" } else { "No" });
}
