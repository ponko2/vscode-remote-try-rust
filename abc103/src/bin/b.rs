use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut s: String,
        t: String,
    }

    for _ in 0..s.len() {
        if t == s {
            println!("Yes");
            return;
        }
        let c = s.pop().unwrap();
        s.insert(0, c);
    }

    println!("No");
}
