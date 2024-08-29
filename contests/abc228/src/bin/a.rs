use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (s, t, x): (usize, usize, usize),
    }

    println!(
        "{}",
        if if s < t {
            s <= x && t > x
        } else {
            t > x || s <= x
        } {
            "Yes"
        } else {
            "No"
        }
    );
}
