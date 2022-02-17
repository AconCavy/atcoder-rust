use proconio::marker::{Chars, Usize1};
use proconio::*;

#[fastout]
fn main() {
    input! {
        n: Usize1,
        s: Chars,
    }

    let answer = s[n] == 'o';
    println!("{}", if answer { "Yes" } else { "No" });
}
