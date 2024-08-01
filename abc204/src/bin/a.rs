use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (x, y): (usize, usize),
    }

    println!("{}", if x == y { x } else { 3 - x - y });
}
