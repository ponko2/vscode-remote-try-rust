use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (a, b): (usize, usize),
    }

    for c in 1..=3 {
        if (a * b * c) % 2 == 1 {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
