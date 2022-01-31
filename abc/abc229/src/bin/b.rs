use proconio::marker::Chars;
use proconio::*;

#[fastout]
fn main() {
    input! {
        a: Chars,
        b: Chars,
    }

    let mut answer = false;
    for (a, b) in a.iter().rev().zip(b.iter().rev()) {
        answer |= a.to_digit(10).unwrap_or_default() + b.to_digit(10).unwrap_or_default() >= 10;
    }

    println!("{}", if answer { "Hard" } else { "Easy" });
}
