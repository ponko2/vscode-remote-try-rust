use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, m): (i32, i32),
    }

    println!("{}", n * (n - 1) / 2 + m * (m - 1) / 2);
}
