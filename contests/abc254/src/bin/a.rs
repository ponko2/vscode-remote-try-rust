use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: String,
    }

    println!("{}", &n[1..]);
}
