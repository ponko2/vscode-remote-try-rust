use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        (r, c): (Usize1, Usize1),
        a: [[usize; 2]; 2],
    }

    println!("{}", a[r][c]);
}
