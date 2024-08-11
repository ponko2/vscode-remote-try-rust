use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: f32,
    }

    println!("{}", ((n + 1.0) / 2.0).floor() / n);
}
