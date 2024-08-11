use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (w, h): (usize, usize),
    }

    println!("{}", if h * 4 == w * 3 { "4:3" } else { "16:9" })
}
