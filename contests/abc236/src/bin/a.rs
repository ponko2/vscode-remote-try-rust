use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

#[fastout]
fn main() {
    input! {
        mut s: Chars,
        (a, b): (Usize1, Usize1),
    }

    s.swap(a, b);

    println!("{}", s.iter().collect::<String>());
}
