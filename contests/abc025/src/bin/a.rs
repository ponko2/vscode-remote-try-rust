use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
        n: usize,
    }

    let mut c = 0;
    for i in &s {
        for j in &s {
            c += 1;
            if c == n {
                println!("{i}{j}");
                return;
            }
        }
    }
}
