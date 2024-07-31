use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut abc: [usize; 3],
    }

    abc.sort();

    println!(
        "{}",
        if abc[0] == abc[1] {
            abc[2]
        } else if abc[1] == abc[2] {
            abc[0]
        } else {
            0
        }
    );
}
