use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        v: [(usize, usize); n],
    }

    println!(
        "{}",
        v.iter()
            .map(|(a, b)| (a + b) * (b - a + 1) / 2)
            .sum::<usize>()
    );
}
