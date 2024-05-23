use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        d: usize,
    }

    println!(
        "{}",
        match d {
            25 => "Christmas",
            24 => "Christmas Eve",
            23 => "Christmas Eve Eve",
            22 => "Christmas Eve Eve Eve",
            _ => unreachable!(),
        }
    )
}
