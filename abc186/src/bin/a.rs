use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, w): (usize, usize),
    }

    println!("{}", n / w);
}
