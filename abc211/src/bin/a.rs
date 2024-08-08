use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (a, b): (f64, f64),
    }

    println!("{}", (a - b) / 3. + b);
}
