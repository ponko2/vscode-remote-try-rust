use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (a, op, b): (isize, char, isize),
    }

    println!(
        "{}",
        match op {
            '+' => a + b,
            '-' => a - b,
            _ => unreachable!(),
        }
    )
}
