use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: [String; 3],
    }

    println!(
        "{}",
        s.iter()
            .map(|s| s.chars().next().unwrap().to_ascii_uppercase())
            .collect::<String>()
    );
}
