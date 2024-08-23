use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (a, b): (i32, i32),
    }

    println!("{}", 32_i32.pow(a.abs_diff(b)));
}
