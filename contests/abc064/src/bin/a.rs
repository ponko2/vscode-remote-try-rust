use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (r, g, b): (usize, usize, usize),
    }

    println!(
        "{}",
        if (r * 100 + g * 10 + b) % 4 == 0 {
            "YES"
        } else {
            "NO"
        }
    );
}
