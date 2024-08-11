use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut a: [usize; 3],
    }

    a.sort();

    println!(
        "{}",
        if a[2] - a[1] == a[1] - a[0] {
            "Yes"
        } else {
            "No"
        }
    );
}
