use proconio::marker::{Chars, Usize1};
use proconio::*;

#[fastout]
fn main() {
    input! {
        l: Usize1,
        r: usize,
        mut s: Chars,
    }

    s[l..r].reverse();
    let answer = s.into_iter().collect::<String>();
    println!("{}", answer);
}
