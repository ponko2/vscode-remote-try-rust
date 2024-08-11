use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        n: usize,
    }

    println!("{}", (n..).find(|x| x % a == 0 && x % b == 0).unwrap());
}
