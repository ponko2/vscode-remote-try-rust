use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (a, b, c, x): (usize, usize, usize, usize),
    }

    println!(
        "{:.12}",
        if a >= x {
            1.0
        } else if a < x && b >= x {
            c as f64 / (b - a) as f64
        } else {
            0.0
        }
    );
}
