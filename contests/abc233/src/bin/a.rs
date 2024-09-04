use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (x, y): (usize, usize),
    }

    println!("{}", (9 + y).saturating_sub(x) / 10);
}
