use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (t, x): (f64, f64),
    }

    println!("{}", t / x);
}
