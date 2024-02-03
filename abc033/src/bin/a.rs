use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: Chars,
    }

    let (f, r) = n.split_first().unwrap();

    if r.iter().all(|i| i == f) {
        println!("SAME");
    } else {
        println!("DIFFERENT");
    }
}
