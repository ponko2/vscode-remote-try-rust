use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (h, w): (usize, usize),
        a: [String;h],
    }

    println!("{}", "#".repeat(w + 2));
    for a in &a {
        println!("#{}#", a);
    }
    println!("{}", "#".repeat(w + 2));
}
