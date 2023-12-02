use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    println!(
        "{}",
        a.iter()
            .scan(0, |max, &i| {
                *max = i.max(*max);
                Some(*max - i)
            })
            .sum::<usize>()
    );
}
