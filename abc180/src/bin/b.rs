use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: [i64; n],
    }

    println!("{}", x.iter().map(|i| i.abs()).sum::<i64>());
    println!(
        "{}",
        x.iter().map(|i| i.abs().pow(2) as f64).sum::<f64>().sqrt()
    );
    println!("{}", x.iter().map(|i| i.abs()).max().unwrap());
}
