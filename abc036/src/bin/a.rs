use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (a, b): (usize, usize),
    }

    println!("{}", (b + a - 1) / a);
}
