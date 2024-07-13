use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
        t: String,
    }

    println!("{}", if t.starts_with(&s) { "Yes" } else { "No" });
}
