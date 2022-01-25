use itertools::Itertools;
use proconio::marker::Chars;
use proconio::*;

#[fastout]
fn main() {
    input! {
        x: Chars,
    }

    let n = x.len();
    let mut cum = vec![0; n + 1];
    for (i, x) in x.iter().map(|v| v.to_digit(10).unwrap()).enumerate() {
        cum[i + 1] = cum[i] + x;
    }

    for i in (1..n + 1).rev() {
        cum[i - 1] += cum[i] / 10;
        cum[i] %= 10;
    }

    let answer = cum.iter().skip_while(|&v| *v == 0).join("");
    println!("{}", answer);
}
