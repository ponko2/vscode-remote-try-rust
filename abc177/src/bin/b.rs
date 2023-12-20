use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut ans = t.len();
    for i in 0..=(s.len() - t.len()) {
        let mut diff = 0;
        for j in 0..t.len() {
            if s[i + j] != t[j] {
                diff += 1;
            }
        }
        ans = ans.min(diff);
    }

    println!("{ans}");
}
