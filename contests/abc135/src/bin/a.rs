use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (a, b): (usize, usize),
    }

    if (a + b) % 2 == 0 {
        println!("{}", (a + b) / 2);
    } else {
        println!("IMPOSSIBLE");
    }
}
