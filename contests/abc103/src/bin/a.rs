use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut a: [isize;3],
    }

    a.sort();

    let mut ans = 0;

    for i in 0..2 {
        ans += (a[i + 1] - a[i]).abs();
    }

    println!("{ans}");
}
