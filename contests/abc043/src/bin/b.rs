use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let mut answer = Vec::new();
    for c in s {
        match c {
            'B' => {
                answer.pop();
            }
            v => {
                answer.push(v);
            }
        }
    }

    println!("{}", answer.iter().collect::<String>());
}
