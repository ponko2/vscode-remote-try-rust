use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }

    println!(
        "{}",
        match s.as_str() {
            "Sunny" => "Cloudy",
            "Cloudy" => "Rainy",
            "Rainy" => "Sunny",
            _ => unreachable!(),
        }
    );
}
