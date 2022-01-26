use itertools::Itertools;
use proconio::marker::*;
use proconio::*;

#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let s = s.into_iter().map(|v| v as i32 - 'a' as i32).collect_vec();
    let t = t.into_iter().map(|v| v as i32 - 'a' as i32).collect_vec();
    let d = (&t[0] - &s[0] + 26) % 26;
    let mut answer = true;
    for (s, t) in s.into_iter().zip(t.into_iter()) {
        answer &= (s + d) % 26 == t;
    }

    println!("{}", if answer { "Yes" } else { "No" });
}
