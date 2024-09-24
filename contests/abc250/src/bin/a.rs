use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (h, w): (usize, usize),
        (r, c): (usize, usize),
    }

    let mut ans = 0;

    if 1 < r {
        ans += 1;
    }
    if r < h {
        ans += 1;
    }
    if 1 < c {
        ans += 1;
    }
    if c < w {
        ans += 1;
    }

    println!("{ans}");
}
