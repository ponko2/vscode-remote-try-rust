use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        (_, x): (usize, usize),
        s: Chars,
    }

    let mut answer = x;

    for v in s {
        match v {
            'x' => answer = answer.saturating_sub(1),
            'o' => answer += 1,
            _ => unreachable!(),
        }
    }

    println!("{answer}");
}
