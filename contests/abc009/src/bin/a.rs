use num_integer::Integer;
use proconio::{fastout, input};

#[fastout]
#[allow(unstable_name_collisions)]
fn main() {
    input! {
        n: usize,
    }

    println!("{}", n.div_ceil(&2));
}
