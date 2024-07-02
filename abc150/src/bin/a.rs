use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (k, x): (usize, usize),
    }

    println!("{}", if 500 * k >= x { "Yes" } else { "No" })
}
