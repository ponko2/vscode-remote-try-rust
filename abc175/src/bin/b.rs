use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut l: [usize; n],
    }

    l.sort();

    let mut answer = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            for k in (j + 1)..n {
                if l[i] != l[j] && l[j] != l[k] && l[i] + l[j] > l[k] {
                    answer += 1;
                }
            }
        }
    }

    println!("{answer}");
}
