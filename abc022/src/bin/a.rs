use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, s, t): (usize, i32, i32),
        mut w: i32,
        mut a: [i32; n - 1],
    }

    a.insert(0, 0);

    let mut ans = 0;
    for a in &a {
        w += a;
        if w >= s && w <= t {
            ans += 1;
        }
    }

    println!("{ans}");
}
