use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: Chars,
    }

    println!(
        "{}",
        n.iter()
            .map(|c| match c {
                '1' => '9',
                '9' => '1',
                _ => unreachable!(),
            })
            .collect::<String>()
    )
}
