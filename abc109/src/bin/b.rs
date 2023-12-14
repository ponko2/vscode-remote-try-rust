use proconio::{fastout, input};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        w: [String;n],
    }

    let mut l = w[0].chars().last().unwrap();
    let mut c = HashSet::from([&w[0]]);
    for w in &w[1..] {
        if c.contains(w) || !w.starts_with(l) {
            println!("No");
            return;
        }
        c.insert(w);
        l = w.chars().last().unwrap();
    }

    println!("Yes");
}
