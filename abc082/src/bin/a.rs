use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (a, b): (f32, f32),
    }

    println!("{}", ((a + b) / 2.0).ceil());
}
