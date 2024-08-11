use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (x, y): (usize, usize),
    }

    if y >= x * 2 && y <= x * 4 && y % 2 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
