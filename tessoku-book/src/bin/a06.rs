use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, q): (usize, usize),
        a: [usize; n],
        lr: [(usize, usize); q],
    }

    let mut s = Vec::with_capacity(n + 1);
    s.push(0);
    for i in 0..n {
        s.push(s[i] + a[i]);
    }

    for i in 0..q {
        let (l, r) = lr[i];
        println!("{}", s[r] - s[l - 1]);
    }
}
