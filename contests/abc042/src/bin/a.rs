use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut abc: [usize; 3],
    }

    abc.sort();

    println!("{}", if abc == vec![5, 5, 7] { "YES" } else { "NO" });
}
