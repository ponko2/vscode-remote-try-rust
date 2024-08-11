use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        xy: String,
    }

    if let Some((x, y)) = xy.split_once('.') {
        println!(
            "{x}{}",
            match y.parse::<u32>().unwrap() {
                0..=2 => "-",
                7..=9 => "+",
                _ => "",
            }
        );
    }
}
