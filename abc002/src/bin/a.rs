use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (x, y): (usize, usize),
    }

    println!("{}", x.max(y));
}
