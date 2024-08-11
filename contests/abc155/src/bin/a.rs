use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        abc: [usize; 3],
    }

    println!(
        "{}",
        if abc.iter().unique().count() == 2 {
            "Yes"
        } else {
            "No"
        }
    );
}
