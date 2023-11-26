use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    for i in 0..(n - 2) {
        for j in (i + 1)..(n - 1) {
            for k in (j + 1)..n {
                if a[i] + a[j] + a[k] == 1000 {
                    println!("Yes");
                    return;
                }
            }
        }
    }

    println!("No");
}
