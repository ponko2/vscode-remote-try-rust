use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (a, b): (usize, usize),
    }

    println!(
        "{}",
        if [1, 9].contains(&a.abs_diff(b)) {
            "Yes"
        } else {
            "No"
        }
    );
}
