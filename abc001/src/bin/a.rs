use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h1: isize,
        h2: isize,
    }

    println!("{}", h1 - h2);
}
