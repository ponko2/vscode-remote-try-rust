use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    println!(
        "{}",
        match n % 10 {
            2 | 4 | 5 | 7 | 9 => "hon",
            0 | 1 | 6 | 8 => "pon",
            3 => "bon",
            _ => unreachable!(),
        }
    )
}
