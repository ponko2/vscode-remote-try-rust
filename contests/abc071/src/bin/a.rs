use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (x, a, b): (usize, usize, usize),
    }

    println!(
        "{}",
        if x.abs_diff(a) < x.abs_diff(b) {
            "A"
        } else {
            "B"
        }
    );
}
