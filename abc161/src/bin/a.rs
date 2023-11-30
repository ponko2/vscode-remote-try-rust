use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (x, y, z): (usize, usize, usize),
    }

    println!("{z} {x} {y}");
}
