use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
    }

    for ans in 1..=3 {
        if a != ans && b != ans {
            println!("{ans}");
            return;
        }
    }
}
