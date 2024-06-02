use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        b: char,
    }

    println!(
        "{}",
        match b {
            'A' => 'T',
            'C' => 'G',
            'G' => 'C',
            'T' => 'A',
            _ => unreachable!(),
        }
    )
}
