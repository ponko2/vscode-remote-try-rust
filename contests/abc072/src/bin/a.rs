use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (x, t): (usize, usize),
    }

    println!("{}", x.saturating_sub(t));
}
