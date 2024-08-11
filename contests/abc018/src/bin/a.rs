use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        abc: [usize; 3],
    }

    let mut s = abc.clone();
    s.sort();
    s.reverse();

    for i in abc {
        println!("{}", s.iter().position(|&v| v == i).unwrap() + 1);
    }
}
