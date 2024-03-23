use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (h, w, n): (usize, usize, usize),
        abcd: [(usize, usize, usize, usize); n],
    }

    let mut x = vec![vec![0; w + 2]; h + 2];

    for (a, b, c, d) in abcd {
        x[a][b] += 1;
        x[a][d + 1] -= 1;
        x[c + 1][b] -= 1;
        x[c + 1][d + 1] += 1;
    }

    for xi in x.iter_mut().skip(1).take(h) {
        for j in 1..=w {
            xi[j] += xi[j - 1];
        }
    }

    for j in 1..=w {
        for i in 1..=h {
            x[i][j] += x[i - 1][j];
        }
    }

    for xi in x.iter().skip(1).take(h) {
        println!("{}", xi.iter().skip(1).take(w).join(" "));
    }
}
