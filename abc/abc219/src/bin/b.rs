use itertools::Itertools;
use proconio::marker::Chars;
use proconio::*;

#[fastout]
fn main() {
    input! {
        s: [String; 3],
        t: Chars,
    }

    let answer = t
        .into_iter()
        .map(|x| s[(x.to_digit(10).unwrap() - 1) as usize].clone())
        .join("");
    println!("{}", answer);
}
