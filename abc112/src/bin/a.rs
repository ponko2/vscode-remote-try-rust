use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    if n == 1 {
        println!("Hello World");
    } else {
        input! {
            a: usize,
            b: usize,
        }
        println!("{}", a + b);
    }
}
