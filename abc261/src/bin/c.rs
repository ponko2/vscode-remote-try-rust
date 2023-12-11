use std::collections::HashMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String;n],
    }

    let mut h = HashMap::new();

    for si in &s {
        let c = h.entry(si).or_insert(0);
        if *c == 0 {
            println!("{si}");
        } else {
            println!("{si}({})", *c);
        }
        *c += 1;
    }
}
