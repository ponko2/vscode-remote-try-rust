use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (n, k): (usize, usize),
    }

    let mut answer = 0;
    for x in 1..=n {
        for y in 1..=n {
            if let Some(z) = k.checked_sub(x + y) {
                if z >= 1 && z <= n {
                    answer += 1;
                }
            }
        }
    }

    println!("{}", answer);
}
