use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut abc: [usize; 3],
    }

    abc.sort();

    println!(
        "{}",
        if abc[0] + abc[1] == abc[2] {
            "Yes"
        } else {
            "No"
        }
    );
}
