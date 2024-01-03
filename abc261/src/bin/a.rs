use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (l1, r1, l2, r2): (usize, usize, usize, usize),
    }

    println!("{}", r1.min(r2).saturating_sub(l1.max(l2)));
}
