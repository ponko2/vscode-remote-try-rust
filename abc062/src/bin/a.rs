use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        (x, y): (usize, usize),
    }

    for g in [vec![1, 3, 5, 7, 8, 10, 12], vec![4, 6, 9, 11], vec![2]] {
        if g.contains(&x) && g.contains(&y) {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
