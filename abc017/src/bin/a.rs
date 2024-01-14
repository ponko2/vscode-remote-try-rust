use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        es: [(f64, f64); 3],
    }

    println!("{}", es.iter().map(|(e, s)| e * s / 10.0).sum::<f64>());
}
