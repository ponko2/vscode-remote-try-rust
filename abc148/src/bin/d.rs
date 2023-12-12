use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize;n],
    }

    let mut ans = 0;
    let mut i = 1;

    for a in &a {
        if *a != i {
            ans += 1;
        } else {
            i += 1;
        }
    }

    if ans == a.len() {
        println!("-1");
    } else {
        println!("{ans}");
    }
}
