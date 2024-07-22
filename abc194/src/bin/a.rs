use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (a, b): (usize, usize),
    }

    println!(
        "{}",
        if a + b >= 15 && b >= 8 {
            1
        } else if a + b >= 10 && b >= 3 {
            2
        } else if a + b >= 3 {
            3
        } else {
            4
        }
    );
}
