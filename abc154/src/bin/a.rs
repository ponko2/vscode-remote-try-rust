use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (s, t): (String, String),
        (mut a, mut b): (usize, usize),
        u: String,
    }

    if u == s {
        a -= 1;
    }

    if u == t {
        b -= 1;
    }

    println!("{a} {b}")
}
