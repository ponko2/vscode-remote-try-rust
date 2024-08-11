use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (a, b, c): (usize, usize, usize),
    }

    println!(
        "{}",
        if a.pow(2) + b.pow(2) < c.pow(2) {
            "Yes"
        } else {
            "No"
        }
    );
}
