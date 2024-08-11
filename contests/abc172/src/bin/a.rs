use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
    }

    println!("{}", a + a.pow(2) + a.pow(3));
}
