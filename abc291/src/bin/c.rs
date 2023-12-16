use proconio::{fastout, input, marker::Chars};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        _: usize,
        s: Chars,
    }

    let mut p = (0, 0);
    let mut set = HashSet::new();
    set.insert(p);

    for si in &s {
        match *si {
            'R' => p.0 += 1,
            'L' => p.0 -= 1,
            'U' => p.1 += 1,
            'D' => p.1 -= 1,
            _ => unreachable!(),
        }
        if !set.insert(p) {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
