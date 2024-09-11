use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u32,
    }

    println!("{}", if (2..=4).contains(&n) { "No" } else { "Yes" });
}
