use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }

    println!(
        "{}",
        match s.as_str() {
            "ABC" => "ARC",
            "ARC" => "ABC",
            _ => unreachable!(),
        }
    )
}
