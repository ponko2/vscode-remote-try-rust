use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }

    println!(
        "{}",
        if s.replace('/', "").parse::<i32>().unwrap() <= 20190430 {
            "Heisei"
        } else {
            "TBD"
        }
    );
}
