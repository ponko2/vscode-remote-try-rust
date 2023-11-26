use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (a, b): (usize, usize)
    }

    if (a..=b).any(|i| 100 % i == 0) {
        println!("Yes");
    } else {
        println!("No");
    }
}
