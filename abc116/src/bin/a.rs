use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (ab, bc, _ca): (usize, usize, usize),
    }

    println!("{}", ab * bc / 2);
}
