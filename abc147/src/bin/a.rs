use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: [usize; 3],
    }

    println!(
        "{}",
        if a.iter().sum::<usize>() >= 22 {
            "bust"
        } else {
            "win"
        }
    );
}
