use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        xy: [(i32, i32); 3],
    }

    let (x, y) = xy.iter().fold((0, 0), |a, b| (a.0 ^ b.0, a.1 ^ b.1));

    println!("{x} {y}");
}
