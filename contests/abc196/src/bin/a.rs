use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (a, b): (i32, i32),
        (c, d): (i32, i32),
    }

    println!("{}", a.max(b) - c.min(d));
}
