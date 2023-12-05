use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        n: Bytes,
    }

    println!(
        "{}",
        if n.iter().fold(0, |sum, c| sum + (c - b'0') as usize) % 9 == 0 {
            "Yes"
        } else {
            "No"
        }
    );
}
