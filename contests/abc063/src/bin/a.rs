use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (a, b): (usize, usize),
    }

    let ans = a + b;

    if ans < 10 {
        println!("{ans}");
    } else {
        println!("error");
    }
}
