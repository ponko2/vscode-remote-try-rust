use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut abc: [usize; 3],
    }

    abc.sort();

    println!("{}", abc[2] * 10 + abc[1] + abc[0]);
}
