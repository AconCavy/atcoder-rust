use proconio::marker::Chars;
use proconio::*;

#[fastout]
fn main() {
    input! {
        s: String
    }

    let mut max = s.clone();
    let mut min = s.clone();
    for i in 0..s.len() {
        let x = s[i..].to_string() + &s[..i];
        if x > max {
            max = x.clone();
        }
        if x < min {
            min = x.clone();
        }
    }

    println!("{}", min);
    println!("{}", max);
}
