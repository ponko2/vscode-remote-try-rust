use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        r: i32,
        g: i32,
    }

    println!("{}", g * 2 - r);
}
