use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

#[fastout]
fn main() {
    input! {
        s: Chars,
        i: Usize1,
    }

    println!("{}", s[i]);
}
