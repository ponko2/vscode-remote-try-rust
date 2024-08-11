use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (a, b, h, m): (f64, f64, f64, f64),
    }

    println!(
        "{}",
        (a * a + b * b - 2. * a * b * (h * 30. - m * 5.5).to_radians().cos()).sqrt()
    );
}
