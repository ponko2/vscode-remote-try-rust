use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: [usize; 10],
    }

    println!("{}", a[a[a[0]]]);
}
