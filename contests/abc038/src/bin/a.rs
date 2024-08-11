use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }

    println!("{}", if s.ends_with('T') { "YES" } else { "NO" });
}
