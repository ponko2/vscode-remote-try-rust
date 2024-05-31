use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (a, b, c): (usize, usize, usize),
    }

    println!("{}", (b / a).min(c));
}
