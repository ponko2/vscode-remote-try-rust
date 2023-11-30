use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (a, b): (isize, isize),
        (c, d): (isize, isize),
    }

    println!("{}", a * d - b * c);
}
