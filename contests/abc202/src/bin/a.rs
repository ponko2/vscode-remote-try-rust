use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        abc: [usize; 3],
    }

    println!("{}", abc.iter().map(|x| 7 - x).sum::<usize>());
}
