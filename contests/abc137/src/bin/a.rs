use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (a, b): (i32, i32),
    }

    println!("{}", (a + b).max(a - b).max(a * b))
}
