use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (mut v, a, b, c): (i32, i32, i32, i32),
    }

    println!(
        "{}",
        loop {
            v -= a;
            if v < 0 {
                break "F";
            }
            v -= b;
            if v < 0 {
                break "M";
            }
            v -= c;
            if v < 0 {
                break "T";
            }
        }
    );
}
