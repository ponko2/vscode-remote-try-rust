use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (a, b, d): (f64, f64, f64),
    }

    let d = d.to_radians();

    println!(
        "{} {}",
        a * d.cos() - b * d.sin(),
        a * d.sin() + b * d.cos()
    );
}
