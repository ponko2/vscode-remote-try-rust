use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        r: usize,
    }

    println!("{}", r.pow(2));
}
