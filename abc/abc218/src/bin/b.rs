use ascii::ToAsciiChar;
use itertools::Itertools;
use proconio::marker::Usize1;
use proconio::*;

#[fastout]
fn main() {
    input! {
        p: [Usize1; 26]
    }

    let answer = p
        .into_iter()
        .map(|x| (x as u16 + b'a' as u16).to_ascii_char().unwrap())
        .join("");
    println!("{}", answer);
}
