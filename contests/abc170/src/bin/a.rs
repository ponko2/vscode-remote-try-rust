use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (x1, x2, x3, x4): (usize, usize, usize, usize),
    }

    println!(
        "{}",
        if x1 == 0 {
            1
        } else if x2 == 0 {
            2
        } else if x3 == 0 {
            3
        } else if x4 == 0 {
            4
        } else {
            5
        }
    )
}
