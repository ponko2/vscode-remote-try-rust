use proconio::{fastout, input};

fn f(x: usize) -> usize {
    x.pow(2) + 2 * x + 3
}

#[fastout]
fn main() {
    input! {
        t: usize,
    }

    println!("{}", f(f(f(t) + t) + f(f(t))));
}
