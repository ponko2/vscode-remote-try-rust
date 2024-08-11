use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, y): (usize, usize),
    }

    for i in 0..=n {
        for j in 0..=n {
            if let Some(k) = n.checked_sub(i + j) {
                if i * 10000 + j * 5000 + k * 1000 == y {
                    println!("{i} {j} {k}");
                    return;
                }
            }
        }
    }

    println!("-1 -1 -1");
}
