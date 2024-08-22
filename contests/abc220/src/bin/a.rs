use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (a, b, c): (usize, usize, usize),
    }

    if let Some(ans) = (a..=b).find(|x| x % c == 0) {
        println!("{ans}");
    } else {
        println!("-1");
    }
}
