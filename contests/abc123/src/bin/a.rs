use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (a, _b, _c, _d, e, k): (usize, usize, usize, usize, usize, usize),
    }

    println!("{}", if e - a > k { ":(" } else { "Yay!" });
}
