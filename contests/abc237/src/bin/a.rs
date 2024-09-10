use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i64,
    }

    println!(
        "{}",
        if (-2_i64).pow(31) <= n && 2_i64.pow(31) > n {
            "Yes"
        } else {
            "No"
        }
    );
}
