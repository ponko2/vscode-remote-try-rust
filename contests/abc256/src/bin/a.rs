use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u32,
    }

    println!("{}", 2_u32.pow(n));
}
