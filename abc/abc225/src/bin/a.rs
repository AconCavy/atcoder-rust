use proconio::marker::Chars;
use proconio::*;

#[fastout]
fn main() {
    input! {
        mut s: Chars
    }

    s.sort();
    s.dedup();
    let count = s.len();
    let answer = match count {
        1 => 1,
        2 => 3,
        3 => 6,
        _ => 0,
    };

    println!("{}", answer);
}
