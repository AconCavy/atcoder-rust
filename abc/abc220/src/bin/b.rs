use proconio::marker::Chars;
use proconio::*;

#[fastout]
fn main() {
    input! {
        k: u64,
        a: Chars,
        b: Chars
    }

    let f = |x: Vec<char>, k: u64| -> u64 {
        let mut result = 0;
        let mut kk = 1;
        for c in x.iter().map(|x| x.to_digit(10).unwrap()).rev() {
            result += kk * c as u64;
            kk *= k;
        }

        result
    };

    let answer = f(a, k) * f(b, k);
    println!("{}", answer);
}
