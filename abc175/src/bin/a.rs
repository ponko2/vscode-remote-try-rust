use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String,
    }

    let mut count = 0;
    let mut answer = 0;
    for c in s.chars() {
        match c {
            'R' => count += 1,
            'S' => count = 0,
            _ => unreachable!(),
        }
        answer = answer.max(count);
    }

    println!("{answer}");
}
