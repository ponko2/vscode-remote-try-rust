use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        abc: Chars,
    }

    println!(
        "{}",
        vec![
            format!("{}{}{}", abc[0], abc[1], abc[2]),
            format!("{}{}{}", abc[1], abc[2], abc[0]),
            format!("{}{}{}", abc[2], abc[0], abc[1])
        ]
        .iter()
        .map(|x| x.parse::<usize>().unwrap())
        .sum::<usize>()
    );
}
