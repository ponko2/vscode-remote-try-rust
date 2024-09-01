use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    println!("AGC{:03}", if n < 42 { n } else { n + 1 });
}
