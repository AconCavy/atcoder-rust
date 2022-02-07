use proconio::marker::Chars;
use proconio::*;

#[fastout]
fn main() {
    input! {
        s: Chars
    }

    let answer = if *s.last().unwrap() == 'r' {
        "er"
    } else {
        "ist"
    };
    println!("{}", answer);
}
