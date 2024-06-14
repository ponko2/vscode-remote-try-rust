use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        r: usize,
    }

    println!("{}", 3 * r.pow(2));
}
