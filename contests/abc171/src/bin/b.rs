use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, k): (usize, usize),
        mut p: [usize; n],
    }

    p.sort();

    println!("{}", p[..k].iter().sum::<usize>());
}
