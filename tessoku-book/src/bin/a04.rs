use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    for i in (0..=9).rev() {
        print!("{}", (n / (1 << i)) % 2);
    }

    println!();
}
