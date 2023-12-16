use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let s = (s[0] as i16).abs_diff(s[1] as i16);
    let t = (t[0] as i16).abs_diff(t[1] as i16);

    println!(
        "{}",
        if s.min(5 - s) == t.min(5 - t) {
            "Yes"
        } else {
            "No"
        }
    );
}
