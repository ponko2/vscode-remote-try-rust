use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        c: String,
    }

    println!(
        "{}",
        if "aiueo".contains(&c) {
            "vowel"
        } else {
            "consonant"
        }
    );
}
