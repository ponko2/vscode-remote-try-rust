use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (v, t, s, d): (usize, usize, usize, usize),
    }

    println!(
        "{}",
        if v * t <= d && d <= v * s {
            "No"
        } else {
            "Yes"
        }
    );
}
