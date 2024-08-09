use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (a, b): (usize, usize),
    }

    println!(
        "{}",
        match (a, b) {
            (_, 0) => "Gold",
            (0, _) => "Silver",
            _ => "Alloy",
        }
    );
}
