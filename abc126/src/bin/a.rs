use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

#[fastout]
fn main() {
    input! {
        (_n, k): (usize, Usize1),
        mut s: Chars,
    }

    s[k] = s[k].to_ascii_lowercase();

    println!("{}", s.iter().collect::<String>());
}
