use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u32,
    }

    println!("{}", char::from_u32(n).unwrap());
}
