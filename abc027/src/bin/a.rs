use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (l1, l2, l3): (usize, usize, usize),
    }

    println!(
        "{}",
        if l1 == l2 {
            l3
        } else if l1 == l3 {
            l2
        } else {
            l1
        }
    );
}
