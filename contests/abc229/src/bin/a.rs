use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: [String; 2],
    }

    println!(
        "{}",
        if s != [".#", "#."] && s != ["#.", ".#"] {
            "Yes"
        } else {
            "No"
        }
    );
}
