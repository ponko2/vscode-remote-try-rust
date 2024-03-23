use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
        q: usize,
        abcd: [(usize, usize, usize, usize); q],
    }

    let mut z = vec![vec![0; 1501]; 1501];

    for (x, y) in xy {
        z[x][y] += 1;
    }

    for zi in z.iter_mut().skip(1).take(1500) {
        for j in 1..=1500 {
            zi[j] += zi[j - 1];
        }
    }

    for j in 1..=1500 {
        for i in 1..=1500 {
            z[i][j] += z[i - 1][j];
        }
    }

    for (a, b, c, d) in abcd {
        println!("{}", z[c][d] + z[a - 1][b - 1] - z[a - 1][d] - z[c][b - 1]);
    }
}
