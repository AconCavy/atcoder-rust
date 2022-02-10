use proconio::marker::Chars;
use proconio::*;

#[fastout]
fn main() {
    input! {
        mut s: Chars,
        mut t: Chars
    }

    let n = s.len();
    let mut answer = s == t;
    for i in 0..(n - 1) {
        s.swap(i, i + 1);
        answer |= s == t;
        s.swap(i, i + 1);
    }

    println!("{}", if answer { "Yes" } else { "No" });
}
