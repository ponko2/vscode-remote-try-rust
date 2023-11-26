use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, x): (isize, i32),
        a: [i32; n],
    }

    if a.iter().any(|&i| i == x) {
        println!("Yes")
    } else {
        println!("No")
    }
}
