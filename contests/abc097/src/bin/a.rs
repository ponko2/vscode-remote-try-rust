use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (a, b, c, d): (usize, usize, usize, usize),
    }

    println!(
        "{}",
        if a.abs_diff(c) <= d || a.abs_diff(b).max(b.abs_diff(c)) <= d {
            "Yes"
        } else {
            "No"
        }
    );
}
