use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: String,
    }

    println!(
        "{}",
        match s.chars().nth(n - 1).unwrap() {
            'o' => "Yes",
            'x' => "No",
            _ => unreachable!(),
        }
    );
}
