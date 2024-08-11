use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: i8,
    }

    println!("{}", if x >= 0 { x } else { 0 });
}
