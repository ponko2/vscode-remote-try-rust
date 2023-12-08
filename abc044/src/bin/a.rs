use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        x: usize,
        y: usize,
    }

    let mut answer = 0;

    for i in 1..=n {
        if i <= k {
            answer += x;
        } else {
            answer += y;
        }
    }

    println!("{answer}");
}
