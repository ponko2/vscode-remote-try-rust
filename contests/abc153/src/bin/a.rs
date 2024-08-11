use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (h, a): (usize, usize),
    }

    println!("{}", (h + a - 1) / a);
}
