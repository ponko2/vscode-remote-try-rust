use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: usize,
    }

    println!(
        "{}",
        match x {
            7 | 5 | 3 => "YES",
            _ => "NO",
        }
    )
}
