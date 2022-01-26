use proconio::marker::*;
use proconio::*;

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let a = s[0].to_digit(10).unwrap();
    let b = s[2].to_digit(10).unwrap();
    let answer = a * b;
    println!("{}", answer);
}
