use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (a, b): (f32, f32),
    }

    println!("{}", (1.0 - b / a) * 100.0);
}
