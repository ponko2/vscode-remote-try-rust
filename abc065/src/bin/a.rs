use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (x, a, b): (i64, i64, i64),
    }

    let y = b - a;

    println!(
        "{}",
        if y <= 0 {
            "delicious"
        } else if y > x {
            "dangerous"
        } else {
            "safe"
        }
    );
}
