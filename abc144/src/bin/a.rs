use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (a, b): (i32, i32),
    }

    println!("{}", if a.max(b) <= 9 { a * b } else { -1 });
}
